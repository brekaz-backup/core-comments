use async_trait::async_trait;
use blumer_lib_errors::AppError;

#[async_trait]
pub trait CommentRedisRepositoryInterface {
    async fn get_comment_page_state(
        &self,
        comment_page_state_id: &String,
    ) -> Result<Option<Vec<u8>>, AppError>;
    async fn store_comment_page_state(
        &self,
        comment_page_state_id: &String,
        data: &Vec<u8>,
    ) -> Result<(), AppError>;
    async fn delete_comment_page_state(
        &self,
        comment_page_state_id: &String,
    ) -> Result<(), AppError>;
}
