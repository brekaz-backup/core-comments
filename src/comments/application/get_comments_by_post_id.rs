use crate::comments::domain::{
    CommentEntity, CommentRedisRepositoryInterface, CommentRepositoryInterface,
};
use crate::utils::general::can_view_post;
use anyhow::Result;
use async_graphql::*;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use uuid::Uuid;

pub struct GetCommentsByPostIdUseCase;

impl GetCommentsByPostIdUseCase {
    pub async fn execute(
        post_authorization: PostAuthorization,
        comment_repo: &impl CommentRepositoryInterface,
        comment_redis_repo: &impl CommentRedisRepositoryInterface,
        post_id: Uuid,
        user_id: Uuid,
        next_page: Option<String>,
    ) -> Result<(Vec<CommentEntity>, String), AppError> {
        can_view_post(post_authorization, post_id, user_id)
            .await
            .extend()
            .unwrap();

        let incoming_page_state: Option<Vec<u8>>;
        let mut comment_page_state_id: String;

        match next_page {
            Some(page_state) => {
                incoming_page_state = comment_redis_repo
                    .get_comment_page_state(&page_state)
                    .await?;
                comment_page_state_id = page_state;
            }
            None => {
                incoming_page_state = None;
                comment_page_state_id = format!("{}:{}", user_id, Uuid::new_v4());
            }
        };

        // Getting the comments from the database
        let result: (Vec<CommentEntity>, Option<Vec<u8>>) = comment_repo
            .get_comments_from_post_id(post_id, incoming_page_state)
            .await?;

        let comments: Vec<CommentEntity> = result.0;
        let comment_page_state: Option<Vec<u8>> = result.1;

        match comment_page_state {
            Some(comment_page_state) => {
                comment_redis_repo
                    .store_comment_page_state(&comment_page_state_id, &comment_page_state)
                    .await?;
            }
            None => {
                if comment_page_state_id.is_empty() {
                    comment_redis_repo
                        .delete_comment_page_state(&comment_page_state_id)
                        .await?;
                }
                comment_page_state_id = "".to_string();
            }
        }
        return Ok((comments, comment_page_state_id));
    }
}
