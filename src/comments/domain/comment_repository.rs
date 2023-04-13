use super::{
    CommentCreateEntity, CommentEntity, CommentRepositoryInterface, CREATE_COMMENT,
    CREATE_INACTIVE_COMMENT, DELETE_COMMENT, DELETE_INACTIVE_COMMENT, GET_COMMENTS_BY_POST_ID,
    GET_COMMENT_BY_COMMENT_ID, GET_INACTIVE_COMMENT_BY_ID,
};
use crate::utils::general::s3_get_signed_url;
use anyhow::Result;
use async_trait::async_trait;
use blumer_lib_errors::AppError;
use chrono::Duration;
use scylla::prepared_statement::PreparedStatement;
use scylla::{QueryResult, Session};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CommentRepository {
    session: Arc<Session>,
    aws_cloudfront_url: String,
    aws_key_pair_id: String,
    aws_cloudfront_private_key: String,
}

impl CommentRepository {
    pub fn new(session: Arc<Session>) -> Self {
        let aws_cloudfront_private_key = std::env::var("AWS_CLOUDFRONT_PRIVATE_KEY")
            .expect("Can't get cloudfront private key")
            .replace("\\r", "\r")
            .replace("\\n", "\n");
        let aws_cloudfront_url =
            std::env::var("AWS_CLOUDFRONT_URL").expect("Can't get cloudfront URL");
        let aws_key_pair_id =
            std::env::var("AWS_CLOUDFRONT_KEY_PAIR_ID").expect("Can't get cloudfront key pair id");

        CommentRepository {
            session,
            aws_cloudfront_url,
            aws_key_pair_id,
            aws_cloudfront_private_key,
        }
    }
}

#[async_trait]
impl CommentRepositoryInterface for &CommentRepository {
    async fn create_comment(&self, comment: CommentCreateEntity) -> Result<(), AppError> {
        let query: &str = if comment.active {
            CREATE_COMMENT
        } else {
            CREATE_INACTIVE_COMMENT
        };

        let query_statement: PreparedStatement = self.session.prepare(query).await?;
        self.session
            .execute(
                &query_statement,
                (
                    comment.comment_id,
                    comment.post_id,
                    comment.user_id,
                    comment.description,
                    comment.image,
                    comment.audio,
                    comment.gif,
                ),
            )
            .await?;

        Ok(())
    }

    async fn get_comments_from_post_id(
        &self,
        post_id: Uuid,
        incoming_page_state: Option<Vec<u8>>,
    ) -> Result<(Vec<CommentEntity>, Option<Vec<u8>>), AppError> {
        let mut query_statement: PreparedStatement =
            self.session.prepare(GET_COMMENTS_BY_POST_ID).await?;
        query_statement.set_page_size(10);

        // Convert incoming page state to Option<scylla::Bytes>
        let paging_state: Option<scylla::Bytes> = match incoming_page_state {
            Some(paging_state) => Some(scylla::Bytes::from(paging_state)),
            None => None,
        };

        let query_result: QueryResult = self
            .session
            .execute_paged(&query_statement, (post_id,), paging_state)
            .await?;

        let page_state: Option<Vec<u8>> = match &query_result.paging_state {
            Some(page_state) => Some(page_state.to_vec()),
            None => None,
        };

        let mut rows_stream = query_result.rows_typed_or_empty::<CommentEntity>();

        let mut comments: Vec<CommentEntity> = Vec::new();
        while let Some(next_row_res) = rows_stream.next() {
            let mut next_row: CommentEntity =
                next_row_res.expect("Error when getting next row in comments by post id");
            // Converting fields that comes: Some("") to None
            next_row.image = if next_row.image == Some("".to_string()) {
                None
            } else {
                next_row.image
            };
            next_row.audio = if next_row.audio == Some("".to_string()) {
                None
            } else {
                next_row.audio
            };
            next_row.gif = if next_row.gif == Some("".to_string()) {
                None
            } else {
                next_row.gif
            };

            next_row.image = match next_row.image {
                Some(image) => Some(
                    s3_get_signed_url(
                        &self.aws_cloudfront_url,
                        &self.aws_key_pair_id,
                        &self.aws_cloudfront_private_key,
                        Some(image),
                    )
                    .await
                    .expect("Error when converting image key to signed url"),
                ),
                None => None,
            };
            next_row.audio = match next_row.audio {
                Some(audio) => Some(
                    s3_get_signed_url(
                        &self.aws_cloudfront_url,
                        &self.aws_key_pair_id,
                        &self.aws_cloudfront_private_key,
                        Some(audio),
                    )
                    .await
                    .expect("Error when converting audio key to signed url"),
                ),
                None => None,
            };

            comments.push(next_row);
        }
        Ok((comments, page_state))
    }

