use crate::adapters::kafka::KafkaProducer;
use crate::comments::domain::CommentRepositoryInterface;
use crate::comments::infrastructure::capn_proto::models::{
    CommentCountsDeleteEntity as CommentCountsDeleteEntityProto,
    CommentDeleteEntity as CommentDeleteEntityProto,
};
use crate::comments::infrastructure::capn_proto::{CommentCountsDeleteMapper, CommentDeleteMapper};
use crate::comments::infrastructure::graphql::objects::CommentsDeleteInput;
use crate::comments::infrastructure::kafka::{
    KAFKA_TOPIC_COMMENT_DELETE, KAFKA_TOPIC_DELETE_COMMENT_COUNTS,
};
use crate::reply_comments::domain::ReplyCommentRepositoryInterface;
use crate::utils::general::can_view_post;
use anyhow::Result;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use std::str::FromStr;
use uuid::Uuid;

pub struct DeleteCommentUseCase;

impl DeleteCommentUseCase {
    pub async fn execute(
        post_authorization: PostAuthorization,
        comment_repo: &impl CommentRepositoryInterface,
        reply_comment_repo: &impl ReplyCommentRepositoryInterface,
        comment: CommentsDeleteInput,
        user_id: Uuid,
        kafka_producer: &KafkaProducer,
    ) -> Result<bool, AppError> {
        let post_id = Uuid::from_str(&comment.post_id)
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;
        let comment_id = Uuid::from_str(&comment.comment_id)
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let original_post_owner_id = can_view_post(post_authorization, post_id, user_id)
            .await?
            .data
            .unwrap()
            .owner_id;

        let comment_db = comment_repo
            .get_comment_by_id(&post_id, &comment_id)
            .await?
            .ok_or(AppError::DatasourceError(
                "Error getting comment".to_owned(),
            ))?;

        if user_id != original_post_owner_id && comment_db.user_id != user_id {
            return Err(AppError::Forbidden);
        }

        comment_repo.delete_comment(&post_id, &comment_id).await?;

        reply_comment_repo
            .delete_comment_replies_by_comment_id(&post_id, &comment_id)
            .await?;

        let obj = CommentDeleteEntityProto {
            user_id,
            post_id,
            post_owner_id: original_post_owner_id,
            comment_id,
        };
        let message = CommentDeleteMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_COMMENT_DELETE, &message)
            .await;

        let obj = CommentCountsDeleteEntityProto {
            post_id,
            comment_id,
        };
        let message = CommentCountsDeleteMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_DELETE_COMMENT_COUNTS, &message)
            .await;

        return Ok(true);
    }
}
