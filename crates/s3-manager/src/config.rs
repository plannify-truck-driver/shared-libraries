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
        endpoint: impl Into<String>,
        bucket: impl Into<String>,
        region: Option<impl Into<String>>,
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            bucket_name: bucket.into(),
            region: region.map(|r| r.into()),
            access_key: access_key.into(),
            secret_key: secret_key.into(),
        }
    }
}
