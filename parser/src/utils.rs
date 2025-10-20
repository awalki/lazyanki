use reqwest::{
    Client,
    header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue},
};

pub async fn get_client() -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("ru-RU,ru;q=0.9,en;q=0.8"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
