use super::mapper::CommentMapper;
use super::objects::{CommentOutput, CommentResponseOutput, SimpleProfile};
use crate::comments::application::GetCommentsByPostIdUseCase;
use crate::comments::domain::{CommentEntity, CommentRedisRepository, CommentRepository};
use crate::infrastructure::graphql::state::AppState;
use async_graphql::*;
use blumer_lib_auth_rs::{Role, RoleGuard, User};
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default)]
pub struct CommentQuery;

#[Object]
impl CommentQuery {
    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn get_post_comments(
        &self,
        ctx: &Context<'_>,
        post_id: ID,
        next_page: Option<String>,
    ) -> FieldResult<CommentResponseOutput> {
        let post_client = ctx.data::<PostAuthorization>()?.clone();
        let app_state: &AppState = ctx.data::<AppState>()?;
        let user: User = User::get_user(ctx).extend()?;
        let comment_repository: &CommentRepository = &app_state.comment_repository;
        let comment_redis_repository: &CommentRedisRepository = &app_state.comment_redis_repository;

        let result: (Vec<CommentEntity>, String) = GetCommentsByPostIdUseCase::execute(
            post_client,
            &comment_repository,
            &comment_redis_repository,
            Uuid::from_str(&post_id).map_err(|e| AppError::DatasourceError(e.to_string()))?,
            user.user_id,
            next_page,
        )
        .await
        .extend()?;

        let comments: Result<Vec<CommentOutput>, AppError> = result
            .0
            .into_iter()
            .map(|comment| Ok(CommentMapper::object(comment)?))
            .collect();

        let next_page: Option<String> = if result.1.is_empty() || result.1 == "" {
            None
        } else {
            Some(result.1)
        };

        Ok(CommentResponseOutput {
            comments: comments?,
            next_page,
        })
    }

    #[graphql(entity, guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn get_comment_by_id(&self, _ctx: &Context<'_>, _id: ID) -> Option<CommentOutput> {
        None
    }

    #[graphql(entity, guard = "RoleGuard::new(vec![Role::USER])")]
    async fn find_profile_by_id(&self, id: ID) -> SimpleProfile {
        SimpleProfile { id }
    }
}
