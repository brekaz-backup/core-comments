use crate::reply_comments::domain::{
    CommentReplyEntity, ReplyCommentRedisRepositoryInterface, ReplyCommentRepositoryInterface,
};
use crate::utils::general::can_view_post;
use anyhow::Result;
use async_graphql::*;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use uuid::Uuid;

pub struct GetReplyCommentsByPostIdUseCase;

impl GetReplyCommentsByPostIdUseCase {
    pub async fn execute(
        post_authorization: PostAuthorization,
        reply_comment_repo: &impl ReplyCommentRepositoryInterface,
        reply_comment_redis_repo: &impl ReplyCommentRedisRepositoryInterface,
        post_id: Uuid,
        comment_id: Uuid,
        user_id: Uuid,
        next_page: Option<String>,
    ) -> Result<(Vec<CommentReplyEntity>, String), AppError> {
        can_view_post(post_authorization, post_id, user_id)
            .await
            .extend()
            .unwrap();

        let incoming_page_state: Option<Vec<u8>>;
        let mut reply_comment_page_state_id: String;

        match next_page {
            Some(page_state) => {
                incoming_page_state = reply_comment_redis_repo
                    .get_reply_comment_page_state(&page_state)
                    .await?;
                reply_comment_page_state_id = page_state;
            }
            None => {
                incoming_page_state = None;
                reply_comment_page_state_id = format!("{}:{}", user_id, Uuid::new_v4());
            }
        };

        // Getting the comments from the database
        let result: (Vec<CommentReplyEntity>, Option<Vec<u8>>) = reply_comment_repo
            .get_all_comment_replies_from_comment_id(post_id, comment_id, incoming_page_state)
            .await?;

        let reply_comments: Vec<CommentReplyEntity> = result.0;
        let reply_comment_page_state: Option<Vec<u8>> = result.1;

        match reply_comment_page_state {
            Some(reply_comment_page_state) => {
                reply_comment_redis_repo
                    .store_reply_comment_page_state(
                        &reply_comment_page_state_id,
                        &reply_comment_page_state,
                    )
                    .await?;
            }
            None => {
                if reply_comment_page_state_id.is_empty() {
                    reply_comment_redis_repo
                        .delete_reply_comment_page_state(&reply_comment_page_state_id)
                        .await?;
                }
                reply_comment_page_state_id = "".to_string();
            }
        }
        return Ok((reply_comments, reply_comment_page_state_id));
    }
}
