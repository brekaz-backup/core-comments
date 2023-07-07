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
use blumer_lib_errors::AppError;
use std::str::FromStr;
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
        let post_client = ctx.data::<PostAuthorization>().extend()?.clone();
        let app_state: &AppState = ctx.data::<AppState>().extend()?;
        let user: User = User::get_user(ctx).extend()?;
        let reply_comment_repository: &ReplyCommentRepository = &app_state.reply_comment_repository;
        let reply_comment_redis_repository: &ReplyCommentRedisRepository =
            &app_state.reply_comment_redis_repository;

        let result: (Vec<CommentReplyEntity>, String) = GetReplyCommentsByPostIdUseCase::execute(
            post_client,
            &reply_comment_repository,
            &reply_comment_redis_repository,
            Uuid::from_str(&post_id)
                .map_err(|e| AppError::DatasourceError(e.to_string()))
                .extend()?,
            Uuid::from_str(&comment_id)
                .map_err(|e| AppError::DatasourceError(e.to_string()))
                .extend()?,
            user.user_id,
            next_page,
        )
        .await
        .extend()?;

        let reply_comments: Result<Vec<CommentReplyOutput>, AppError> = result
            .0
            .into_iter()
            .map(|reply| Ok(ReplyCommentMapper::object(reply)?))
            .collect();

        let next_page: Option<String> = if result.1.is_empty() || result.1 == "" {
            None
        } else {
            Some(result.1)
        };

        Ok(ReplyResponseOutput {
            comments: reply_comments?,
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
