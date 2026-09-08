use std::error::Error;
mod synchttp;
use synchttp::{simple_get, simple_get_post};


fn main() -> Result<(), Box<dyn Error>>  {
    let http1 = "https://jsonplaceholder.typicode.com/posts/1";
    let http = "https://jsonplaceholder.typicode.com/posts";

    //get_post(http).await.unwrap();
    //send_post(http1).await.unwrap();
    //headers(http).await.unwrap();
    // let result = headers_default(http).await;
    // match result {
    //     Ok(_) => println!("All Ok!!"),
    //     Err(e) => println!("Error: {:?}", e),       
    // }
    //get_posts(http1).await;
    //simple_get(http1)?;
    simple_get_post(http)?;

    Ok(())
}
