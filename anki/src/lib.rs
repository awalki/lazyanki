pub mod models;

use models::{AddNote, BaseAction, DeckNamesResponse};
use reqwest::Client;

use crate::models::{AddNoteResponse, Params};

pub struct AnkiClient {
    pub url: String,
    pub client: Client,
}

impl AnkiClient {
    pub async fn deck_names(&self) -> anyhow::Result<DeckNamesResponse> {
        let action = BaseAction {
            action: "deckNames".to_string(),
            version: 6,
        };
        let json = self
            .client
            .post(&self.url)
            .json::<BaseAction>(&action)
            .send()
            .await?
            .json::<DeckNamesResponse>()
            .await?;

        Ok(json)
    }

    pub async fn add_note(&self, params: Params) -> anyhow::Result<AddNoteResponse> {
        let action = AddNote {
            action: "addNote".to_string(),
            version: 6,
            params: params,
        };
        let json = self
            .client
            .post(&self.url)
            .json::<AddNote>(&action)
            .send()
            .await?
            .json::<AddNoteResponse>()
            .await?;

        Ok(json)
    }
}
