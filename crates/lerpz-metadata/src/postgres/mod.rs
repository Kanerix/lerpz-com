mod rows;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    MetadataClient, MetadataKind,
    error::{Error, Result},
    models::{Metadata, StorageMetadata},
};

use rows::{AudioRow, ImageRow, VideoRow};

/// Postgres-side representation of the storage provider.
///
/// Maps to the `storage_provider` enum type defined in the database schema.
/// Use [`From<&StorageMetadata>`] to convert from the domain model.
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "storage_provider", rename_all = "lowercase")]
enum StorageProvider {
    S3,
    Abs,
}

impl From<&StorageMetadata> for StorageProvider {
    fn from(s: &StorageMetadata) -> Self {
        match s {
            StorageMetadata::S3 { .. } => StorageProvider::S3,
            StorageMetadata::ABS { .. } => StorageProvider::Abs,
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
            MetadataKind::Image => sqlx::query_as!(
                ImageRow,
                r#"SELECT
                id,
                prompt,
                model,
                title,
                tags,
                storage_provider AS "storage_provider: StorageProvider",
                storage_bucket,
                storage_key,
                format,
                width,
                height,
                created_at,
                updated_at
                FROM image_metadata WHERE id = $1"#,
                id
            )
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),

            MetadataKind::Video => sqlx::query_as!(
                VideoRow,
                r#"SELECT
                id,
                prompt,
                model,
                title,
                tags,
                storage_provider AS "storage_provider: StorageProvider",
                storage_bucket,
                storage_key,
                format,
                width,
                height,
                duration,
                created_at,
                updated_at
                FROM video_metadata WHERE id = $1"#,
                id
            )
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),

