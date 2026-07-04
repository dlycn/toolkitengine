use reqwest::Client;
use futures;
use anyhow::{Result, anyhow};
use std::time::Duration;
use tokio::time::sleep;

// 1️⃣ 获取状态码和文本
async fn test_status_and_text(client: &Client, url: &str) -> Result<()> {
    let resp = client.get(url).send().await?;
    let status = resp.status();
    println!("状态码: {}", status);
    if status.is_success() {
        let text = resp.text().await?;
        println!("文本长度: {} 字节", text.len());
        // 只打印前200个字符
        if text.len() > 200 {
            println!("内容预览: {:.200}...", text);
        } else {
            println!("内容: {}", text);
        }
    } else {
        println!("请求失败，状态码: {}", status);
    }
    Ok(())
}

// 2️⃣ 测试空链接（无效域名/错误URL）
async fn test_invalid_url(client: &Client) -> Result<()> {
    // 故意构造一个不存在的主机
    let url = "https://this-domain-should-not-exist-12345.com/";
    match client.get(url).send().await {
        Ok(_) => println!("意外成功访问了无效链接？"),
        Err(e) => println!("❌ 访问无效链接正确报错: {}", e),
    }
    Ok(())
}

// 3️⃣ 超时与重试（自己实现重试逻辑）
async fn test_timeout_and_retry(client: &Client, url: &str) -> Result<()> {
    let max_retries = 7;
    let mut dtime: u64 = 5;
    let mut attempt = 0;
    loop {
        attempt += 1;
        let timeout = Duration::from_secs(dtime);
        println!("第 {} 次尝试 (超时设置: {}s)...", attempt, dtime);
        let start = std::time::Instant::now(); // 开始计时
        match tokio::time::timeout(timeout, client.get(url).send()).await {
            Ok(Ok(resp)) => {
                let elapsed = start.elapsed(); // 计算实际耗时
                println!("✅ 成功获取响应，状态码: {}，实际耗时: {:.2}s", resp.status(), elapsed.as_secs_f64());
                return Ok(());
            }
            Ok(Err(e)) => {
                let elapsed = start.elapsed();
                println!("请求错误: {}，耗时: {:.2}s", e, elapsed.as_secs_f64());
            }
            Err(_) => {
                let elapsed = start.elapsed();
                println!("⏰ 超时！实际耗时: {:.2}s (超过设定超时)", elapsed.as_secs_f64());
            }
        }
        if attempt >= max_retries {
            return Err(anyhow!("超过最大重试次数"));
        }
        dtime += 5;
        sleep(Duration::from_secs(1)).await;
    }
}

// // 4️⃣ 构造请求头（User-Agent, Referer, 自定义头等）
// async fn test_custom_headers(client: &Client) -> Result<()> {
//     // 创建一个新的 Client 以演示自定义 headers（也可以使用 .header() 单次发送）
//     let custom_client = reqwest::Client::builder()
//         .user_agent("MyRustCrawler/1.0")
//         .default_headers({
//             let mut headers = reqwest::header::HeaderMap::new();
//             headers.insert("X-Custom-Header", "Hello".parse().unwrap());
//             headers
//         })
//         .build()?;

//     let resp = custom_client
//         .get("https://httpbin.org/headers")
//         .header("Referer", "https://example.com")
//         .send()
//         .await?;
//     let body = resp.text().await?;
//     println!("📨 服务端收到的请求头: \n{}", body);
//     Ok(())
// }

// 5️⃣ 下载直链资源（图片/文件）
async fn test_download_file(client: &Client, url: &str, save_path: &str) -> Result<()> {
    let resp = client.get(url).send().await?;
    if !resp.status().is_success() {
        return Err(anyhow!("下载失败，状态码: {}", resp.status()));
    }
    let bytes = resp.bytes().await?;
    std::fs::write(save_path, &bytes)?;
    println!("📥 文件已保存至: {} (大小: {} bytes)", save_path, bytes.len());
    Ok(())
}

// 6️⃣ 动态网页测试（模拟等待——但普通 reqwest 无法执行 JS）
async fn test_dynamic_page() {
    println!("⚠️ 注意：reqwest 本身不能执行 JavaScript，只能获取原始 HTML。");
    println!("如果需要渲染动态内容，请使用 headless 浏览器库（如 thirtyfour、headless_chrome）或预渲染服务。");
    // 可以尝试：获取一个 SPA 页面，打印其 HTML，你会发现内容很少。
    let client = Client::new();
    let url = "https://example.com"; // 这是一个静态页面，改为一个真正 SPA 比如某个 Vue 应用。
    // 例如：https://vuejs.org/（Vue官网是SSR，其实内容完整，不好演示）
    // 可以故意拿一个需要JS渲染的页面，但这里只作概念说明。
    match client.get(url).send().await {
        Ok(resp) => {
            let html = resp.text().await.unwrap_or_default();
            println!("📄 获取到 HTML 长度: {} 字符", html.len());
            println!("（如果你期望看到动态内容，请换用 headless 浏览器）");
        }
        Err(e) => println!("请求失败: {}", e),
    }
}

async fn concurrent_test(client: &Client) -> Result<()> {
    let urls = [
        "https://postman-echo.com/delay/5",
        "https://postman-echo.com/delay/5",
        "https://postman-echo.com/delay/5",
    ];
    let start = std::time::Instant::now();
    let tasks: Vec<_> = urls.iter()
        .map(|url| client.get(*url).send())
        .collect();
    let responses = futures::future::join_all(tasks).await;
    for resp in responses {
        let status = resp?.status();
        println!("状态码: {}", status);
    }
    println!("并发总耗时: {:.2}s", start.elapsed().as_secs_f64());
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {

    let client = Client::new();

    println!("========== 1. 状态码与文本 ==========");
    test_status_and_text(&client, "https://httpbin.org/ip").await?;

    println!("\n========== 2. 空链接（无效域名） ==========");
    test_invalid_url(&client).await?;

    println!("\n========== 3. 超时与重试 ==========");
    // 使用一个响应较慢的 URL（httpbin.org/delay/10）模拟超时
    test_timeout_and_retry(&client, "https://postman-echo.com/delay/10").await?; // 会超时然后重试

    // println!("\n========== 4. 自定义请求头 ==========");
    // test_custom_headers(&client).await?;

    println!("\n========== 5. 下载直链资源 ==========");
    // 下载一张小图片（例如 favicon）
    let img_url = "https://www.rust-lang.org/static/images/favicon.ico";
    test_download_file(&client, img_url, "rust-favicon.ico").await?;

    println!("\n========== 6. 动态页面说明 ==========");
    test_dynamic_page().await;

    concurrent_test(&client).await?;

    println!("\n✅ 所有测试完成！");
    Ok(())
}