use mini_redis::{Result, client};

#[tokio::main]
pub async fn main() -> Result<()> {
    // mini-resid　アドレスのコネクションを開く
    let mut client = client::connect("127.0.0.1:6379").await?;

    // "hello" というキーに"world"という値をセット
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value from servre; result={:?}", result);

    Ok(())
}
