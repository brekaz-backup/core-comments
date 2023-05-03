use log::{info, warn};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;
use std::time::Duration;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        let producer = ClientConfig::new()
            .set(
                "bootstrap.servers",
                std::env::var("KAFKA_BROKER").expect("Can't get GRAPHQL_PORT env var"),
            )
            .set(
                "security.protocol",
                std::env::var("KAFKA_SECURITY_PROTOCOL")
                    .expect("Can't get KAFKA_SECURITY_PROTOCOL env var"),
            )
            .set(
                "sasl.mechanisms",
                std::env::var("KAFKA_SASL_MECHANISMS")
                    .expect("Can't get KAFKA_SASL_MECHANISMS env var"),
            )
            .set(
                "sasl.username",
                std::env::var("KAFKA_SASL_USERNAME")
                    .expect("Can't get KAFKA_SASL_USERNAME env var"),
            )
            .set(
                "sasl.password",
                std::env::var("KAFKA_SASL_PASSWORD")
                    .expect("Can't get KAFKA_SASL_PASSWORD env var"),
            )
            .set("message.timeout.ms", "45000")
            .create()
            .expect("Producer creation failed");
        KafkaProducer { producer }
    }

    pub async fn send_message(&self, kafka_topic: &str, message: &Vec<u8>) {
        let record: FutureRecord<String, Vec<u8>> = FutureRecord::to(kafka_topic).payload(&message);
        let delivery_status = self
            .producer
            .send(record, Timeout::After(Duration::from_secs(0)))
            .await;

        match delivery_status {
            Ok(_) => info!("Message was sent, topic: {}", kafka_topic),
            Err(res) => warn!("Message wasn't sent: {}", res.0),
        }
    }
}
