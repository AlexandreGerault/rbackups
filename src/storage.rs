use async_trait::async_trait;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use std::sync::Arc;

pub struct S3 {
    client: Client,
    bucket: String,
}

impl S3 {
    pub fn new(
        access_key: String,
        access_secret: String,
        bucket: String,
        region: String,
        url: Option<&str>,
    ) -> S3 {
        let config = aws_sdk_s3::Config::builder()
            .region(Region::new(region))
            .endpoint_url(url.unwrap())
            .credentials_provider(Credentials::new(
                access_key,
                access_secret,
                None,
                None,
                "test",
            ))
            .force_path_style(true)
            .behavior_version(BehaviorVersion::v2024_03_28())
            .build();

        let client = Client::from_conf(config);

        S3 { client, bucket }
    }
}

// pub struct Local {
//     path: String,
// }
//
// pub struct Ftp {
//     host: String,
//     port: u16,
//     path: String,
// }

#[async_trait]
pub trait Driver {
    async fn write(&self, content: String, filename: &str);
}

#[async_trait]
impl Driver for S3 {
    async fn write(&self, content: String, filename: &str) {
        let key = filename; // Adjust the key as needed
        let body = ByteStream::from(content.into_bytes());

        let result = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body)
            .send()
            .await;

        match result {
            Ok(_) => println!("Content uploaded successfully to S3."),
            Err(e) => {
                eprintln!("Failed to upload to S3: {}", e); // Display the error
                eprintln!("Debug information: {:?}", e); // Debug output for more details
            }
        }
    }
}

// #[async_trait]
// impl Driver for Local {
//     async fn write(&self, content: String) {
//         println!("[local] {}", content);
//     }
// }
//
// #[async_trait]
// impl Driver for Ftp {
//     async fn write(&self, content: String) {
//         println!("[ftp] {}", content);
//     }
// }

pub struct Storage {
    id: String,
    driver: Arc<dyn Driver>,
}

impl Storage {
    pub fn new(id: String, driver: Arc<dyn Driver>) -> Storage {
        Storage { id, driver }
    }

    pub async fn write(&self, content: String) {
        println!("[{}] - Writing content: \"{}\"", self.id, content);
        self.driver.write(content, "backup_of_today.sql").await;
    }
}
