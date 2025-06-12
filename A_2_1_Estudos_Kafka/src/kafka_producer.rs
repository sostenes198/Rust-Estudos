use kafka::client::{Compression, KafkaClient};
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

pub fn produce() -> kafka::Result<()> {
    let mut client = KafkaClient::new(vec![String::from("localhost:9092")]);
    client.set_client_id("kafka-rust-console-producer".into());
    client.load_metadata_all()?;

    let mut producer = Producer::from_client(client)
        .with_ack_timeout(Duration::from_secs(5))
        .with_required_acks(RequiredAcks::All)
        .with_compression(Compression::GZIP)
        .with_connection_idle_timeout(Duration::from_secs(30))
        .create()?;
    
    producer.send(&mut Record::from_value("my-topic", "test-value"))?;
    producer.send(&mut Record::from_value("my-topic", "test-value-1"))?;
    producer.send(&mut Record::from_value("my-topic", "test-value-2"))?;
    producer.send(&mut Record::from_value("my-topic", "test-value-3"))
}
