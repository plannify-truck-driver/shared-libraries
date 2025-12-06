use crate::{S3Client, S3Error, types::ObjectMetadata};

impl S3Client {
    pub async fn list_objects(&self, key: String) -> Result<Vec<ObjectMetadata>, S3Error> {
        let request = self
            .inner
            .list_objects_v2()
            .bucket(&self.config.bucket_name)
            .prefix(&key);

        let result = request
            .send()
            .await
            .map_err(|e| S3Error::from(aws_sdk_s3::Error::from(e)))?;

        let objects = result
            .contents()
            .iter()
            .map(|obj| ObjectMetadata {
                key: obj.key().unwrap_or_default().to_string(),
                size: obj.size().unwrap_or(0) as u64,
                e_tag: obj.e_tag().unwrap_or_default().to_string(),
            })
            .collect();

        Ok(objects)
    }
}
