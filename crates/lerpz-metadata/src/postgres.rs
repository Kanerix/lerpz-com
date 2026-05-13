use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    MetadataClient, MetadataKind,
    error::{Error, Result},
    models::{AnalysisMetadata, GeneralMetadata, GenerationMetadata, Metadata, StorageMetadata},
};

#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "storage_provider", rename_all = "lowercase")]
enum StorageProvider {
    S3,
    Abs,
}

#[derive(sqlx::FromRow)]
struct ChatRow {
    id: Uuid,
    prompt: String,
    model: String,
    title: Option<String>,
    tags: Option<Vec<String>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct ImageRow {
    id: Uuid,
    prompt: String,
    model: String,
    title: Option<String>,
    tags: Option<Vec<String>>,
    storage_provider: StorageProvider,
    storage_bucket: String,
    storage_key: String,
    width: i32,
    height: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct VideoRow {
    id: Uuid,
    prompt: String,
    model: String,
    title: Option<String>,
    tags: Option<Vec<String>>,
    storage_provider: StorageProvider,
    storage_bucket: String,
    storage_key: String,
    width: i32,
    height: i32,
    duration: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
struct AudioRow {
    id: Uuid,
    prompt: String,
    model: String,
    title: Option<String>,
    tags: Option<Vec<String>>,
    storage_provider: StorageProvider,
    storage_bucket: String,
    storage_key: String,
    duration: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

fn general(id: Uuid, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> GeneralMetadata {
    GeneralMetadata {
        id: Some(id),
        created_at,
        updated_at,
    }
}

fn to_analysis(title: Option<String>, tags: Option<Vec<String>>) -> Option<AnalysisMetadata> {
    title.map(|title| AnalysisMetadata {
        title,
        tags: tags.unwrap_or_default(),
    })
}

fn from_analysis(analysis: &Option<AnalysisMetadata>) -> (Option<String>, Option<Vec<String>>) {
    match analysis {
        Some(a) => (Some(a.title.clone()), Some(a.tags.clone())),
        None => (None, None),
    }
}

fn to_storage(provider: StorageProvider, bucket: String, key: String) -> StorageMetadata {
    match provider {
        StorageProvider::S3 => StorageMetadata::S3 { bucket, key },
        StorageProvider::Abs => StorageMetadata::ABS {
            container: bucket,
            blob: key,
        },
    }
}

fn from_storage(storage: &StorageMetadata) -> (StorageProvider, &str, &str) {
    match storage {
        StorageMetadata::S3 { bucket, key } => (StorageProvider::S3, bucket, key),
        StorageMetadata::ABS { container, blob } => (StorageProvider::Abs, container, blob),
    }
}

impl From<ChatRow> for Metadata {
    fn from(r: ChatRow) -> Self {
        Metadata::Chat {
            general: general(r.id, r.created_at, r.updated_at),
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: to_analysis(r.title, r.tags),
        }
    }
}

impl From<ImageRow> for Metadata {
    fn from(r: ImageRow) -> Self {
        Metadata::Image {
            general: general(r.id, r.created_at, r.updated_at),
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: to_analysis(r.title, r.tags),
            storage: to_storage(r.storage_provider, r.storage_bucket, r.storage_key),
            width: r.width as u32,
            height: r.height as u32,
        }
    }
}

impl From<VideoRow> for Metadata {
    fn from(r: VideoRow) -> Self {
        Metadata::Video {
            general: general(r.id, r.created_at, r.updated_at),
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: to_analysis(r.title, r.tags),
            storage: to_storage(r.storage_provider, r.storage_bucket, r.storage_key),
            width: r.width as u32,
            height: r.height as u32,
            duration: r.duration as u32,
        }
    }
}

impl From<AudioRow> for Metadata {
    fn from(r: AudioRow) -> Self {
        Metadata::Audio {
            general: general(r.id, r.created_at, r.updated_at),
            generation: GenerationMetadata {
                prompt: r.prompt,
                model: r.model,
            },
            analysis: to_analysis(r.title, r.tags),
            storage: to_storage(r.storage_provider, r.storage_bucket, r.storage_key),
            duration: r.duration as u32,
        }
    }
}

/// A metadata client backed by a PostgreSQL connection pool.
pub struct Client {
    pool: PgPool,
}

impl Client {
    /// Connect to the database at `uri` and return a new [`Client`].
    pub async fn new(uri: &str) -> Result<Self> {
        let pool = PgPool::connect(uri).await?;
        Ok(Self { pool })
    }

    /// Create a [`Client`] from an existing [`PgPool`].
    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl MetadataClient for Client {
    async fn get(&self, id: Uuid, kind: MetadataKind) -> Result<Metadata> {
        match kind {
            MetadataKind::Chat => {
                sqlx::query_as::<_, ChatRow>("SELECT * FROM chat_metadata WHERE id = $1")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                    .map(Metadata::from)
                    .ok_or(Error::NotFound(id))
            }

            MetadataKind::Image => {
                sqlx::query_as::<_, ImageRow>("SELECT * FROM image_metadata WHERE id = $1")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                    .map(Metadata::from)
                    .ok_or(Error::NotFound(id))
            }

            MetadataKind::Video => {
                sqlx::query_as::<_, VideoRow>("SELECT * FROM video_metadata WHERE id = $1")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                    .map(Metadata::from)
                    .ok_or(Error::NotFound(id))
            }

            MetadataKind::Audio => {
                sqlx::query_as::<_, AudioRow>("SELECT * FROM audio_metadata WHERE id = $1")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                    .map(Metadata::from)
                    .ok_or(Error::NotFound(id))
            }
        }
    }

    async fn insert(&self, metadata: Metadata) -> Result<Uuid> {
        match metadata {
            Metadata::Chat {
                generation,
                analysis,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (id,) = sqlx::query_as::<_, (Uuid,)>(
                    "INSERT INTO chat_metadata (prompt, model, title, tags)
                     VALUES ($1, $2, $3, $4)
                     RETURNING id",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .fetch_one(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Image {
                generation,
                analysis,
                storage,
                width,
                height,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (provider, bucket, key) = from_storage(&storage);
                let (id,) = sqlx::query_as::<_, (Uuid,)>(
                    "INSERT INTO image_metadata
                         (prompt, model, title, tags,
                          storage_provider, storage_bucket, storage_key,
                          width, height)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                     RETURNING id",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(provider)
                .bind(bucket)
                .bind(key)
                .bind(width as i32)
                .bind(height as i32)
                .fetch_one(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Video {
                generation,
                analysis,
                storage,
                width,
                height,
                duration,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (provider, bucket, key) = from_storage(&storage);
                let (id,) = sqlx::query_as::<_, (Uuid,)>(
                    "INSERT INTO video_metadata
                         (prompt, model, title, tags,
                          storage_provider, storage_bucket, storage_key,
                          width, height, duration)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                     RETURNING id",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(provider)
                .bind(bucket)
                .bind(key)
                .bind(width as i32)
                .bind(height as i32)
                .bind(duration as i32)
                .fetch_one(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Audio {
                generation,
                analysis,
                storage,
                duration,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (provider, bucket, key) = from_storage(&storage);
                let (id,) = sqlx::query_as::<_, (Uuid,)>(
                    "INSERT INTO audio_metadata
                         (prompt, model, title, tags,
                          storage_provider, storage_bucket, storage_key,
                          duration)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                     RETURNING id",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(provider)
                .bind(bucket)
                .bind(key)
                .bind(duration as i32)
                .fetch_one(&self.pool)
                .await?;
                Ok(id)
            }
        }
    }

    async fn update(&self, id: Uuid, updates: Metadata) -> Result<Uuid> {
        match updates {
            Metadata::Chat {
                generation,
                analysis,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                sqlx::query(
                    "UPDATE chat_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4
                     WHERE id = $5",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(id)
                .execute(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Image {
                generation,
                analysis,
                storage,
                width,
                height,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (provider, bucket, key) = from_storage(&storage);
                sqlx::query(
                    "UPDATE image_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4,
                         storage_provider = $5, storage_bucket = $6, storage_key = $7,
                         width = $8, height = $9
                     WHERE id = $10",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(provider)
                .bind(bucket)
                .bind(key)
                .bind(width as i32)
                .bind(height as i32)
                .bind(id)
                .execute(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Video {
                generation,
                analysis,
                storage,
                width,
                height,
                duration,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (provider, bucket, key) = from_storage(&storage);
                sqlx::query(
                    "UPDATE video_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4,
                         storage_provider = $5, storage_bucket = $6, storage_key = $7,
                         width = $8, height = $9, duration = $10
                     WHERE id = $11",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(provider)
                .bind(bucket)
                .bind(key)
                .bind(width as i32)
                .bind(height as i32)
                .bind(duration as i32)
                .bind(id)
                .execute(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Audio {
                generation,
                analysis,
                storage,
                duration,
                ..
            } => {
                let (title, tags) = from_analysis(&analysis);
                let (provider, bucket, key) = from_storage(&storage);
                sqlx::query(
                    "UPDATE audio_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4,
                         storage_provider = $5, storage_bucket = $6, storage_key = $7,
                         duration = $8
                     WHERE id = $9",
                )
                .bind(&generation.prompt)
                .bind(&generation.model)
                .bind(title)
                .bind(tags)
                .bind(provider)
                .bind(bucket)
                .bind(key)
                .bind(duration as i32)
                .bind(id)
                .execute(&self.pool)
                .await?;
                Ok(id)
            }
        }
    }

    async fn delete(&self, id: Uuid, kind: MetadataKind) -> Result<Metadata> {
        match kind {
            MetadataKind::Chat => {
                sqlx::query_as::<_, ChatRow>("DELETE FROM chat_metadata WHERE id = $1 RETURNING *")
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await?
                    .map(Metadata::from)
                    .ok_or(Error::NotFound(id))
            }

            MetadataKind::Image => sqlx::query_as::<_, ImageRow>(
                "DELETE FROM image_metadata WHERE id = $1 RETURNING *",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),

            MetadataKind::Video => sqlx::query_as::<_, VideoRow>(
                "DELETE FROM video_metadata WHERE id = $1 RETURNING *",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),

            MetadataKind::Audio => sqlx::query_as::<_, AudioRow>(
                "DELETE FROM audio_metadata WHERE id = $1 RETURNING *",
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),
        }
    }
}
