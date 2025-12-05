#[derive(Debug, Clone)]
pub struct S3Config {
    pub bucket_name: String,
    pub region: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
}

impl S3Config {
    pub fn default(bucket: impl Into<String>, region: impl Into<String>) -> Self {
        Self {
            bucket_name: bucket.into(),
            region: region.into(),
            access_key: None,
            secret_key: None,
        }
    }
}
