use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseAction {
    pub action: String,
    pub version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddNote {
    pub action: String,
    pub version: u8,
    pub params: Params,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub note: Note,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    #[serde(rename = "Front")]
    pub front: String,
    #[serde(rename = "Back")]
    pub back: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    #[serde(rename = "deckName")]
    pub deck_name: String,

    #[serde(rename = "modelName")]
    pub model_name: String,

    pub fields: Fields,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<NoteOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<MediaFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<MediaFile>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Vec<MediaFile>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteOptions {
    #[serde(rename = "allowDuplicate")]
    pub allow_duplicate: bool,

    #[serde(rename = "duplicateScope")]
    pub duplicate_scope: String,

    #[serde(rename = "duplicateScopeOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_scope_options: Option<DuplicateScopeOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicateScopeOptions {
    #[serde(rename = "deckName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deck_name: Option<String>,
    #[serde(rename = "checkChildren")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_children: Option<bool>,
    #[serde(rename = "checkAllModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_all_models: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaFile {
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(rename = "skipHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_hash: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnkiResponse<T> {
    pub result: Option<T>,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeckNamesResponse {
    pub result: Vec<String>,
    pub error: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddNoteResponse {
    pub result: Option<u64>,
    pub error: Option<String>,
}
