use super::{CommentCreateEntity, CommentEntity};
use anyhow::Result;
use async_trait::async_trait;
use blumer_lib_errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait CommentRepositoryInterface {
    async fn create_comment(&self, comment: CommentCreateEntity) -> Result<(), AppError>;

    async fn get_comments_from_post_id(
        &self,
        post_id: Uuid,
        incoming_page_state: Option<Vec<u8>>,
    ) -> Result<(Vec<CommentEntity>, Option<Vec<u8>>), AppError>;

    async fn get_comment_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<Option<CommentEntity>, AppError>;

    async fn delete_comment(&self, post_id: &Uuid, comment_id: &Uuid) -> Result<(), AppError>;

    async fn get_inactive_comment_by_id(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<Option<CommentEntity>, AppError>;

    async fn delete_inactive_comment(
        &self,
        post_id: &Uuid,
        comment_id: &Uuid,
    ) -> Result<(), AppError>;
}