    async fn get_comment_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<Option<CommentEntity>, AppError> {
        let first_int_val: Option<(
            Uuid,
            Uuid,
            Uuid,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Duration,
        )> = self
            .session
            .query(GET_COMMENT_BY_COMMENT_ID, (post_id, comment_id))
            .await?
            .maybe_first_row_typed::<(
                Uuid,
                Uuid,
                Uuid,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Duration,
            )>()?;

        if let Some((comment_id, post_id, user_id, description, image, audio, gif, created_at)) =
            first_int_val
        {
            let image: Option<String> = match image {
                Some(image) => Some(
                    s3_get_signed_url(
                        &self.aws_cloudfront_url,
                        &self.aws_key_pair_id,
                        &self.aws_cloudfront_private_key,
                        Some(image),
                    )
                    .await
                    .expect("Error when converting image key to signed url"),
                ),
                None => None,
            };
            let audio: Option<String> = match audio {
                Some(audio) => Some(
                    s3_get_signed_url(
                        &self.aws_cloudfront_url,
                        &self.aws_key_pair_id,
                        &self.aws_cloudfront_private_key,
                        Some(audio),
                    )
                    .await
                    .expect("Error when converting audio key to signed url"),
                ),
                None => None,
            };

            // Convert created_at to String
            let comment: CommentEntity = CommentEntity {
                post_id,
                user_id,
                comment_id,
                description,
                image,
                audio,
                gif,
                created_at,
            };
            return Ok(Some(comment));
        } else {
            return Ok(None);
        }
    }

    async fn delete_comment(&self, post_id: &Uuid, comment_id: &Uuid) -> Result<(), AppError> {
        let query_statement: PreparedStatement = self.session.prepare(DELETE_COMMENT).await?;

        let _ = self
            .session
            .execute(&query_statement, (post_id, comment_id))
            .await
            .expect("Error when trying to delete comment");

        Ok(())
    }

    async fn get_inactive_comment_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<Option<CommentEntity>, AppError> {
        let first_int_val: Option<(
            Uuid,
            Uuid,
            Uuid,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Duration,
        )> = self
            .session
            .query(GET_INACTIVE_COMMENT_BY_ID, (post_id, comment_id))
            .await?
            .maybe_first_row_typed::<(
                Uuid,
                Uuid,
                Uuid,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Duration,
            )>()?;

        if let Some((comment_id, post_id, user_id, description, image, audio, gif, created_at)) =
            first_int_val
        {
            // Convert created_at to String
            let comment: CommentEntity = CommentEntity {
                post_id,
                user_id,
                comment_id,
                description,
                image,
                audio,
                gif,
                created_at,
            };
            return Ok(Some(comment));
        } else {
            return Ok(None);
        }
    }

    async fn delete_inactive_comment(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<(), AppError> {
        let query_statement: PreparedStatement =
            self.session.prepare(DELETE_INACTIVE_COMMENT).await?;

        let _ = self
            .session
            .execute(&query_statement, (post_id, comment_id))
            .await
            .expect("Error when trying to delete comment");

        Ok(())
    }
}
