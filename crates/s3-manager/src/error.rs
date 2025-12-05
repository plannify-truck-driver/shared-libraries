use thiserror::Error;
use aws_sdk_s3::Error as AwsError;

#[derive(Error, Debug)]
pub enum S3Error {
    #[error("Erreur AWS: {0}")]
    AwsError(#[from] AwsError),

    #[error("Configuration invalide: {0}")]
    InvalidConfig(String),

    #[error("Échec de l'upload: {0}")]
    UploadFailed(String),
}
