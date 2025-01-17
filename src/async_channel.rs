use async_channel::{bounded, Receiver, Sender};
use tokio::{task, try_join};

async fn publish(data_tx: Sender<Vec<u8>>, vector_size: usize) {
    for _ in 0..100 {
        let vec = vec![0u8; vector_size];
        data_tx
            .send(vec)
            .await
            .expect("Failed to send a vector to data_tx");
    }
}

async fn consume(data_rx: Receiver<Vec<u8>>, archive_tx: Sender<Vec<u8>>) {
    while let Ok(chunk) = data_rx.recv().await {
        println!("Consumer received: {:?}", chunk);

        archive_tx.send(chunk).await.expect("Failed to send a chunk");
    }
}

async fn archive(archive_rx: Receiver<Vec<u8>>) {
    while let Ok(chunk) = archive_rx.recv().await {
        println!("Archiver received: {:?}", chunk);
    }

    // let archive_stream = receiver_to_stream(archive_rx);
    // let archive_reader = StreamReader::new(archive_stream);
    //
    // let bz_encoder = BzEncoder::new(archive_reader);
    // let bz_encoder_reader_stream = ReaderStream::new(bz_encoder);
    //
    // bz_encoder_reader_stream
    //     .for_each(|result| async {
    //         match result {
    //             Ok(bytes) => {
    //                 // Вывод байтов
    //                 println!("{:?}", bytes);
    //             }
    //             Err(e) => {
    //                 eprintln!("Error reading from the stream: {}", e);
    //             }
    //         }
    //     })
    //     .await;
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_QUEUE_SIZE: usize = 10;
    let (data_tx, data_rx) = bounded(MAX_QUEUE_SIZE);
    let (archive_tx, archive_rx) = bounded(MAX_QUEUE_SIZE);
    const VECTOR_SIZE: usize = 10;

    let publisher_handle = task::spawn(publish(data_tx, VECTOR_SIZE));
    let consumer_handle = task::spawn(consume(data_rx, archive_tx));
    let archive_handle = task::spawn(archive(archive_rx));

    try_join!(publisher_handle, consumer_handle, archive_handle)?;

    Ok(())
}
