use log::info;
use simple_kafka::KafkaMessage;

pub const TOPIC: &str = "my_topic";

/// consumer 消费消息的测试方法
pub fn message_handler(message: KafkaMessage) {
    let partition = message.partition;
    if let Some(value) = &message.value {
        let value = String::from_utf8_lossy(value);
        info!(
            "partition = {:#?}, offset = {:?} message : {:#?}",
            partition, message.offset, value.to_string()
        );
    }
}
