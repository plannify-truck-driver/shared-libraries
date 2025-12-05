use crate::{S3Client, S3Error, types::ObjectMetadata};

impl S3Client {
    pub async fn download(
        &self,
        key: String,
    ) -> Result<ObjectMetadata, S3Error> {
        let key = key.into();
        let bucket = &self.config.bucket_name;

        let request = self
            .inner
            .get_object()
            .bucket(bucket)
            .key(&key);

        let result = request.send().await.map_err(|e| S3Error::from(aws_sdk_s3::Error::from(e)))?;

        Ok(ObjectMetadata {
            key,
            size: result.content_length().unwrap_or(0) as u64,
            e_tag: result.e_tag.unwrap_or_default(),
        })
    }
}
