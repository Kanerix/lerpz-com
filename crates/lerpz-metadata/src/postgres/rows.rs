use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    StorageLocation, StorageLocationSource,
    models::{AnalysisMetadata, GeneralMetadata, GenerationMetadata, Metadata, StorageMetadata},
};

use super::StorageProvider;

#[derive(sqlx::FromRow)]
pub struct ImageRow {
    pub id: Uuid,
    pub prompt: String,
    pub model: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub storage_provider: StorageProvider,
    pub storage_bucket: String,
    pub storage_key: String,
    pub format: String,
    pub width: i32,
    pub height: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct VideoRow {
    pub id: Uuid,
    pub prompt: String,
    pub model: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub storage_provider: StorageProvider,
    pub storage_bucket: String,
    pub storage_key: String,
    pub format: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct AudioRow {
    pub id: Uuid,
    pub prompt: String,
    pub model: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub storage_provider: StorageProvider,
    pub storage_bucket: String,
    pub storage_key: String,
    pub format: String,
    pub duration: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn storage_location<'a>(
    provider: &StorageProvider,
    bucket: &'a str,
    key: &'a str,
) -> StorageLocation<'a> {
    match provider {
        StorageProvider::S3 => StorageLocation::S3 { bucket, key },
        StorageProvider::AzureBlob => StorageLocation::AzureBlob {
            container: bucket,
            blob: key,
        },
    }
}

impl StorageLocationSource for ImageRow {
    fn storage_location(&self) -> StorageLocation<'_> {
        storage_location(
            &self.storage_provider,
            &self.storage_bucket,
            &self.storage_key,
        )
    }
}

impl StorageLocationSource for VideoRow {
    fn storage_location(&self) -> StorageLocation<'_> {
        storage_location(
            &self.storage_provider,
            &self.storage_bucket,
            &self.storage_key,
        )
    }
}

impl StorageLocationSource for AudioRow {
    fn storage_location(&self) -> StorageLocation<'_> {
        storage_location(
            &self.storage_provider,
            &self.storage_bucket,
            &self.storage_key,
        )
    }
}

impl From<ImageRow> for Metadata {
    fn from(r: ImageRow) -> Self {
        Metadata::Image {
            general: GeneralMetadata {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: r.title.map(|title| AnalysisMetadata {
                title,
                tags: r.tags.unwrap_or_default(),
            }),
            storage: match r.storage_provider {
                StorageProvider::S3 => StorageMetadata::S3 {
                    bucket: r.storage_bucket,
                    key: r.storage_key,
                },
                StorageProvider::AzureBlob => todo!(),
            },
            format: r.format,
            width: r.width as u32,
            height: r.height as u32,
        }
    }
}

impl From<VideoRow> for Metadata {
    fn from(r: VideoRow) -> Self {
        Metadata::Video {
            general: GeneralMetadata {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: r.title.map(|title| AnalysisMetadata {
                title,
                tags: r.tags.unwrap_or_default(),
            }),
            storage: match r.storage_provider {
                StorageProvider::S3 => StorageMetadata::S3 {
                    bucket: r.storage_bucket,
                    key: r.storage_key,
                },
                StorageProvider::AzureBlob => todo!(),
            },
            format: r.format,
            width: r.width as u32,
            height: r.height as u32,
            duration: r.duration as u32,
        }
    }
}

impl From<AudioRow> for Metadata {
    fn from(r: AudioRow) -> Self {
        Metadata::Audio {
            general: GeneralMetadata {
                id: r.id,
                created_at: r.created_at,
                updated_at: r.updated_at,
            },
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: r.title.map(|title| AnalysisMetadata {
                title,
                tags: r.tags.unwrap_or_default(),
            }),
            storage: match r.storage_provider {
                StorageProvider::S3 => StorageMetadata::S3 {
                    bucket: r.storage_bucket,
                    key: r.storage_key,
                },
                StorageProvider::AzureBlob => todo!(),
            },
            format: r.format,
            duration: r.duration as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_location_preserves_provider() {
        assert_eq!(
            storage_location(&StorageProvider::S3, "media", "file.mp4"),
            StorageLocation::S3 {
                bucket: "media",
                key: "file.mp4",
            }
        );
        assert_eq!(
            storage_location(&StorageProvider::AzureBlob, "media", "file.mp4"),
            StorageLocation::AzureBlob {
                container: "media",
                blob: "file.mp4",
            }
        );
    }
}
