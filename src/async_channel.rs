use async_channel::{bounded, Receiver, Sender};
use tokio::{task, try_join};

async fn publish(data_tx: Sender<Vec<u8>>, vector_size: usize) {
    for _ in 0..10 {
        let vec = vec![0u8; vector_size];
        data_tx
            .send(vec)
            .await
            .expect("Failed to send a vector to data_tx");
    }
}


async fn consume(data_rx: Receiver<Vec<u8>>) {
    while let Ok(chunk) = data_rx.recv().await {
        println!("Received: {:?}", chunk);
    }
}


pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_QUEUE_SIZE: usize = 10;
    let (data_tx, data_rx) = bounded(MAX_QUEUE_SIZE);
    const VECTOR_SIZE: usize = 10;

    let publisher_handle = task::spawn(publish(data_tx, VECTOR_SIZE));
    let consumer_handle = task::spawn(consume(data_rx));

    try_join!(publisher_handle, consumer_handle)?;

    Ok(())
}
