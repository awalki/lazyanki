use anyhow::Ok;
use async_trait::async_trait;
use lazyanki_common::{get_client, load_config};
use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

#[async_trait]
pub trait ParserStrategy {
    async fn parse(&self) -> anyhow::Result<ParseResult>;
}

pub struct GermanStrategy {
    pub url: String,
    pub word: String,
}

pub struct EnglishStrategy {
    pub url: String,
    pub word: String,
}

pub struct ParseResult {
    pub word: String,
    pub translation: String,
    pub table: String,
}

pub async fn get_strategy(word: String) -> anyhow::Result<Box<dyn ParserStrategy>> {
    let config = load_config().expect("config not found");

    let strategy: Box<dyn ParserStrategy> = match config.languages.target_lang_tag.as_str() {
        "en-US" => Box::new(EnglishStrategy {
            url: "https://verbformen.ru/".to_string(),
            word: word.clone(),
        }),
        "de-DE" => Box::new(GermanStrategy {
            url: "https://verbformen.ru/".to_string(),
            word: word.clone(),
        }),
        _ => {
            println!("Unsupported language");
            return Err(anyhow::anyhow!("Unsupported language"));
        }
    };

    Ok(strategy)
}

#[async_trait]
impl ParserStrategy for EnglishStrategy {
    async fn parse(&self) -> anyhow::Result<ParseResult> {
        Ok(ParseResult {
            word: self.word.clone(),
            translation: String::from("not implemented"),
            table: String::from("not implemented"),
        })
    }
}

#[async_trait]
impl ParserStrategy for GermanStrategy {
    async fn parse(&self) -> anyhow::Result<ParseResult> {
        let mut url = Url::parse(&self.url)?;
        let client = get_client().await?;
        url.query_pairs_mut().append_pair("w", &self.word);

        let resp = client.get(url).send().await?;
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

        Ok(ParseResult {
            word: self.word.clone(),
            translation: format!("{}<br><br>{}", text, example_text),
            table: table_html,
        })
    }
}
