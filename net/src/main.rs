use reqwest::Error;
mod http;
use http::{get_post, send_post};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let http = "https://jsonplaceholder.typicode.com/posts/1";
    let http1 = "https://jsonplaceholder.typicode.com/posts";

    //get_post(http).await.unwrap();
    send_post(http1).await.unwrap();

    Ok(())
}
