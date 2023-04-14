use super::reply_comment_repository_interface::ReplyCommentRepositoryInterface;
use super::{
    CommentReplyCreateEntity, CommentReplyDeleteEntity, CommentReplyEntity, CREATE_COMMENT_REPLY,
    CREATE_INACTIVE_COMMENT_REPLY, DELETE_COMMENT_REPLIES_BY_COMMENT_ID, DELETE_COMMENT_REPLY,
    DELETE_INACTIVE_REPLY_COMMENT, GET_COMMENTS_REPLY_BY_COMMENT_ID, GET_COMMENT_REPLY_BY_REPLY_ID,
    GET_INACTIVE_REPLY_COMMENT_BY_ID,
};
use crate::utils::cloud_front_client::CloudFrontSigner;
use anyhow::Result;
use async_trait::async_trait;
use blumer_lib_errors::AppError;
use chrono::Duration;
use scylla::prepared_statement::PreparedStatement;
use scylla::{QueryResult, Session};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct ReplyCommentRepository {
    session: Arc<Session>,
}

impl ReplyCommentRepository {
    pub fn new(session: Arc<Session>) -> Self {
        ReplyCommentRepository { session }
    }
}

#[async_trait]
impl ReplyCommentRepositoryInterface for &ReplyCommentRepository {
    async fn create_comment_reply(
        &self,
        comment: CommentReplyCreateEntity,
    ) -> Result<(), AppError> {
        let query: &str = if comment.active {
            CREATE_COMMENT_REPLY
        } else {
            CREATE_INACTIVE_COMMENT_REPLY
        };

        let query_statement: PreparedStatement = self.session.prepare(query).await?;
        self.session
            .execute(
                &query_statement,
                (
                    comment.post_id,
                    comment.reply_id,
                    comment.comment_id,
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

    async fn get_comment_reply_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
        reply_id: &Uuid,
    ) -> Result<Option<CommentReplyEntity>, AppError> {
        let first_int_val: Option<(
            Uuid,
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
            .query(
                GET_COMMENT_REPLY_BY_REPLY_ID,
                (post_id, comment_id, reply_id),
            )
            .await?
            .maybe_first_row_typed::<(
                Uuid,
                Uuid,
                Uuid,
                Uuid,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Duration,
            )>()?;

        if let Some((
            post_id,
            reply_id,
            comment_id,
            user_id,
            description,
            image,
            audio,
            gif,
            created_at,
        )) = first_int_val
        {
            let image: Option<String> = CloudFrontSigner::sing(image);
            let audio: Option<String> = CloudFrontSigner::sing(audio);
            let comment_reply: CommentReplyEntity = CommentReplyEntity {
                post_id,
                reply_id,
                comment_id,
                user_id,
                description,
                image,
                audio,
                gif,
                created_at,
            };
            return Ok(Some(comment_reply));
        } else {
            return Ok(None);
        }
    }

    async fn get_all_comment_replies_from_comment_id(
        &self,
        post_id: Uuid,
        comment_id: Uuid,
        incoming_page_state: Option<Vec<u8>>,
    ) -> Result<(Vec<CommentReplyEntity>, Option<Vec<u8>>), AppError> {
        let mut query_statement: PreparedStatement = self
            .session
            .prepare(GET_COMMENTS_REPLY_BY_COMMENT_ID)
            .await?;
        query_statement.set_page_size(10);

        // Convert incoming to Option<scylla::Bytes>
        let paging_state: Option<scylla::Bytes> = match incoming_page_state {
            Some(paging_state) => Some(scylla::Bytes::from(paging_state)),
            None => None,
        };

        let query_result: QueryResult = self
            .session
            .execute_paged(&query_statement, (post_id, comment_id), paging_state)
            .await?;

        let new_page_state: Option<Vec<u8>> = match &query_result.paging_state {
            Some(page_state) => Some(page_state.to_vec()),
            None => None,
        };

        let mut rows_stream = query_result.rows_typed_or_empty::<CommentReplyEntity>();

        let mut comment_replies: Vec<CommentReplyEntity> = Vec::new();
        while let Some(next_row_res) = rows_stream.next() {
            let mut next_row: CommentReplyEntity =
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

            next_row.image = CloudFrontSigner::sing(next_row.image);
            next_row.audio = CloudFrontSigner::sing(next_row.audio);

            comment_replies.push(next_row);
        }
        Ok((comment_replies, new_page_state))
    }

    async fn delete_comment_reply(
        &self,
        reply_comment: CommentReplyDeleteEntity,
    ) -> Result<(), AppError> {
        let query_statement: PreparedStatement = self.session.prepare(DELETE_COMMENT_REPLY).await?;

        let _ = self
            .session
            .execute(
                &query_statement,
                (
                    reply_comment.post_id,
                    reply_comment.comment_id,
                    reply_comment.reply_id,
                ),
            )
            .await
            .expect("Error when trying to delete comment reply");
        Ok(())
    }

    async fn delete_comment_replies_by_comment_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<(), AppError> {
        let query_statement: PreparedStatement = self
            .session
            .prepare(DELETE_COMMENT_REPLIES_BY_COMMENT_ID)
            .await?;

        let _ = self
            .session
            .execute(&query_statement, (post_id, comment_id))
            .await
            .expect("Error when trying to delete comment replies by comment id");
        Ok(())
    }

    async fn get_inactive_reply_comment_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
        reply_id: &Uuid,
    ) -> Result<Option<CommentReplyEntity>, AppError> {
        let first_int_val: Option<(
            Uuid,
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
            .query(
                GET_INACTIVE_REPLY_COMMENT_BY_ID,
                (post_id, comment_id, reply_id),
            )
            .await?
            .maybe_first_row_typed::<(
                Uuid,
                Uuid,
                Uuid,
                Uuid,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Duration,
            )>()?;

        if let Some((
            post_id,
            reply_id,
            comment_id,
            user_id,
            description,
            image,
            audio,
            gif,
            created_at,
        )) = first_int_val
        {
            let comment_reply: CommentReplyEntity = CommentReplyEntity {
                post_id,
                reply_id,
                comment_id,
                user_id,
                description,
                image,
                audio,
                gif,
                created_at,
            };
            return Ok(Some(comment_reply));
        } else {
            return Ok(None);
        }
    }

    async fn delete_inactive_reply_comment(
        &self,
        reply_comment: CommentReplyDeleteEntity,
    ) -> Result<(), AppError> {
        let query_statement: PreparedStatement =
            self.session.prepare(DELETE_INACTIVE_REPLY_COMMENT).await?;

        let _ = self
            .session
            .execute(
                &query_statement,
                (
                    reply_comment.post_id,
                    reply_comment.comment_id,
                    reply_comment.reply_id,
                ),
            )
            .await
            .expect("Error when trying to delete comment reply");
        Ok(())
    }
}
