use reqwest::{
    Client,
    header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue},
};

pub async fn get_client(native_lang_tag: String) -> anyhow::Result<Client> {
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_str(&native_lang_tag)?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client)
}
