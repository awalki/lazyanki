use anyhow::{Context, Result};
use directories::BaseDirs;
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
    if let Some(base_dirs) = BaseDirs::new() {
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
        } else {
            println!("🟥 Config already exists");
        }
    }

    Ok(())
}

pub fn load_config() -> Result<Config> {
    if let Some(base_dirs) = BaseDirs::new() {
        let path = base_dirs.config_dir().join("lazyanki").join("config.toml");

        if !path.exists() {
            panic!("init lazyanki first\n\nuse: lazyanki init --native en-US --target de-DE")
        }

        let s = fs::read_to_string(path).context("read config")?;
        let cfg: Config = toml::from_str(&s).context("parse TOML")?;

        return Ok(cfg);
    }

    panic!("unexpected error: feel free to make an issue on github")
}
