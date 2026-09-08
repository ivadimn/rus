use serde::{Deserialize, Serialize};
use ureq::rustls::InvalidMessage::UnsupportedKeyExchangeAlgorithm;
use std::error::Error;


#[derive(Debug, Deserialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String
}

#[derive(Debug, Serialize)]
struct NewPost {
    title: String,
    body: String,
    userId: i32,
}

#[derive(Debug, Deserialize)]
struct PostResponse {
    id: i32,
    title: String,
    body: String,
    userId: i32,
}

pub fn simple_get(url: &str) -> Result<(), Box<dyn Error>> {

    let response = ureq::get(url)
        .set("Accept", "application/json")
        .call()?;

    let post: Post = response.into_json()?;

    println!("Получен пост:");
    println!("ID: {}", post.id);
    println!("Заголовок: {}", post.title);
    println!("Содержание: {}", post.body);

    Ok(())    
}

pub fn simple_get_post(url: &str) -> Result<(), Box<dyn Error>> {

    //создаём новый пост
    let new_post = NewPost {
       title: "Изучаем Rust".to_string(),
       body: "Rust - отличные язык программирования".to_string(),
       userId: 1,
    };

    //сериализуем в json
    let post_json = serde_json::to_string(&new_post)?;

    //отправляем POST-запрос
    let response = ureq::post(url)
        .set("Content-Type", "application/json")
        .send_string(&post_json)?;

    let post_response: PostResponse = response.into_json()?;

    println!("Создан пост с ID: {}", post_response.id);

    Ok(())
}