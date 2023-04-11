use super::KAFKA_TOPIC_COMMENT_UPLOAD;
use crate::comments::application::MoveCommentToActiveUseCase;
use crate::comments::domain::CommentRepository;
use crate::media_comments::infrastructure::capn_proto::{
    models::CommentTypeEntity, CommentUploadMapper,
};
use crate::ports::kafka::KafkaConsumerInterface;
use crate::reply_comments::application::MoveReplyCommentToActiveUseCase;
use crate::reply_comments::domain::ReplyCommentRepository;
use anyhow::Result;
use async_trait::async_trait;
use log::info;
use rdkafka::{
    message::{BorrowedMessage, OwnedMessage},
    Message,
};

pub struct KafkaConsumer {
    comment_repository: CommentRepository,
    reply_comment_repository: ReplyCommentRepository,
}

impl KafkaConsumer {
    pub fn new(
        comment_repository: CommentRepository,
        reply_comment_repository: ReplyCommentRepository,
    ) -> KafkaConsumer {
        KafkaConsumer {
            comment_repository,
            reply_comment_repository,
        }
    }

    pub async fn comment_upload(&self, payload: &[u8]) {
        let event = CommentUploadMapper::event(&payload).await.unwrap();
        let _ = match event.comment_type {
            CommentTypeEntity::Comment => {
                MoveCommentToActiveUseCase::execute(&self.comment_repository, event).await
            }
            CommentTypeEntity::Reply => {
                MoveReplyCommentToActiveUseCase::execute(&self.reply_comment_repository, event)
                    .await
            }
        };
    }
}

#[async_trait]
impl KafkaConsumerInterface for KafkaConsumer {
    async fn record_borrowed_message_receipt(&self, msg: &BorrowedMessage<'_>) {
        info!("Message received Borrowed: {:?}", msg);
    }

    async fn record_owned_message_receipt(&self, msg: &OwnedMessage) -> Result<()> {
        //info!("Message received Owned: {:?}", msg);

        let payload = msg.payload().expect("Kafka message should contain payload");
        match msg.topic() {
            KAFKA_TOPIC_COMMENT_UPLOAD => self.comment_upload(payload).await,
            _ => {}
        };
        Ok(())
    }
}
