use crate::{S3Client, S3Error, types::ObjectMetadata, types::ObjectType};

impl S3Client {
    pub async fn upload(
        &self,
        object_type: ObjectType,
        entity_id: &str,
        content: &[u8],
    ) -> Result<ObjectMetadata, S3Error> {
        let key = object_type.to_s3_path(entity_id);
        let bucket = &self.config.bucket_name;
        let body = aws_sdk_s3::primitives::ByteStream::from(content.to_vec());

        let request = self.inner.put_object().bucket(bucket).key(&key).body(body);

        let result = request
            .send()
            .await
            .map_err(|e| S3Error::from(aws_sdk_s3::Error::from(e)))?;

        Ok(ObjectMetadata {
            key,
            size: content.len() as u64,
            e_tag: result.e_tag.unwrap_or_default(),
        })
    }
}
