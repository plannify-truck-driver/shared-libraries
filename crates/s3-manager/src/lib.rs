pub mod client;
pub mod config;
pub mod error;
pub mod operations;
pub mod types;

pub use client::S3Client;
pub use error::S3Error;
pub use types::ObjectMetadata;