// use crate::source::{Source, SourceDumper};
use crate::storage::{Storage, S3};
use std::sync::Arc;
use crate::source::{Mysql, Source};

mod source;
mod storage;
mod workflow;

// struct Exporter {
//     // source: Source,
//     storage: Storage,
// }

// impl Exporter {
//     async fn export(&self) {
//         self.storage.write(self.source.dump()).await;
//     }
// }

#[tokio::main]
async fn main() {
    let mysql_source_dumper = Mysql::new("localhost", 3306, "root", "mysql123", "exportable");

    let mut mysql_source = Source::new("my_sql_source", Arc::new(mysql_source_dumper));

    let s3_driver = S3::new(
        "1SJFWjxLE52SkM7xpO7E".to_string(),
        "yzCXh5rH7NFPuQZKf1zVkER8TDiTNhKgE8dr4ehO".to_string(),
        "test".to_string(),
        "us-east-1".to_string(),
        Some("http://localhost:9000")
    );

    let storage = Storage::new("storage1".to_string(), Arc::new(s3_driver));

    let content = mysql_source.dump();

    storage.write(content).await;
}
