use aws_sdk_s3::{Client, primitives::ByteStream};

use crate::{
    Metadata,
    error::{Error, Result},
    models::StorageMetadata,
};

/// Saves the given bytes to S3 using the provided metadata.
///
/// Returns an error if the metadata has an storage type which is not
/// [`StorageMetadata::S3`] or if the save operation fails.
pub async fn save_to_s3(s3: &Client, metadata: &Metadata, bytes: &[u8]) -> Result<()> {
    let (content_type, bucket, key) = match metadata {
        Metadata::Image {
            storage, format, ..
        } => {
            let (bucket, key) = match storage {
                StorageMetadata::S3 { bucket, key } => (bucket, key),
                StorageMetadata::ABS { .. } => {
                    return Err(Error::InvalidMetadata(
                        "image metadata has wrong storage type".to_string(),
                    ));
                }
            };
            (format!("image/{format}"), bucket, key)
        }
        Metadata::Video {
            storage, format, ..
        } => {
            let (bucket, key) = match storage {
                StorageMetadata::S3 { bucket, key } => (bucket, key),
                StorageMetadata::ABS { .. } => {
                    return Err(Error::InvalidMetadata(
                        "video metadata has wrong storage type".to_string(),
                    ));
                }
            };
            (format!("video/{format}"), bucket, key)
        }
        Metadata::Audio {
            storage, format, ..
        } => {
            let (bucket, key) = match storage {
                StorageMetadata::S3 { bucket, key } => (bucket, key),
                StorageMetadata::ABS { .. } => {
                    return Err(Error::InvalidMetadata(
                        "audio metadata has wrong storage type".to_string(),
                    ));
                }
            };
            (format!("audio/{format}"), bucket, key)
        }
    };

    let bytes = ByteStream::from(bytes.to_vec());

    s3.put_object()
        .bucket(bucket)
        .key(key)
        .content_type(content_type)
        .body(bytes)
        .send()
        .await
        .map_err(|e| Error::S3(e.into()))?;

    Ok(())
}

/// Saves the given bytes to Azure Blob Storage using the provided metadata.
///
/// Returns an error if the metadata has an storage type which is not
/// [`StorageMetadata::ABS`] or if the save operation fails.
pub async fn save_to_abs(_metadata: &Metadata, _client: &Client) {
    todo!()
}
