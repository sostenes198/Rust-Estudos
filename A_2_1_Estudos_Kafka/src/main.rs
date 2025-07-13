mod kafka_consumer;
mod kafka_producer;

fn main() {
    // kafka_producer::produce().unwrap();
    kafka_consumer::consume().unwrap();
}
