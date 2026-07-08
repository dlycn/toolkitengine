//networksearch

use reqwest;
use scraper;

#[tokio::main]
pub async fn readurl(url: &str) -> String {
    let client = reqwest::Client::new();
    match client.get(url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                let doc = scraper::Html::parse_document(&text);
                doc.html() 
            }
            Err(_) => String::from("响应失败"),
        },
        Err(_) => String::from("请求失败"),
    }
}