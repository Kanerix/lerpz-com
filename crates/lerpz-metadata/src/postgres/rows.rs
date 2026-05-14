use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{
    AnalysisMetadata, GeneralMetadata, GenerationMetadata, Metadata, StorageMetadata,
};

use super::StorageProvider;

#[derive(sqlx::FromRow)]
pub(super) struct ImageRow {
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
pub(super) struct VideoRow {
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
pub(super) struct AudioRow {
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
                StorageProvider::Abs => todo!(),
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
                StorageProvider::Abs => todo!(),
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
                StorageProvider::Abs => todo!(),
            },
            format: r.format,
            duration: r.duration as u32,
        }
    }
}
