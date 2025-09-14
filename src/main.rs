use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use hyper::body::to_bytes;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 创建支持 HTTPS 的连接器
    let https = HttpsConnector::new();
    // 创建客户端
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri: Uri = "https://mp.jobleap4u.com/".parse()?;

    let resp = client.get(uri).await?;

    println!("响应状态: {}", resp.status());

    let bytes = to_bytes(resp.into_body()).await?;
    let body_str = std::str::from_utf8(&bytes)?;
    println!("响应体内容:\n{}", body_str);

    Ok(())
}
