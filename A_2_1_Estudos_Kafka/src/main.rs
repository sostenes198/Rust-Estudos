mod kafka_producer;
mod kafka_consumer;

fn main() {
    // kafka_producer::produce().unwrap();
    kafka_consumer::consume().unwrap();
}
