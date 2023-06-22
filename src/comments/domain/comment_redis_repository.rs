use super::CommentRedisRepositoryInterface;
use async_trait::async_trait;
use blumer_lib_errors::AppError;
use redis::cluster::ClusterClient;

#[derive(Clone)]
pub struct CommentRedisRepository {
    session: ClusterClient,
}

impl CommentRedisRepository {
    pub fn new(session: ClusterClient) -> Self {
        CommentRedisRepository { session }
    }
}

#[async_trait]
impl CommentRedisRepositoryInterface for &CommentRedisRepository {
    async fn get_comment_page_state(
        &self,
        comment_page_state_id: &String,
    ) -> Result<Option<Vec<u8>>, AppError> {
        let mut conn = self
            .session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res: Vec<u8> = redis::Cmd::get(comment_page_state_id)
            .query_async::<_, Vec<u8>>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        Ok(if !res.is_empty() { Some(res) } else { None })
    }

    async fn store_comment_page_state(
        &self,
        comment_page_state_id: &String,
        data: &Vec<u8>,
    ) -> Result<(), AppError> {
        let mut conn = self
            .session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res: String = redis::Cmd::set_ex(comment_page_state_id, data, 120)
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        // not strictly necessary, but successful SET operations return "OK"
        if res == "OK" {
            Ok(())
        } else {
            Err(AppError::DatasourceError(
                "Cant Store comment page state in redis".to_string(),
            ))
        }
    }

    async fn delete_comment_page_state(
        &self,
        comment_page_state_id: &String,
    ) -> Result<(), AppError> {
        let mut conn = self
            .session
            .get_async_connection()
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let res: usize = redis::Cmd::del(comment_page_state_id)
            .query_async::<_, usize>(&mut conn)
            .await
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        if res == 1 {
            Ok(())
        } else {
            Err(AppError::DatasourceError(
                "Cant Delete comment page state in redis".to_string(),
            ))
        }
    }
}
