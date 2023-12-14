# Rust web 开发-6.kafka
本系列文章从以下几个方面学习如何使用 Rust 进行 web 开发。

1. web 框架
2. 数据库/orm
3. config
4. log
5. 线程池
6. kafka
7. redis
8. 打包成 docker 镜像
   ……



---
本篇文章介绍一下 Rust 使用消息中间件 kafka。

Rust 接入 kafka 的库中，使用最多的是 rdkafka。本篇文章就来介绍一下 rdkafka，以及封装一个开箱即用的 simple-kafka 库。

## rdkafka
rdkafka 是 Rust 的 Kafka 客户端库，提供了与 Kafka 的低级别交互。它是对 librdkafka C 库的绑定，提供了高性能和可靠性。rdkafka 提供了广泛的功能，包括生产者和消费者 API、支持各种配置选项、消息压缩、事务等。

rdkafka 提供了高阶的 API：StreamConsumer、FutureProducer。

- StreamConsumer:负责自动轮询消费者的消息流。
- FutureProducer:为生成的每条消息返回一个 Future，Future 中可以得到消息的发送结果。


```toml
rdkafka = "0.36.0"
```


### 创建生产者
```rust
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
```

### 发送消息
有两个方法可以使用：send、send_result


1. `send` 方法：`send` 方法用于发送消息到 Kafka，并返回一个 `DeliveryFuture` 对象。它会尝试将消息发送到 Kafka，如果发送失败，将会在 `DeliveryFuture` 中返回一个错误。`send` 方法会在发送过程中进行重试，如果 Kafka 的生产者队列已满，可以使用 `queue_timeout` 参数来控制重试的时间。你可以将 `queue_timeout` 设置为 `Timeout::Never` 来永远重试，或者设置为 `Timeout::After(0)` 来立即返回错误。如果重试超时并且队列仍然满，`DeliveryFuture` 中将报告一个 `RDKafkaErrorCode::QueueFull` 错误。

2. `send_result` 方法：`send_result` 方法与 `send` 方法类似，同样用于发送消息到 Kafka，并返回一个 `DeliveryResult` 对象。但是，与 `send` 方法不同的是，如果消息无法入队，`send_result` 方法会立即返回一个错误，同时返回提供的 `FutureRecord` 对象。它不会进行重试操作。

发送消息的时候需要构建一个 `FutureRecord`对象
```rust
let message_record = FutureRecord::to(TOPIC).key("1").payload(message.as_bytes());
let result = producer.send(message_record, Timeout::After(Duration::from_secs(3))).await;
```

```log
2023-11-27 18:06:09.596  INFO ThreadId(25) rust_web::middleware::kafka: 54: create kafka producer,brokers=localhost:9092    
2023-11-27 18:06:09.597  INFO ThreadId(25) rust_web::middleware::kafka: 48: send message: FutureRecord { topic: "my_topic", partition: None, payload: Some([104, 101, 108, 108, 111]), key: Some("1"), timestamp: None, headers: None }    
2023-11-27 18:06:09.648  INFO ThreadId(25) rust_web::middleware::kafka: 50: send result: Ok((0, 7))    
2023-11-27 18:06:09.648  INFO ThreadId(05) rust_web::middleware::kafka: 97: kafka consumer message. message = [Message { ptr: 0x10c6043b8, event_ptr: 0x10c604340 }]    
2023-11-27 18:06:09.648  INFO ThreadId(05) rust_web::middleware::kafka: 111: partition = 0, offset = 7 message : "hello"    
2023-11-27 18:06:09.648  INFO ThreadId(05) rust_web::middleware::kafka: 93: recv...    
```

简单的使用 Rust 发送和接收 kafka 消息。以上代码在 [github](https://github.com/hitolz/rust-web/tree/kafka)。

## simple-kafka

基于以上代码，做了一个简化使用 rdkafka 组件的小工具 [simple-kafka](https://crates.io/crates/simple-kafka)。
使用这个小工具之后不用再写一堆的创建生产者、消费者代码，极大的减少了 kafka 的接入工作。

在 main 中，通过 tokio::spawn 线程初始化 kafka 生产者及消费者就能够在 Rust 中使用 kafka 了。

```rust
let _init_task = tokio::spawn(async {
    let simple_kafka_config:simple_kafka::KafkaConfig = kafka_config.to_owned().into();
    simple_kafka::kafka_init::init_producers(&simple_kafka_config).await;
    simple_kafka::kafka_init::init_consumers(&simple_kafka_config,"my_topic", message_handler).await;
});
```

simple-kafka 在 [github](https://github.com/hitolz/rust-web/tree/simple-kafka)。



## 小结
以上就是 Rust 使用 kafka 的简单示例，并写了一个小工具减少配置相关的代码。

