use super::objects::{CommentsReplyCreateInput, CommentsReplyDeleteInput};
use crate::adapters::kafka::KafkaProducer;
use crate::comments::domain::CommentRepository;
use crate::comments::infrastructure::graphql::objects::{CreateOutput, Status};
use crate::infrastructure::graphql::state::AppState;
use crate::reply_comments::application::{CreateReplyCommentUseCase, DeleteReplyCommentUseCase};
use crate::reply_comments::domain::ReplyCommentRepository;
use async_graphql::*;
use blumer_lib_auth_rs::{Role, RoleGuard, User};
use blumer_lib_authorization_rs::clients::post::PostAuthorization;

#[derive(Default)]
pub struct ReplyCommentMutation;

#[Object]
impl ReplyCommentMutation {
    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn create_comment_reply(
        &self,
        ctx: &Context<'_>,
        comment_reply: CommentsReplyCreateInput,
    ) -> FieldResult<CreateOutput> {
        let post_client = ctx.data::<PostAuthorization>()?.clone();
        let app_state: &AppState = ctx.data::<AppState>()?;
        let user: User = User::get_user(ctx).extend()?;
        let comment_repository: &CommentRepository = &app_state.comment_repository;
        let reply_comment_repository: &ReplyCommentRepository = &app_state.reply_comment_repository;
        let kafka_producer: &KafkaProducer = &app_state.kafka_producer;

        let result = CreateReplyCommentUseCase::execute(
            post_client,
            &comment_repository,
            &reply_comment_repository,
            comment_reply,
            user.user_id,
            kafka_producer,
        )
        .await?;

        Ok(CreateOutput {
            status: Status::Created,
            message: result.to_string(),
        })
    }

    #[graphql(guard = "RoleGuard::new(vec![Role::USER])")]
    pub async fn delete_comment_reply(
        &self,
        ctx: &Context<'_>,
        comment_reply: CommentsReplyDeleteInput,
    ) -> FieldResult<bool> {
        let post_client = ctx.data::<PostAuthorization>()?.clone();
        let app_state: &AppState = ctx.data::<AppState>()?;
        let user: User = User::get_user(ctx).extend()?;
        let reply_comment_repository: &ReplyCommentRepository = &app_state.reply_comment_repository;
        let kafka_producer: &KafkaProducer = &app_state.kafka_producer;

        let result = DeleteReplyCommentUseCase::execute(
            post_client,
            &reply_comment_repository,
            comment_reply,
            user.user_id,
            kafka_producer,
        )
        .await?;

        Ok(result)
    }
}
