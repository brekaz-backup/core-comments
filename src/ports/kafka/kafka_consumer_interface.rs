use anyhow::Result;
use async_trait::async_trait;
use rdkafka::message::{BorrowedMessage, OwnedMessage};

#[async_trait]
pub trait KafkaConsumerInterface {
    async fn record_borrowed_message_receipt(&self, msg: &BorrowedMessage<'_>);

    async fn record_owned_message_receipt(&self, msg: &OwnedMessage) -> Result<()>;
}
