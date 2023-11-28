use std::collections::HashMap;
use std::time::Duration;

use log::{info, warn};
use rdkafka::{
    config::RDKafkaLogLevel,
    consumer::{Consumer, StreamConsumer},
    message::OwnedMessage,
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};
use rdkafka::util::Timeout;

use crate::config;

const TOPIC: &str = "my_topic";

#[derive(Default, Debug)]
pub struct KafkaMessage {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
    pub key: Option<Vec<u8>>,
    pub value: Option<Vec<u8>>,
    pub timestamp: Option<i64>,
    pub headers: Option<HashMap<String, String>>,
}

impl From<OwnedMessage> for KafkaMessage {
    fn from(v: OwnedMessage) -> Self {
        KafkaMessage {
            topic: v.topic().to_owned(),
            partition: v.partition(),
            offset: v.offset(),
            key: v.key().map(|v| v.to_vec()),
            value: v.payload().map(|v| v.to_vec()),
            timestamp: v.timestamp().to_millis(),
            headers: Some(HashMap::new()),
        }
    }
}

pub async fn send(message: &str) {
    let kafka_config: &config::KafkaConfig = &config::SERVER_CONFIG.kafka_config;
    let brokers = &kafka_config.brokers;
    let producer = create_producer(brokers);
    let message_record = FutureRecord::to(TOPIC).key("1").payload(message.as_bytes());
    info!("send message: {:?}", message_record);
    let result = producer.send(message_record, Timeout::After(Duration::from_secs(3))).await;
    info!("send result: {:?}", result);
}

fn create_producer(brokers: &str) -> FutureProducer {
    info!("create kafka producer,brokers={}", brokers);
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .set("acks", "1")
        .create()
        .expect("Failed to create producer");
    producer
}

pub async fn init_consumer() {
    let kafka_config: &config::KafkaConfig = &config::SERVER_CONFIG.kafka_config;
    let brokers = &kafka_config.brokers;
    let group_id = &kafka_config.group_id;

    info!(
        "init consumer,topic = {},brokers = {},group id = {}",
        TOPIC, brokers, group_id
    );

    let stream_consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    // 订阅主题
    stream_consumer
        .subscribe(&vec![TOPIC])
        .expect("Can't subscribe to specified topics");

    info!("subscribe to topics");

    tokio::spawn(async move {
        loop {
            info!("recv...");
            match stream_consumer.recv().await {
                Err(e) => warn!("kafka error: {}", e),
                Ok(m) => {
                    info!("kafka consumer message. message = [{:#?}]", m);
                    let message: KafkaMessage = KafkaMessage::from(m.detach());
                    message_handler(message);
                }
            }
        }
    });
}

/// consumer 消费消息的测试方法
pub fn message_handler(message: KafkaMessage) {
    let partition = message.partition;
    if let Some(value) = &message.value {
        let value = String::from_utf8_lossy(value);
        info!(
            "partition = {:#?}, offset = {:?} message : {:#?}",
            partition, message.offset, value
        );
    }
}
