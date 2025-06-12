use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::Consumer;
use std::io;
use std::io::Write;
use std::time::Duration;

pub fn consume() -> kafka::Result<()> {
    let mut c = {
        let mut cb = Consumer::from_hosts(vec![String::from("localhost:9092")])
            .with_group("kafka-consumer-group-producer".into())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_fetch_max_wait_time(Duration::from_secs(1))
            .with_fetch_max_bytes_per_partition(100_000)
            .with_retry_max_bytes_limit(1_000_000)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .with_client_id("kafka-rust-console-consumer".into())
            .with_topic("my-topic".into());

        cb.create()?
    };

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = Vec::with_capacity(1024);

    loop {
        for ms in c.poll()?.iter() {
            for m in ms.messages() {
                // ~ clear the output buffer
                unsafe { buf.set_len(0) };
                // ~ format the message for output
                let _ = writeln!(buf, "{}:{}@{}:", ms.topic(), ms.partition(), m.offset);
                buf.extend_from_slice(m.value);
                buf.push(b'\n');
                // ~ write to output channel
                stdout.write_all(&buf)?;
            }
            let _ = c.consume_messageset(ms);
        }
        c.commit_consumed()?;

        return Ok(());
    }
}
