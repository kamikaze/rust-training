use async_channel::{bounded, Receiver, Sender};
use async_compression::tokio::bufread::BzEncoder;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use http_body::{Body, Frame};
use std::pin::Pin;
use std::task::{Context, Poll};
use tar::Builder;
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

        archive_tx
            .send(chunk)
            .await
            .expect("Failed to send a chunk");
    }
}

async fn archive(archive_rx: Receiver<Vec<u8>>, compress_tx: Sender<u8>) {
    while let Ok(chunk) = archive_rx.recv().await {
        println!("Archiver received: {:?}", chunk);

        for byte in chunk {
            compress_tx
                .send(byte)
                .await
                .expect("Failed to send a chunk");
        }
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


async fn compress(compress_rx: Receiver<u8>) {
    let mut tar_buffer = Vec::new();
    let mut builder = Builder::new(&mut tar_buffer);
    let bz_encoder = BzEncoder::new(compress_rx);

    // let my_stream_body = MyStreamBody::new(compress_rx);
    // let byte_stream = ByteStream::from(compress_rx);
}


#[derive(Debug)]
pub struct ArchiveStream {
    vector_size: usize,
    max_queue_size: usize,
}

impl ArchiveStream {
    pub fn new(vector_size: usize, max_queue_size: usize) -> Self {
        ArchiveStream {
            vector_size,
            max_queue_size,
        }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let (data_tx, data_rx) = bounded(self.max_queue_size);
        let (archive_tx, archive_rx) = bounded(self.max_queue_size);
        let (compress_tx, compress_rx) = bounded(self.max_queue_size);

        let publisher_handle = task::spawn(publish(data_tx, self.vector_size));
        let consumer_handle = task::spawn(consume(data_rx, archive_tx));
        let archive_handle = task::spawn(archive(archive_rx, compress_tx));
        let compress_handle = task::spawn(compress(compress_rx));

        try_join!(
            publisher_handle,
            consumer_handle,
            archive_handle,
            compress_handle
        )?;

        Ok(())
    }
}

impl Body for ArchiveStream {
    type Data = Bytes;
    type Error = std::io::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        todo!()
    }
}


pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    const MAX_QUEUE_SIZE: usize = 10;
    const VECTOR_SIZE: usize = 10;

    Ok(())
}
