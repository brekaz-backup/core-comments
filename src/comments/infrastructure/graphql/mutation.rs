use super::objects::{CommentsCreateInput, CommentsDeleteInput, CreateOutput, Status};
use crate::infrastructure::graphql::state::AppState;
use crate::reply_comments::domain::ReplyCommentRepository;
use crate::{
    adapters::kafka::KafkaProducer,
    comments::{
        application::{CreateCommentUseCase, DeleteCommentUseCase},
        domain::CommentRepository,
    },
};
use async_graphql::*;
use blumer_lib_auth_rs::{Role, RoleGuard, User};
use blumer_lib_authorization_rs::clients::post::PostAuthorization;

#[derive(Default)]
pub struct CommentMutation;

#[Object]
impl CommentMutation {
    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn create_comment(
        &self,
        ctx: &Context<'_>,
        comment: CommentsCreateInput,
    ) -> FieldResult<CreateOutput> {
        let post_client = ctx.data::<PostAuthorization>()?.clone();
        let app_state: &AppState = ctx.data::<AppState>()?;
        let user: User = User::get_user(ctx).extend()?;
        let comment_repository: &CommentRepository = &app_state.comment_repository;
        let kafka_producer: &KafkaProducer = &app_state.kafka_producer;

        let result = CreateCommentUseCase::execute(
            post_client,
            &comment_repository,
            comment,
            user.user_id,
            kafka_producer,
        )
        .await?;

        Ok(CreateOutput {
            status: Status::Created,
            message: result.to_string(),
        })
    }

    pub async fn delete_comment(
        &self,
        ctx: &Context<'_>,
        comment: CommentsDeleteInput,
    ) -> FieldResult<bool> {
        let post_client = ctx.data::<PostAuthorization>()?.clone();
        let app_state: &AppState = ctx.data::<AppState>()?;
        let user: User = User::get_user(ctx).extend()?;
        let comment_repository: &CommentRepository = &app_state.comment_repository;
        let reply_comment_repository: &ReplyCommentRepository = &app_state.reply_comment_repository;
        let kafka_producer: &KafkaProducer = &app_state.kafka_producer;

        let result = DeleteCommentUseCase::execute(
            post_client,
            &comment_repository,
            &reply_comment_repository,
            comment,
            user.user_id,
            kafka_producer,
        )
        .await?;

        Ok(result)
    }
}
