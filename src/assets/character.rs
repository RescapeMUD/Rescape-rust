use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use serde_json::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub image: String,
    pub collection: CollectionInfo,
    pub attributes: Vec<Attribute>,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub name: String,
    pub family: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub files: Vec<MediaFile>,
    pub category: String,
    pub creators: Vec<Creator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaFile {
    pub uri: String,
    #[serde(rename = "type")]
    pub media_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    pub address: String,
    pub share: u8,
}

impl CharacterMetadata {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let metadata: CharacterMetadata = from_str(&contents)?;
        Ok(metadata)
    }
}