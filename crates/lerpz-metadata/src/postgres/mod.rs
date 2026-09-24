mod rows;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    MetadataClient, MetadataKind,
    error::{Error, Result},
    models::{AnalysisMetadata, GenerationMetadata, Metadata, MetadataUpdates, StorageMetadata},
};

use rows::{AudioRow, ImageRow, VideoRow};

/// Postgres-side representation of the storage provider.
///
/// Maps to the `storage_provider` enum type defined in the database schema.
/// Use [`From<&StorageMetadata>`] to convert from the domain model.
#[derive(sqlx::Type, Debug)]
#[sqlx(type_name = "storage_provider", rename_all = "lowercase")]
enum StorageProvider {
    /// S3-compatible storage provider.
    S3,
    /// Azure Blob Storage provider.
    #[sqlx(rename = "abs")]
    AzureBlob,
}

impl From<&StorageMetadata> for StorageProvider {
    fn from(s: &StorageMetadata) -> Self {
        match s {
            StorageMetadata::S3 { .. } => StorageProvider::S3,
            StorageMetadata::AzureBlob { .. } => StorageProvider::AzureBlob,
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
                    StorageMetadata::AzureBlob { .. } => todo!(),
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
                    StorageMetadata::AzureBlob { .. } => todo!(),
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
                    StorageMetadata::AzureBlob { .. } => todo!(),
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

    async fn update(&self, id: Uuid, kind: MetadataKind, updates: MetadataUpdates) -> Result<Uuid> {
        let MetadataUpdates {
            generation,
            analysis,
            storage,
        } = updates;

        let (prompt, model) = match generation {
            Some(GenerationMetadata { prompt, model }) => (Some(prompt), Some(model)),
            None => (None, None),
        };

        // COALESCE treats a null bind as "keep the current value", so it cannot
        // write an intentional null. Analysis carries a separate flag saying
        // whether to write at all, which lets a null clear the columns.
        let write_analysis = analysis.is_some();
        let (title, tags) = match analysis.flatten() {
            Some(AnalysisMetadata { title, tags }) => (Some(title), Some(tags)),
            None => (None, None),
        };

        let provider = storage.as_ref().map(StorageProvider::from);
        let (bucket, key) = match &storage {
            Some(StorageMetadata::S3 { bucket, key }) => {
                (Some(bucket.as_str()), Some(key.as_str()))
            }
            Some(StorageMetadata::AzureBlob { .. }) => todo!(),
            None => (None, None),
        };

        let result = match kind {
            MetadataKind::Image => {
                sqlx::query!(
                    "UPDATE image_metadata
                     SET prompt = COALESCE($2, prompt),
                         model = COALESCE($3, model),
                         title = CASE WHEN $4 THEN $5 ELSE title END,
                         tags = CASE WHEN $4 THEN $6 ELSE tags END,
                         storage_provider = COALESCE($7, storage_provider),
                         storage_bucket = COALESCE($8, storage_bucket),
                         storage_key = COALESCE($9, storage_key)
                     WHERE id = $1",
                    id,
                    prompt,
                    model,
                    write_analysis,
                    title,
                    tags.as_deref(),
                    provider as Option<StorageProvider>,
                    bucket,
                    key,
                )
                .execute(&self.pool)
                .await?
            }

            MetadataKind::Video => {
                sqlx::query!(
                    "UPDATE video_metadata
                     SET prompt = COALESCE($2, prompt),
                         model = COALESCE($3, model),
                         title = CASE WHEN $4 THEN $5 ELSE title END,
                         tags = CASE WHEN $4 THEN $6 ELSE tags END,
                         storage_provider = COALESCE($7, storage_provider),
                         storage_bucket = COALESCE($8, storage_bucket),
                         storage_key = COALESCE($9, storage_key)
                     WHERE id = $1",
                    id,
                    prompt,
                    model,
                    write_analysis,
                    title,
                    tags.as_deref(),
                    provider as Option<StorageProvider>,
                    bucket,
                    key,
                )
                .execute(&self.pool)
                .await?
            }

            MetadataKind::Audio => {
                sqlx::query!(
                    "UPDATE audio_metadata
                     SET prompt = COALESCE($2, prompt),
                         model = COALESCE($3, model),
                         title = CASE WHEN $4 THEN $5 ELSE title END,
                         tags = CASE WHEN $4 THEN $6 ELSE tags END,
                         storage_provider = COALESCE($7, storage_provider),
                         storage_bucket = COALESCE($8, storage_bucket),
                         storage_key = COALESCE($9, storage_key)
                     WHERE id = $1",
                    id,
                    prompt,
                    model,
                    write_analysis,
                    title,
                    tags.as_deref(),
                    provider as Option<StorageProvider>,
                    bucket,
                    key,
                )
                .execute(&self.pool)
                .await?
            }
        };

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(id));
        }

        Ok(id)
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
