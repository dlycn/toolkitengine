//networksearch

use reqwest;
use scraper;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // 方式一：快捷方式（每次创建新 Client，不推荐复用）
    let html = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    let doc = scraper::Html::parse_document(&html);
    println!("{}",doc.html());
    Ok(())
}