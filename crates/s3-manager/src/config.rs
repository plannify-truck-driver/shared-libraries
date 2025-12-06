#[derive(Debug, Clone)]
pub struct S3Config {
    pub endpoint: String,
    pub bucket_name: String,
    pub region: Option<String>,
    pub access_key: String,
    pub secret_key: String,
}

impl S3Config {
    pub fn default(
        bucket: impl Into<String>,
        region: impl Into<String>,
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Self {
        Self {
            endpoint: String::new(),
            bucket_name: bucket.into(),
            region: Some(region.into()),
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }
}
