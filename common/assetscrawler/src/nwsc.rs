//networksearch

use reqwest;
use scraper;

#[tokio::main]
pub async fn readurl(url:&str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let html = client.get(url)
        .send().await?
        .text().await?;
    let doc = scraper::Html::parse_document(&html);
    Ok(doc.html())
}