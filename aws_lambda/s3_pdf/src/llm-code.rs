use aws_sdk_s3::Client;
use aws_sdk_s3::types::SdkError;
use aws_sdk_s3::error::GetObjectError;
use aws_sdk_s3::model::ObjectCannedAcl;
use pdf::file::File as PdfFile;
use pdf::object::*;
use pdf::writer::*;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use uuid::Uuid;
use urlencoding::decode;

// use aws_sdk_s3 as s3;

// #[::tokio::main]
// async fn main() -> Result<(), s3::Error> {
//     let config = aws_config::load_from_env().await;
//     let client = aws_sdk_s3::Client::new(&config);

       // ... make some calls with the client

//     Ok(())
// }

#[tokio::main]
async fn main() -> Result<(), SdkError<GetObjectError>> {
    let shared_config = aws_config::load_from_env().await;
    let s3_client = Client::new(&shared_config);

    // Simulate an event
    let event = get_mock_event();

    for record in event.records {
        let bucket = record.s3.bucket.name;
        let key = decode(&record.s3.object.key).unwrap().into_owned();
        let download_path = format!("/tmp/{}.pdf", Uuid::new_v4());
        let upload_path = format!("/tmp/converted-{}.pdf", Uuid::new_v4());

        if key.to_lowercase().ends_with(".pdf") {
            download_file(&s3_client, &bucket, &key, &download_path).await?;
            encrypt_pdf(&download_path, &upload_path)?;
            let encrypted_key = add_encrypted_suffix(&key);
            upload_file(&s3_client, &bucket, &encrypted_key, &upload_path).await?;
        }
    }
    Ok(())
}

async fn download_file(client: &Client, bucket: &str, key: &str, download_path: &str) -> Result<(), SdkError<GetObjectError>> {
    let resp = client.get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let body = resp.body.collect().await?;
    fs::write(download_path, &body.into_bytes())?;

    Ok(())
}

fn encrypt_pdf(file_path: &str, encrypted_file_path: &str) -> io::Result<()> {
    let pdf_file = PdfFile::<Vec<u8>>::open(file_path)?;
    let mut writer = PdfWriter::new(File::create(encrypted_file_path)?);

    for page in pdf_file.pages() {
        writer.add_page(page);
    }

    // Add encryption (this is a placeholder, implement encryption as needed)
    writer.finish()?;

    Ok(())
}

fn add_encrypted_suffix(original_key: &str) -> String {
    let mut parts: Vec<&str> = original_key.split('.').collect();
    if parts.len() > 1 {
        let extension = parts.pop().unwrap();
        format!("{}_encrypted.{}", parts.join("."), extension)
    } else {
        format!("{}_encrypted", original_key)
    }
}

async fn upload_file(client: &Client, bucket: &str, key: &str, file_path: &str) -> Result<(), SdkError<GetObjectError>> {
    let body = fs::read(file_path)?;
    client.put_object()
        .bucket(format!("{}-encrypted", bucket))
        .key(key)
        .body(body.into())
        .acl(ObjectCannedAcl::Private)
        .send()
        .await?;

    Ok(())
}

// Mock function to simulate an event
fn get_mock_event() -> Event {
    Event {
        records: vec![
            Record {
                s3: S3 {
                    bucket: Bucket { name: "example-bucket".to_string() },
                    object: Object { key: "example.pdf".to_string() },
                }
            }
        ]
    }
}

// Event, Record, S3, Bucket, and Object structs for mocking
struct Event {
    records: Vec<Record>,
}

struct Record {
    s3: S3,
}

struct S3 {
    bucket: Bucket,
    object: Object,
}

struct Bucket {
    name: String,
}

struct Object {
    key: String,
}