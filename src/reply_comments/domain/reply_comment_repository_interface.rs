use super::{CommentReplyCreateEntity, CommentReplyDeleteEntity, CommentReplyEntity};
use anyhow::Result;
use async_trait::async_trait;
use blumer_lib_errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait ReplyCommentRepositoryInterface {
    async fn create_comment_reply(&self, comment: CommentReplyCreateEntity)
        -> Result<(), AppError>;

    async fn get_comment_reply_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
        reply_id: &Uuid,
    ) -> Result<Option<CommentReplyEntity>, AppError>;

    async fn get_all_comment_replies_from_comment_id(
        &self,
        post_id: Uuid,
        comment_id: Uuid,
        incoming_page_state: Option<Vec<u8>>,
    ) -> Result<(Vec<CommentReplyEntity>, Option<Vec<u8>>), AppError>;

    async fn delete_comment_reply(
        &self,
        reply_comment: CommentReplyDeleteEntity,
    ) -> Result<(), AppError>;

    async fn delete_comment_replies_by_comment_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<(), AppError>;

    async fn get_inactive_reply_comment_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
        reply_id: &Uuid,
    ) -> Result<Option<CommentReplyEntity>, AppError>;

    async fn delete_inactive_reply_comment(
        &self,
        reply_comment: CommentReplyDeleteEntity,
    ) -> Result<(), AppError>;
}
