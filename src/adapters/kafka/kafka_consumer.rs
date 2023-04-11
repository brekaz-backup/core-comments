use crate::ports::kafka::KafkaConsumerInterface;
use log::{info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    ClientConfig,
};

pub struct KafkaConsumer<T: KafkaConsumerInterface> {
    ctrl: T,
    consumer: StreamConsumer,
}
impl<T: KafkaConsumerInterface> KafkaConsumer<T> {
    pub fn new(topics: Vec<&str>, ctrl: T) -> KafkaConsumer<T> {
        KafkaConsumer {
            ctrl,
            consumer: Self::create_consumer(topics),
        }
    }

    pub async fn consume(&self) {
        info!("Starting event loop");
        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(borrowed_message) => {
                    self.ctrl
                        .record_borrowed_message_receipt(&borrowed_message)
                        .await;
                    let owned_message = borrowed_message.detach();
                    let _ = self.ctrl.record_owned_message_receipt(&owned_message).await;
                }
            }
        }
    }

    fn create_consumer(kafka_topics: Vec<&str>) -> StreamConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", std::env::var("APP_NAME").unwrap())
            .set("bootstrap.servers", std::env::var("KAFKA_BROKER").unwrap())
            .set(
                "security.protocol",
                std::env::var("KAFKA_SECURITY_PROTOCOL").unwrap(),
            )
            .set(
                "sasl.mechanisms",
                std::env::var("KAFKA_SASL_MECHANISMS").unwrap(),
            )
            .set(
                "sasl.username",
                std::env::var("KAFKA_SASL_USERNAME").unwrap(),
            )
            .set(
                "sasl.password",
                std::env::var("KAFKA_SASL_PASSWORD").unwrap(),
            )
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "45000")
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(kafka_topics.as_slice())
            .expect("Can't subscribe to specified topics");

        consumer
    }
}
