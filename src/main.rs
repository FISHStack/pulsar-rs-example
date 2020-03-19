use pulsar::{self, Pulsar, SerializeMessage, ProducerError, producer};
use tokio::prelude::*;

#[derive(Debug)]
pub struct SomeData {

}

impl SerializeMessage for SomeData {
    fn serialize_message(input: &Self) -> Result<producer::Message, pulsar::Error> {
        unimplemented!()
    }
}

fn run() {
    let addr = "127.0.0.1:6650".parse().unwrap();
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let pulsar: Pulsar = Pulsar::new(addr, None, runtime.executor())
        .wait().unwrap();

    let producer = pulsar.producer(None);

    let message = SomeData {};

    runtime.executor().spawn({
        producer.send("some_topic", &message)
            .map(drop)
            .map_err(|e| eprintln!("Error handling! {}", e))
    });
}

docker run -it -p 9527:9527 -e REDIRECT_HOST=front-end-ip -e REDIRECT_PORT=front-end-port -e DRIVER_CLASS_NAME=org.postgresql.Driver -e URL=postgres -e USERNAME=root -e PASSWORD=pulsar -e LOG_LEVEL=DEBUG apachepulsar/pulsar-manager /bin/sh