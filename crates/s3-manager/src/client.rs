use crate::{config::S3Config, error::S3Error};
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;

#[derive(Debug, Clone)]
pub struct S3Client {
    pub inner: Client,
    pub config: S3Config,
}

impl S3Client {
    pub async fn new(config: S3Config) -> Result<Self, S3Error> {
        let region_provider =
            RegionProviderChain::default_provider().or_else(Region::new(config.region.clone()));

        let sdk_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(region_provider)
            .load()
            .await;

        let client = Client::new(&sdk_config);
        Ok(Self {
            inner: client,
            config,
        })
    }
}
