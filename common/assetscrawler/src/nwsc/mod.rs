//networksearch

use reqwest;
pub mod task;
mod resdata;

pub async fn ayasurl(url: &str,client: &reqwest::Client) -> String {
    let start = std::time::Instant::now();
    match client.get(url).send().await {
        Ok(resp) => {
            let rcode = resp.status();
            match resp.text().await {
            Ok(text) => {
                let elapsed = start.elapsed(); // 计算实际耗时
                println!("{}\n成功获取响应，状态码: {}，实际耗时: {:.2}s", url,rcode, elapsed.as_secs_f64());
                text
            }
            Err(_) => String::from("响应失败"),
        }},
        Err(_) => String::from("请求失败"),
    }
}

async fn goresurl(client: &reqwest::Client, url: &str, save_path: &str) -> bool {
    let resp = client.get(url).send().await.expect("资源获取失败");
    if !resp.status().is_success() {
        return false;
    }
    let bytes = resp.bytes().await.expect("资源加载失败");
    std::fs::write(save_path, &bytes).expect("资源写入失败");
    println!("文件已保存\npath: {} \nsize: {} bytes)", save_path, bytes.len());
    true
}


pub async fn readurl(url: &str)->String {
    let client = reqwest::Client::new();
    ayasurl(url, &client).await
}


