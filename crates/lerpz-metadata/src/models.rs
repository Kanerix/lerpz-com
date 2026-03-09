use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// General metadata.
///
/// This should be present in all metadata. Use this with serde's flatten field
/// attribute to store them in a nice readable way.
#[derive(Serialize, Deserialize)]
pub struct GeneralMetadata {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

/// Generation metadata.
///
/// This is used when AI has generated the content. It contains the most generic
/// information most models use to generate the content it points to.
#[derive(Serialize, Deserialize)]
pub struct GenerationMetadata {
    prompt: String,
    model: String,
}

/// Analysis metadata.
///
/// This is used when AI has analyzed the content.
#[derive(Serialize, Deserialize)]
pub struct AnalysisMetadata {
    title: String,
    tags: Vec<String>,
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

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Metadata {
    /// Chat metadata.
    Chat {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        analysis: AnalysisMetadata,
    },
    /// Image metadata.
    Image {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        analysis: AnalysisMetadata,
        storage: StorageMetadata,
        width: u32,
        height: u32,
    },
    /// Video metadata.
    Video {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        analysis: AnalysisMetadata,
        storage: StorageMetadata,
        width: u32,
        height: u32,
        duration: u32,
    },
    /// Audio metadata.
    Audio {
        #[serde(flatten)]
        general: GeneralMetadata,
        generation: GenerationMetadata,
        analysis: AnalysisMetadata,
        storage: StorageMetadata,
        duration: u32,
    },
}
