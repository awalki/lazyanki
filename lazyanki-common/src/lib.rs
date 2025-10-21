use anyhow::Ok;
use directories::BaseDirs;
use lazyanki_anki::AnkiClient;
use reqwest::{
    Client,
    header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub languages: Languages,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Languages {
    pub native_lang_tag: String,
    pub target_lang_tag: String,
}

pub fn create_config(native_lang_tag: String, target_lang_tag: String) -> anyhow::Result<()> {
    let base_dirs = BaseDirs::new().expect("cannot found base_dirs");

    let path = base_dirs.config_dir().join("lazyanki");
    let config = Config {
        languages: Languages {
            native_lang_tag,
            target_lang_tag,
        },
    };

    if !path.exists() {
        fs::create_dir_all(&path)?;
        fs::write(path.join("config.toml"), toml::to_string(&config)?)?;
        println!("✅ Config has been created successfully");
    }

    Ok(())
}

pub fn load_config() -> Option<Config> {
    let base_dirs = BaseDirs::new().expect("cannot found base_dirs");
    let path = base_dirs.config_dir().join("lazyanki").join("config.toml");

    if !path.exists() {
        return None;
    }

    let s = fs::read_to_string(path).unwrap();
    let cfg: Config = toml::from_str(&s).unwrap();

    Some(cfg)
}

pub async fn get_client() -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    let config = load_config().expect("config not found");

    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_str(&config.languages.native_lang_tag)?,
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}

pub async fn get_anki_client() -> anyhow::Result<AnkiClient> {
    let client = get_client().await?;

    let anki_client = AnkiClient {
        client,
        url: "http://localhost:8765".to_string(),
    };

    Ok(anki_client)
}
