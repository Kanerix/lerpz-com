use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// General metadata.
///
/// This should be present in all metadata. Use this with serde's flatten field
/// attribute to store them in a nice readable way.
#[derive(Serialize, Deserialize)]
pub struct GeneralMetadata {
    pub id: Uuid,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

/// Generation metadata.
///
/// This is used when AI has generated the content. It contains the most generic
/// information most models use to generate the content it points to.
#[derive(Serialize, Deserialize)]
pub struct GenerationMetadata {
    pub prompt: String,
    pub model: String,
}

/// Analysis metadata.
///
/// This is used when AI has analyzed the content.
#[derive(Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub title: String,
    pub tags: Vec<String>,
}

/// Storage metadata.
///
/// This is where the content is stored. This will be used to retrieve the
/// content.
#[derive(Serialize, Deserialize)]
pub enum StorageMetadata {
    /// AWS S3.
    S3 { bucket: String, key: String },
    /// Azure Blob Storage.
    ABS { container: String, blob: String },
}

/// Metadata for a generic content type.
///
/// This is the main metadata type that is stored in the database.
///
/// It is tagged with the `type` field so that the database can distinguish
/// between chat, image, and video metadata.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Metadata {
    /// Image metadata.
    Image {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        storage: StorageMetadata,
        analysis: Option<AnalysisMetadata>,
        format: String,
        width: u32,
        height: u32,
    },
    /// Video metadata.
    Video {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        storage: StorageMetadata,
        analysis: Option<AnalysisMetadata>,
        format: String,
        width: u32,
        height: u32,
        duration: u32,
    },
    /// Audio metadata.
    Audio {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        storage: StorageMetadata,
        analysis: Option<AnalysisMetadata>,
        format: String,
        duration: u32,
    },
}
