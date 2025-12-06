use crate::{config::S3Config, error::S3Error};
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, config::Credentials};

#[derive(Debug, Clone)]
pub struct S3Client {
    pub inner: Client,
    pub config: S3Config,
}

impl S3Client {
    pub async fn new(config: S3Config) -> Result<Self, S3Error> {
        let credentials = Credentials::new(
            config.access_key.clone(),
            config.secret_key.clone(),
            None,
            None,
            "plannify",
        );

        let s3_config = aws_sdk_s3::config::Builder::new()
            .credentials_provider(credentials)
            .endpoint_url(config.endpoint.to_string())
            .behavior_version(BehaviorVersion::latest())
            .region(aws_sdk_s3::config::Region::new(
                config
                    .region
                    .clone()
                    .unwrap_or_else(|| "plannify-region".to_string()),
            ))
            .build();

        let client = aws_sdk_s3::Client::from_conf(s3_config);

        Ok(Self {
            inner: client,
            config,
        })
    }
}
