use crate::adapters::kafka::KafkaProducer;
use crate::comments::domain::{CommentCreateEntity, CommentRepositoryInterface};
use crate::comments::infrastructure::capn_proto::models::{
    CommentCountsEntity as CommentCountsEntityProto,
    CommentCreateEntity as CommentCreateEntityProto,
};
use crate::comments::infrastructure::capn_proto::{CommentCountsMapper, CommentMapper};
use crate::comments::infrastructure::graphql::objects::{CommentTypeInput, CommentsCreateInput};
use crate::comments::infrastructure::kafka::{
    KAFKA_TOPIC_COMMENT_CREATE, KAFKA_TOPIC_CREATE_COMMENT_COUNTS,
};
use crate::utils::general::{can_view_post, comment_description_max_len};
use anyhow::Result;
use blumer_lib_authorization_rs::clients::post::PostAuthorization;
use blumer_lib_errors::AppError;
use std::str::FromStr;
use uuid::Uuid;

pub struct CreateCommentUseCase;

impl CreateCommentUseCase {
    pub async fn execute(
        post_authorization: PostAuthorization,
        comment_repo: &impl CommentRepositoryInterface,
        comment: CommentsCreateInput,
        user_id: Uuid,
        kafka_producer: &KafkaProducer,
    ) -> Result<Uuid, AppError> {
        let comment_post_id = Uuid::from_str(&comment.post_id)
            .map_err(|e| AppError::DatasourceError(e.to_string()))?;
        let original_post = can_view_post(post_authorization, comment_post_id, user_id).await?;
        let comment_id = Uuid::new_v4();
        let gif: Option<String>;
        let active: bool;

        match comment.comment_type {
            CommentTypeInput::Text => {
                gif = None;
                active = true;
            }
            CommentTypeInput::Gif => {
                gif = comment.gif;
                active = true;
                if comment.description.to_owned().is_some() {
                    return Err(AppError::ValidationError {
                        reason: "Comment description not allowed".to_string(),
                        code: "DESCRIPTION_NOT_ALLOWED".to_string(),
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
                if comment.description.to_owned().is_some() {
                    return Err(AppError::ValidationError {
                        reason: "Comment description not allowed".to_string(),
                        code: "DESCRIPTION_NOT_ALLOWED".to_string(),
                    });
                }
            }
        };

        if comment.description.to_owned().is_some() {
            let over_max_len: bool =
                comment_description_max_len(&comment.description.to_owned().unwrap());
            if over_max_len {
                return Err(AppError::ValidationError {
                    reason: "Description is too long".to_string(),
                    code: "DESCRIPTION_TOO_LONG".to_string(),
                });
            }
        }

        let comment_db = CommentCreateEntity {
            post_id: Uuid::from_str(&comment.post_id)
                .map_err(|e| AppError::DatasourceError(e.to_string()))?,
            user_id,
            comment_id,
            description: comment.description,
            image: None,
            audio: None,
            gif: gif,
            active,
        };
        comment_repo.create_comment(comment_db.clone()).await?;

        let obj = CommentCreateEntityProto {
            post_id: comment_db.post_id,
            user_id: comment_db.user_id,
            comment_id: comment_db.comment_id,
            post_owner_id: original_post.data.unwrap().owner_id,
            description: comment_db.description,
            image: comment_db.image,
            audio: comment_db.audio,
            gif: comment_db.gif,
            active: comment_db.active,
        };
        let message = CommentMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_COMMENT_CREATE, &message)
            .await;

        let obj = CommentCountsEntityProto {
            post_id: comment_db.post_id,
            comment_id: comment_db.comment_id,
            replies_count: 0,
            reactions_count_1: 0,
            reactions_count_2: 0,
            reactions_count_3: 0,
            reactions_count_4: 0,
            reactions_count_5: 0,
            reactions_count_6: 0,
        };
        let message = CommentCountsMapper::proto(&obj).await?;
        kafka_producer
            .send_message(KAFKA_TOPIC_CREATE_COMMENT_COUNTS, &message)
            .await;

        return Ok(comment_id);
    }
}
