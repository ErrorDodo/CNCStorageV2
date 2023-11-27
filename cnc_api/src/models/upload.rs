use serde::Deserialize;

#[derive(Deserialize)]
pub struct UploadDTO {
    pub file_format: String,
    pub resolution: String,
    pub tags: Vec<String>,

    pub upload_type: String,
    pub duration: Option<i64>,
}
