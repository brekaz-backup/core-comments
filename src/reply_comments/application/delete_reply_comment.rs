use crate::adapters::kafka::KafkaProducer;
use crate::reply_comments::domain::{CommentReplyDeleteEntity, ReplyCommentRepositoryInterface};
use crate::reply_comments::infrastructure::capn_proto::models::{
    CommentRepliesCounterEntity as CommentRepliesCounterEntityProto,
    CommentReplyDeleteEntity as CommentReplyDeleteEntityProto,
    ReplyCountsDeleteEntity as ReplyCountsDeleteEntityProto,
};
use crate::reply_comments::infrastructure::capn_proto::{
    CommentRepliesCounterMapper, CommentReplyDeleteMapper, ReplyCountsDeleteMapper,
};
use crate::reply_comments::infrastructure::graphql::objects::CommentsReplyDeleteInput;
use crate::reply_comments::infrastructure::kafka::{
    KAFKA_TOPIC_COMMENT_REPLIES_COUNTER, KAFKA_TOPIC_COMMENT_REPLY_DELETE,
    KAFKA_TOPIC_DELETE_REPLY_COUNTS,
};
use crate::utils::general::can_view_post;
use anyhow::Result;
use async_graphql::*;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use uuid::Uuid;

pub struct DeleteReplyCommentUseCase;

impl DeleteReplyCommentUseCase {
    pub async fn execute(
        post_authorization: PostAuthorization,
        reply_comment_repo: &impl ReplyCommentRepositoryInterface,
        reply_comment: CommentsReplyDeleteInput,
        user_id: Uuid,
        kafka_producer: &KafkaProducer,
    ) -> Result<bool, AppError> {
        let post_owner_id = can_view_post(
            post_authorization,
            reply_comment.post_id.to_string().parse::<Uuid>().unwrap(),
            user_id,
        )
        .await
        .extend()
        .unwrap()
        .data
        .unwrap()
        .owner_id;

        let reply_comment_db = reply_comment_repo
            .get_comment_reply_by_id(
                &Uuid::parse_str(&reply_comment.post_id.to_string())
                    .expect("Error when parsing post_id"),
                &Uuid::parse_str(&reply_comment.comment_id.to_string())
                    .expect("Error when parsing comment_id"),
                &Uuid::parse_str(&reply_comment.reply_id.to_string())
                    .expect("Error when parsing reply_id"),
            )
            .await
            .expect("Error when getting reply comment")
            .expect("Error when getting reply comment");

        if user_id != post_owner_id && reply_comment_db.user_id != user_id {
            return Ok(false);
        }

        let reply_comment_db = CommentReplyDeleteEntity {
            post_id: Uuid::parse_str(&reply_comment.post_id.to_string())
                .expect("Error when parsing post_id"),
            comment_id: Uuid::parse_str(&reply_comment.comment_id.to_string())
                .expect("Error when parsing comment_id"),
            reply_id: Uuid::parse_str(&reply_comment.reply_id.to_string())
                .expect("Error when parsing reply_id"),
        };

        reply_comment_repo
            .delete_comment_reply(reply_comment_db.clone())
            .await?;

        let obj = CommentReplyDeleteEntityProto {
            post_id: reply_comment_db.post_id,
            user_id,
            comment_id: reply_comment_db.comment_id,
            reply_id: reply_comment_db.reply_id,
            post_owner_id: post_owner_id,
        };
        let message = CommentReplyDeleteMapper::proto(&obj).await.unwrap();
        kafka_producer
            .send_message(KAFKA_TOPIC_COMMENT_REPLY_DELETE, &message)
            .await;

        let obj = ReplyCountsDeleteEntityProto {
            comment_id: reply_comment_db.comment_id,
            reply_id: reply_comment_db.reply_id,
        };
        let message = ReplyCountsDeleteMapper::proto(&obj).await.unwrap();
        kafka_producer
            .send_message(KAFKA_TOPIC_DELETE_REPLY_COUNTS, &message)
            .await;

        let obj = CommentRepliesCounterEntityProto {
            comment_id: reply_comment_db.comment_id,
            replies_count: -1,
        };
        let message = CommentRepliesCounterMapper::proto(&obj).await.unwrap();
        kafka_producer
            .send_message(KAFKA_TOPIC_COMMENT_REPLIES_COUNTER, &message)
            .await;

        return Ok(true);
    }
}
