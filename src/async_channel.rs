use async_channel::{bounded, Receiver, Sender};
use async_compression::tokio::bufread::BzEncoder;
use async_compression::Level;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Error;
use aws_sdk_s3::primitives::{ByteStream, ByteStreamError, DateTime};
use aws_sdk_s3::types::{CompletedMultipartUpload, CompletedPart};
use aws_sdk_s3::{Client, Config};
use aws_types::region::Region;
use std::error::Error;
use std::io::Write;
use std::sync::Arc;
use std::{env, fmt};
use std::time::SystemTime;
use tar::Builder;
use tokio::{task, try_join};

#[derive(Debug)]
pub enum S3Error {
    ListObjectsError(SdkError<ListObjectsV2Error>),
    GetObjectError(SdkError<GetObjectError>),
    ByteStreamError(ByteStreamError),
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S3Error::ListObjectsError(e) => write!(f, "ListObjectsV2 error: {}", e),
            S3Error::GetObjectError(e) => write!(f, "GetObject error: {}", e),
            S3Error::ByteStreamError(e) => write!(f, "ByteStream error: {}", e),
        }
    }
}

impl Error for S3Error {}

struct S3Params {
    region: String,
    access_key: String,
    secret_key: String,
    endpoint: String,
}

fn get_s3_params() -> S3Params {
    let region = env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let endpoint =
        env::var("OBJECT_STORAGE_ENDPOINT").expect("OBJECT_STORAGE_ENDPOINT must be set");
    let access_key = env::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY must be set");
    let secret_key = env::var("AWS_SECRET_KEY").expect("AWS_SECRET_KEY must be set");

    let params = S3Params {
        region,
        access_key,
        secret_key,
        endpoint,
    };

    params
}

fn get_client(params: &S3Params) -> Client {
    let config = Config::builder()
        .region(Region::new(params.region.clone()))
        .endpoint_url(&params.endpoint)
        .credentials_provider(Credentials::new(
            &params.access_key,
            &params.secret_key,
            None,
            None,
            "static",
        ))
        .build();

    let client = Client::from_conf(config);

    client
}

#[derive(Debug, Clone)]
struct ObjectChunk {
    idx: u64,
    object_key: String,
    object_size: i64,
    last_modified: DateTime,
    data: Arc<[u8]>,
    is_last: bool,
}

async fn publish(data_tx: Sender<Arc<ObjectChunk>>, vector_size: usize) {
    let object_key: String = String::from("/path/to/file.txt");
    let object_size: i64 = 12345;
    let last_modified = DateTime::from(SystemTime::now());

    for idx in 0..100 {
        let data: Arc<[u8]> = Arc::from(vec![0u8; vector_size]);
        let object_chunk: ObjectChunk = ObjectChunk {
            idx,
            object_key: object_key.clone(),
            object_size,
            last_modified,
            data,
            is_last: idx == 99,
        };
        let chunk: Arc<ObjectChunk> = Arc::from(object_chunk);

        data_tx
            .send(chunk)
            .await
            .expect("Failed to send a vector to data_tx");
    }
}

async fn consume(data_rx: Receiver<Arc<ObjectChunk>>, archive_tx: Sender<Arc<ObjectChunk>>) {
    while let Ok(chunk) = data_rx.recv().await {
        println!("Consumer received: {:?}", chunk);

        archive_tx
            .send(chunk)
            .await
            .expect("Failed to send a chunk");
    }
}

async fn archive(archive_rx: Receiver<Arc<ObjectChunk>>, compress_tx: Sender<u8>) {
    const MAX_BUFFER_SIZE: usize = 10 * 1024 * 1024;
    let mut tar_buffer = Vec::new();
    let mut builder = Builder::new(&mut tar_buffer);

    while let Ok(chunk) = archive_rx.recv().await {
        println!("Archiver received: {:?}", chunk);
        let data = chunk.data;

        let mut writer = builder
            .append_writer()
            .expect("Failed to append a writer to tar");

        writer
            .write(data.as_ref())
            .expect("Failed to write a chunk to tar");

        while tar_buffer.len() >= MAX_BUFFER_SIZE {
            let tar_chunk: Vec<u8> = tar_buffer.drain(..MAX_BUFFER_SIZE).collect();

            for tar_byte in tar_chunk {
                compress_tx
                    .send(tar_byte)
                    .await
                    .expect("Failed to send a chunk");
            }
        }
    }

    builder.finish();
}

async fn compress(compress_rx: Receiver<u8>, store_tx: Sender<Vec<u8>>) {
    let bz_encoder = BzEncoder::with_quality(compress_rx, Level::Best);

    // let my_stream_body = MyStreamBody::new(compress_rx);
    // let byte_stream = ByteStream::from(compress_rx);
}

async fn store(store_rx: Receiver<Vec<u8>>, bucket: &str, object_key: &str) {
    let s3_params = get_s3_params();
    let client = get_client(&s3_params);

    let create_multipart_upload_response = client
        .create_multipart_upload()
        .bucket(bucket)
        .key(object_key)
        .send()
        .await
        .expect("Failed to create a multipart upload");

    let upload_id = match create_multipart_upload_response.upload_id() {
        Some(upload_id) => upload_id.to_owned(),
        None => panic!("Something went wrong when creating multipart upload."),
    };

    let mut upload_parts: Vec<CompletedPart> = Vec::new();
    let mut part_number = 0i32;

    while let Ok(chunk) = store_rx.recv().await {
        let body = ByteStream::from(chunk);

        let upload_part_response = client
            .upload_part()
            .bucket(bucket)
            .key(object_key)
            .upload_id(&upload_id)
            .part_number(1)
            .body(body)
            .send()
            .await
            .expect("Failed to upload a part.");

        part_number += 1;

        upload_parts.push(
            CompletedPart::builder()
                .e_tag(upload_part_response.e_tag.unwrap_or_default())
                .part_number(part_number)
                .build(),
        );
    }

    let completed_multipart_upload: CompletedMultipartUpload = CompletedMultipartUpload::builder()
        .set_parts(Some(upload_parts))
        .build();

    let _complete_multipart_upload_res = client
        .complete_multipart_upload()
        .bucket(bucket)
        .key(object_key)
        .multipart_upload(completed_multipart_upload)
        .upload_id(upload_id)
        .send()
        .await
        .expect("Failed to complete multipart upload");
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

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        let dst_bucket = "static";
        let dst_object_key = "archive.tar.bz2";

        let (data_tx, data_rx) = bounded(self.max_queue_size);
        let (archive_tx, archive_rx) = bounded(self.max_queue_size);
        let (compress_tx, compress_rx) = bounded(self.max_queue_size);
        let (store_tx, store_rx) = bounded(self.max_queue_size);

        let publisher_handle = task::spawn(publish(data_tx, self.vector_size));
        let consumer_handle = task::spawn(consume(data_rx, archive_tx));
        let archive_handle = task::spawn(archive(archive_rx, compress_tx));
        let compress_handle = task::spawn(compress(compress_rx, store_tx));
        let store_handle = task::spawn(store(store_rx, dst_bucket, dst_object_key));

        try_join!(
            publisher_handle,
            consumer_handle,
            archive_handle,
            compress_handle,
            store_handle
        )?;

        Ok(())
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    const MAX_QUEUE_SIZE: usize = 10;
    const VECTOR_SIZE: usize = 10;

    Ok(())
}