            MetadataKind::Audio => sqlx::query_as!(
                AudioRow,
                r#"SELECT
                id,
                prompt,
                model,
                title,
                tags,
                storage_provider AS "storage_provider: StorageProvider",
                storage_bucket,
                storage_key,
                format,
                duration,
                created_at,
                updated_at
                FROM audio_metadata WHERE id = $1"#,
                id
            )
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),
        }
    }

    async fn insert(&self, metadata: Metadata) -> Result<Uuid> {
        match metadata {
            Metadata::Image {
                general,
                generation,
                analysis,
                storage,
                format,
                width,
                height,
                ..
            } => {
                let title = analysis.as_ref().map(|a| a.title.clone());
                let tags = analysis.as_ref().map(|a| a.tags.clone());
                let provider = StorageProvider::from(&storage);
                let (bucket, key) = match &storage {
                    StorageMetadata::S3 { bucket, key } => (bucket.as_str(), key.as_str()),
                    StorageMetadata::ABS { .. } => todo!(),
                };
                let row = sqlx::query!(
                    "INSERT INTO image_metadata
                         (id, prompt, model, title, tags,
                          storage_provider, storage_bucket, storage_key,
                          format, width, height)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                     RETURNING id",
                    general.id,
                    generation.prompt,
                    generation.model,
                    title,
                    tags.as_deref(),
                    provider as StorageProvider,
                    bucket,
                    key,
                    format,
                    width as i32,
                    height as i32,
                )
                .fetch_one(&self.pool)
                .await?;
                Ok(row.id)
            }

            Metadata::Video {
                general,
                generation,
                analysis,
                storage,
                format,
                width,
                height,
                duration,
                ..
            } => {
                let title = analysis.as_ref().map(|a| a.title.clone());
                let tags = analysis.as_ref().map(|a| a.tags.clone());
                let provider = StorageProvider::from(&storage);
                let (bucket, key) = match &storage {
                    StorageMetadata::S3 { bucket, key } => (bucket.as_str(), key.as_str()),
                    StorageMetadata::ABS { .. } => todo!(),
                };
                let row = sqlx::query!(
                    "INSERT INTO video_metadata
                         (id, prompt, model, title, tags,
                          storage_provider, storage_bucket, storage_key,
                          format, width, height, duration)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                     RETURNING id",
                    general.id,
                    generation.prompt,
                    generation.model,
                    title,
                    tags.as_deref(),
                    provider as StorageProvider,
                    bucket,
                    key,
                    format,
                    width as i32,
                    height as i32,
                    duration as i32,
                )
                .fetch_one(&self.pool)
                .await?;
                Ok(row.id)
            }

            Metadata::Audio {
                general,
                generation,
                analysis,
                storage,
                format,
                duration,
                ..
            } => {
                let title = analysis.as_ref().map(|a| a.title.clone());
                let tags = analysis.as_ref().map(|a| a.tags.clone());
                let provider = StorageProvider::from(&storage);
                let (bucket, key) = match &storage {
                    StorageMetadata::S3 { bucket, key } => (bucket.as_str(), key.as_str()),
                    StorageMetadata::ABS { .. } => todo!(),
                };
                let row = sqlx::query!(
                    "INSERT INTO audio_metadata
                         (id, prompt, model, title, tags,
                          storage_provider, storage_bucket, storage_key,
                          format, duration)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                     RETURNING id",
                    general.id,
                    generation.prompt,
                    generation.model,
                    title,
                    tags.as_deref(),
                    provider as StorageProvider,
                    bucket,
                    key,
                    format,
                    duration as i32,
                )
                .fetch_one(&self.pool)
                .await?;
                Ok(row.id)
            }
        }
    }

    async fn update(&self, id: Uuid, updates: Metadata) -> Result<Uuid> {
        match updates {
            Metadata::Image {
                generation,
                analysis,
                storage,
                format,
                width,
                height,
                ..
            } => {
                let title = analysis.as_ref().map(|a| a.title.clone());
                let tags = analysis.as_ref().map(|a| a.tags.clone());
                let provider = StorageProvider::from(&storage);
                let (bucket, key) = match &storage {
                    StorageMetadata::S3 { bucket, key } => (bucket.as_str(), key.as_str()),
                    StorageMetadata::ABS { .. } => todo!(),
                };
                sqlx::query!(
                    "UPDATE image_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4,
                         storage_provider = $5, storage_bucket = $6, storage_key = $7,
                         format = $8, width = $9, height = $10
                     WHERE id = $11",
                    generation.prompt,
                    generation.model,
                    title,
                    tags.as_deref(),
                    provider as StorageProvider,
                    bucket,
                    key,
                    format,
                    width as i32,
                    height as i32,
                    id,
                )
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
                let title = analysis.as_ref().map(|a| a.title.clone());
                let tags = analysis.as_ref().map(|a| a.tags.clone());
                let provider = StorageProvider::from(&storage);
                let (bucket, key) = match &storage {
                    StorageMetadata::S3 { bucket, key } => (bucket.as_str(), key.as_str()),
                    StorageMetadata::ABS { .. } => todo!(),
                };
                sqlx::query!(
                    "UPDATE video_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4,
                         storage_provider = $5, storage_bucket = $6, storage_key = $7,
                         width = $8, height = $9, duration = $10
                     WHERE id = $11",
                    generation.prompt,
                    generation.model,
                    title,
                    tags.as_deref(),
                    provider as StorageProvider,
                    bucket,
                    key,
                    width as i32,
                    height as i32,
                    duration as i32,
                    id,
                )
                .execute(&self.pool)
                .await?;
                Ok(id)
            }

            Metadata::Audio {
                generation,
                analysis,
                storage,
                format,
                duration,
                ..
            } => {
                let title = analysis.as_ref().map(|a| a.title.clone());
                let tags = analysis.as_ref().map(|a| a.tags.clone());
                let provider = StorageProvider::from(&storage);
                let (bucket, key) = match &storage {
                    StorageMetadata::S3 { bucket, key } => (bucket.as_str(), key.as_str()),
                    StorageMetadata::ABS { .. } => todo!(),
                };
                sqlx::query!(
                    "UPDATE audio_metadata
                     SET prompt = $1, model = $2, title = $3, tags = $4,
                         storage_provider = $5, storage_bucket = $6, storage_key = $7,
                         format = $8, duration = $9
                     WHERE id = $10",
                    generation.prompt,
                    generation.model,
                    title,
                    tags.as_deref(),
                    provider as StorageProvider,
                    bucket,
                    key,
                    format,
                    duration as i32,
                    id,
                )
                .execute(&self.pool)
                .await?;
                Ok(id)
            }
        }
    }

    async fn delete(&self, id: Uuid, kind: MetadataKind) -> Result<Metadata> {
        match kind {
            MetadataKind::Image => sqlx::query_as!(
                ImageRow,
                r#"DELETE FROM image_metadata WHERE id = $1
                   RETURNING id, prompt, model, title, tags,
                             storage_provider AS "storage_provider: StorageProvider",
                             storage_bucket, storage_key,
                             format, width, height,
                             created_at, updated_at"#,
                id
            )
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),

            MetadataKind::Video => sqlx::query_as!(
                VideoRow,
                r#"DELETE FROM video_metadata WHERE id = $1
                   RETURNING id, prompt, model, title, tags,
                             storage_provider AS "storage_provider: StorageProvider",
                             storage_bucket, storage_key,
                             format, width, height, duration,
                             created_at, updated_at"#,
                id
            )
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),

            MetadataKind::Audio => sqlx::query_as!(
                AudioRow,
                r#"DELETE FROM audio_metadata WHERE id = $1
                   RETURNING id, prompt, model, title, tags,
                             storage_provider AS "storage_provider: StorageProvider",
                             storage_bucket, storage_key,
                             format, duration,
                             created_at, updated_at"#,
                id
            )
            .fetch_optional(&self.pool)
            .await?
            .map(Metadata::from)
            .ok_or(Error::NotFound(id)),
        }
    }
}
