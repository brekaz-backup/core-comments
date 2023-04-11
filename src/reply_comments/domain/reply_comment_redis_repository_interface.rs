use async_trait::async_trait;
use blumer_lib_errors::AppError;

#[async_trait]
pub trait ReplyCommentRedisRepositoryInterface {
    async fn get_reply_comment_page_state(
        &self,
        reply_comment_page_state_id: &String,
    ) -> Result<Option<Vec<u8>>, AppError>;
    async fn store_reply_comment_page_state(
        &self,
        reply_comment_page_state_id: &String,
        data: &Vec<u8>,
    ) -> Result<(), AppError>;
    async fn delete_reply_comment_page_state(
        &self,
        reply_comment_page_state_id: &String,
    ) -> Result<(), AppError>;
}
