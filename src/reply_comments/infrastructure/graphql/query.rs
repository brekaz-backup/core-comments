use super::mapper::ReplyCommentMapper;
use super::objects::{CommentReplyOutput, ReplyResponseOutput};
use crate::comments::infrastructure::graphql::objects::SimpleProfile;
use crate::infrastructure::graphql::state::AppState;
use crate::reply_comments::application::GetReplyCommentsByPostIdUseCase;
use crate::reply_comments::domain::{
    CommentReplyEntity, ReplyCommentRedisRepository, ReplyCommentRepository,
};
use async_graphql::*;
use blumer_lib_auth_rs::{Role, RoleGuard, User};
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use uuid::Uuid;

#[derive(Default)]
pub struct ReplyCommentQuery;

#[Object]
impl ReplyCommentQuery {
    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn get_comments_replies(
        &self,
        ctx: &Context<'_>,
        post_id: ID,
        comment_id: ID,
        next_page: Option<String>,
    ) -> FieldResult<ReplyResponseOutput> {
        let post_client = ctx.data::<PostAuthorization>().unwrap().clone();
        let app_state: &AppState = ctx.data::<AppState>()?;
        let user: User = User::get_user(ctx).expect("User not found");
        let reply_comment_repository: &ReplyCommentRepository = &app_state.reply_comment_repository;
        let reply_comment_redis_repository: &ReplyCommentRedisRepository =
            &app_state.reply_comment_redis_repository;

        let result: (Vec<CommentReplyEntity>, String) = GetReplyCommentsByPostIdUseCase::execute(
            post_client,
            &reply_comment_repository,
            &reply_comment_redis_repository,
            Uuid::parse_str(&post_id).unwrap(),
            Uuid::parse_str(&comment_id).unwrap(),
            user.user_id,
            next_page,
        )
        .await?;

        let reply_comments: Vec<CommentReplyOutput> = result
            .0
            .into_iter()
            .map(|reply| ReplyCommentMapper::object(reply))
            .collect();

        let next_page: Option<String> = if result.1.is_empty() || result.1 == "" {
            None
        } else {
            Some(result.1)
        };

        Ok(ReplyResponseOutput {
            comments: reply_comments,
            next_page,
        })
    }

    #[graphql(entity, guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn get_reply_comment_by_id(
        &self,
        _ctx: &Context<'_>,
        _id: ID,
    ) -> Option<CommentReplyOutput> {
        None
    }

    #[graphql(entity, guard = "RoleGuard::new(vec![Role::USER])")]
    async fn find_profile_by_id(&self, id: ID) -> SimpleProfile {
        SimpleProfile { id }
    }
}
