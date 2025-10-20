use anyhow::Ok;
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

pub mod utils;

#[async_trait]
pub trait ParserStrategy {
    async fn parse(&self) -> anyhow::Result<GermanParseResult>;
}

pub struct GermanStrategy {
    pub url: String,
    pub word: String,
    pub client: Client,
}

pub struct GermanParseResult {
    pub word: String,
    pub translation: String,
    pub table: String,
}

#[async_trait]
impl ParserStrategy for GermanStrategy {
    async fn parse(&self) -> anyhow::Result<GermanParseResult> {
        let mut url = Url::parse(&self.url)?;
        url.query_pairs_mut().append_pair("w", &self.word);

        let resp = self.client.get(url).send().await?;
        let html = resp.text().await?;

        let document = Html::parse_document(&html);

        let translation_selector = Selector::parse(r#"span[lang]"#).unwrap();

        let table_selector = Selector::parse(".vTbl").unwrap();
        let example_selector = Selector::parse("p.rInf.r1Zeile.rU3px.rO0px.rNt").unwrap();

        let span = document.select(&translation_selector).next().unwrap();

        let table_html = format!(
            r#"<div style="display: flex; flex-wrap: wrap; gap: 18px; justify-content: center;">
            {}
            </div>"#,
            document
                .select(&table_selector)
                .take(7)
                .map(|el| {
                    let html = el.html();
                    let re = Regex::new(r"<span[^>]*>.*?</span>").unwrap();
                    re.replace_all(&html, "").to_string()
                })
                .collect::<Vec<_>>()
                .join("\n")
        );

        let example_text = document
            .select(&example_selector)
            .last()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_default();
        let text = span.text().collect::<String>();

        Ok(GermanParseResult {
            word: self.word.clone(),
            translation: format!("{}<br><br>{}", text, example_text),
            table: table_html,
        })
    }
}
