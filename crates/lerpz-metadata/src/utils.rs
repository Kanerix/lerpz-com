use aws_sdk_s3::{Client, primitives::ByteStream};

use crate::{
    Metadata,
    error::{Error, Result},
    models::StorageMetadata,
};

pub async fn save_to_s3(s3: &Client, metadata: &Metadata, bytes: &[u8]) -> Result<()> {
    let (content_type, bucket, key) = match metadata {
        Metadata::Chat { .. } => Err(Error::InvalidMetadata(
            "chat metadata cannot be saved to S3".to_string(),
        )),
        Metadata::Image { storage, .. } => {
            let (bucket, key) = match storage {
                StorageMetadata::S3 { bucket, key } => (bucket, key),
                StorageMetadata::ABS { .. } => {
                    return Err(Error::InvalidMetadata(
                        "image metadata has wrong storage type".to_string(),
                    ));
                }
            };
            Ok(("image/jpeg", bucket, key))
        }
        Metadata::Video { storage, .. } => {
            let (bucket, key) = match storage {
                StorageMetadata::S3 { bucket, key } => (bucket, key),
                StorageMetadata::ABS { .. } => {
                    return Err(Error::InvalidMetadata(
                        "video metadata has wrong storage type".to_string(),
                    ));
                }
            };
            Ok(("video/mp4", bucket, key))
        }
        Metadata::Audio { storage, .. } => {
            let (bucket, key) = match storage {
                StorageMetadata::S3 { bucket, key } => (bucket, key),
                StorageMetadata::ABS { .. } => {
                    return Err(Error::InvalidMetadata(
                        "audio metadata has wrong storage type".to_string(),
                    ));
                }
            };
            Ok(("audio/mpeg", bucket, key))
        }
    }?;

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

pub async fn save_to_abs(metadata: &Metadata, client: &Client) {
    todo!()
}
