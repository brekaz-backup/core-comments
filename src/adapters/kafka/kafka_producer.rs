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
