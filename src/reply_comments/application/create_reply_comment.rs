use crate::adapters::kafka::KafkaProducer;
use crate::comments::domain::CommentRepositoryInterface;
use crate::comments::infrastructure::graphql::objects::CommentTypeInput;
use crate::reply_comments::domain::{CommentReplyCreateEntity, ReplyCommentRepositoryInterface};
use crate::reply_comments::infrastructure::capn_proto::models::{
    CommentRepliesCounterEntity as CommentRepliesCounterEntityProto,
    CommentReplyCreateEntity as CommentReplyCreateEntityProto,
    ReplyCountsEntity as ReplyCountsEntityProto,
};
use crate::reply_comments::infrastructure::capn_proto::{
    CommentRepliesCounterMapper, CommentReplyCreateMapper, ReplyCountsMapper,
};
use crate::reply_comments::infrastructure::graphql::objects::CommentsReplyCreateInput;
use crate::reply_comments::infrastructure::kafka::{
    KAFKA_TOPIC_COMMENT_REPLIES_COUNTER, KAFKA_TOPIC_COMMENT_REPLY_CREATE,
    KAFKA_TOPIC_CREATE_REPLY_COUNTS,
};
use crate::utils::general::{can_view_post, comment_description_max_len};
use anyhow::Result;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use std::str::FromStr;
use uuid::Uuid;

pub struct CreateReplyCommentUseCase;

impl CreateReplyCommentUseCase {
    pub async fn execute(
        post_authorization: PostAuthorization,
        comment_repo: &impl CommentRepositoryInterface,
        reply_comment_repo: &impl ReplyCommentRepositoryInterface,
        reply_comment: CommentsReplyCreateInput,
        user_id: Uuid,
        kafka_producer: &KafkaProducer,
    ) -> Result<Uuid, AppError> {
        let reply_id = Uuid::new_v4();
        let reply_comment_post_id = Uuid::from_str(&reply_comment.post_id)
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;
        let reply_comment_comment_id = Uuid::from_str(&reply_comment.comment_id)
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;

        let post_owner_id = can_view_post(post_authorization, reply_comment_post_id, user_id)
            .await?
            .data
            .unwrap()
            .owner_id;

        let comment_owner_id = comment_repo
            .get_comment_by_id(&reply_comment_post_id, &reply_comment_comment_id)
            .await?
            .ok_or(AppError::DatasourceError(
                "Error getting comment".to_owned(),
            ))?
            .user_id;

        let gif: Option<String>;
        let active: bool;
        match reply_comment.comment_type {
            CommentTypeInput::Text => {
                gif = None;
                active = true;
            }
            CommentTypeInput::Gif => {
                gif = reply_comment.gif;
                active = true;
                if reply_comment.description.to_owned().is_some() {
                    return Err(AppError::ValidationError {
                        reason: "Comment description not allowed".to_owned(),
                        code: "DESCRIPTION_NOT_ALLOWED".to_owned(),
                    });
                }
            }
            CommentTypeInput::TextAndImage => {
                gif = None;
                active = false;
            }
            _ => {
                gif = None;
                active = false;
                if reply_comment.description.to_owned().is_some() {
                    return Err(AppError::ValidationError {
                        reason: "Comment description not allowed".to_owned(),
                        code: "DESCRIPTION_NOT_ALLOWED".to_owned(),
                    });
                }
            }
        };

        if reply_comment.description.to_owned().is_some() {
            let over_max_len: bool =
                comment_description_max_len(&reply_comment.description.to_owned().unwrap());
            if over_max_len {
                return Err(AppError::ValidationError {
                    reason: "Description is too long".to_owned(),
                    code: "DESCRIPTION_TO_LONG".to_owned(),
                });
            }
        }

        let comment_reply_db = CommentReplyCreateEntity {
            post_id: reply_comment_post_id,
            comment_id: reply_comment_comment_id,
            reply_id,
            user_id,
            description: reply_comment.description,
            image: None,
            audio: None,
            gif,
            active,
        };

        reply_comment_repo
            .create_comment_reply(comment_reply_db.clone())
            .await?;

        let obj = CommentReplyCreateEntityProto {
            post_id: comment_reply_db.post_id,
            comment_id: comment_reply_db.comment_id,
            reply_id: comment_reply_db.reply_id,
            user_id: comment_reply_db.user_id,
            comment_owner_id: comment_owner_id,
            description: comment_reply_db.description,
            image: comment_reply_db.image,
            audio: comment_reply_db.audio,
            gif: comment_reply_db.gif,
            active: comment_reply_db.active,
            post_owner_id: post_owner_id,
        };
        let message = CommentReplyCreateMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_COMMENT_REPLY_CREATE, &message)
            .await;

        let obj = ReplyCountsEntityProto {
            comment_id: comment_reply_db.comment_id,
            reply_id: comment_reply_db.reply_id,
            reactions_count_1: 0,
            reactions_count_2: 0,
            reactions_count_3: 0,
            reactions_count_4: 0,
            reactions_count_5: 0,
            reactions_count_6: 0,
        };
        let message = ReplyCountsMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_CREATE_REPLY_COUNTS, &message)
            .await;

        let obj = CommentRepliesCounterEntityProto {
            comment_id: comment_reply_db.comment_id,
            replies_count: 1,
        };
        let message = CommentRepliesCounterMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_COMMENT_REPLIES_COUNTER, &message)
            .await;

        return Ok(reply_id);
    }
}
