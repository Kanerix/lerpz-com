use aws_sdk_s3::{Client, primitives::ByteStream};

use crate::{
    Metadata, StorageLocation, StorageLocationSource,
    error::{Error, Result},
    models::StorageMetadata,
};

/// Build a public URL for an S3 or Azure Blob object.
///
/// The stored object must be publicly accessible at `endpoint`. S3 uses a
/// path-style URL; Azure Blob uses the container as a subdomain.
pub fn public_url(record: &impl StorageLocationSource, endpoint: &str) -> Result<String> {
    match record.storage_location() {
        StorageLocation::S3 { bucket, key } => Ok(format!(
            "{}/{}/{}",
            endpoint.trim_end_matches('/'),
            bucket,
            key,
        )),
        StorageLocation::AzureBlob { container, blob } => {
            let host = endpoint
                .trim_end_matches('/')
                .trim_start_matches("https://")
                .trim_start_matches("http://");
            Ok(format!("https://{container}.{host}/{blob}"))
        }
    }
}

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
                StorageMetadata::AzureBlob { .. } => {
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
                StorageMetadata::AzureBlob { .. } => {
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
                StorageMetadata::AzureBlob { .. } => {
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
        .map_err(|e| Error::S3(Box::new(e.into())))?;

    Ok(())
}

/// Saves the given bytes to Azure Blob Storage using the provided metadata.
///
/// Returns an error if the metadata has an storage type which is not
/// [`StorageMetadata::AzureBlob`] or if the save operation fails.
pub async fn save_to_abs(_metadata: &Metadata, _client: &Client) {
    todo!()
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use uuid::Uuid;

    use super::*;
    use crate::models::{GeneralMetadata, GenerationMetadata};

    #[test]
    fn public_urls_for_all_metadata_kinds() {
        let storage = || StorageMetadata::S3 {
            bucket: "media".to_string(),
            key: "file.mp4".to_string(),
        };
        let general = || GeneralMetadata {
            id: Uuid::nil(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let generation = || GenerationMetadata {
            prompt: String::new(),
            model: String::new(),
        };
        let records = [
            Metadata::Image {
                general: general(),
                generation: generation(),
                storage: storage(),
                analysis: None,
                format: "png".to_string(),
                width: 1,
                height: 1,
            },
            Metadata::Video {
                general: general(),
                generation: generation(),
                storage: storage(),
                analysis: None,
                format: "mp4".to_string(),
                width: 1,
                height: 1,
                duration: 1,
            },
            Metadata::Audio {
                general: general(),
                generation: generation(),
                storage: storage(),
                analysis: None,
                format: "mp3".to_string(),
                duration: 1,
            },
        ];

        for record in &records {
            assert_eq!(
                public_url(record, "https://cdn.example.com///").expect("S3 URL"),
                "https://cdn.example.com/media/file.mp4"
            );
        }
        assert_eq!(
            public_url(&("media", "file.mp4"), "https://cdn.example.com/").expect("S3 URL"),
            "https://cdn.example.com/media/file.mp4"
        );
    }

    #[test]
    fn azure_blob_storage_uses_container_subdomain() {
        let storage = StorageMetadata::AzureBlob {
            container: "media".to_string(),
            blob: "file.mp4".to_string(),
        };
        assert_eq!(
            storage.storage_location(),
            StorageLocation::AzureBlob {
                container: "media",
                blob: "file.mp4",
            }
        );
        assert_eq!(
            public_url(&storage, "https://cdn.example.com/").expect("Azure Blob URL"),
            "https://media.cdn.example.com/file.mp4"
        );
        assert_eq!(
            public_url(&storage, "http://cdn.example.com/").expect("Azure Blob URL"),
            "https://media.cdn.example.com/file.mp4"
        );
    }
}
