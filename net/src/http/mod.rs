use serde::{Serialize, Deserialize};
use reqwest::{Client, Error, Response, header};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String
}

#[derive(Debug, Serialize, Deserialize)]
struct NewPost {
    title: String,
    body: String,
    userId: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostResponse {
    id: i32,
    title: String,
    body: String,
    userId: i32,
}

pub async fn get_post(http: &str) -> Result<(), Error> {
    let response = 
        reqwest::get(http)
        .await;

    match response {
        Ok(r) => {
            println!("Статус: {}", r.status());
            //let post: Post = r.json().await.unwrap();
            //println!("Тело поста:\n{:?}", post);    
            if let Ok(post) = r.json::<Post>().await {
                println!("Тело поста:\n{:?}", post);    
            }
            else {
                println!("Json Error!!!");
            }
        },
        Err(e) => println!("Error: {:?}", e),
    }    
    Ok(())
}

pub async fn send_post(http: &str) -> Result<(), Error> {
    let client = Client::new();
    let new_post = NewPost {
       title: "Изучаем Rust".to_string(),
       body: "Rust - отличные язык программирования".to_string(),
       userId: 1,
    };

    let result = client.post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await;
    
    match result {
        Ok(response) => {
            println!("Статус: {}", response.status());
            let post_response: PostResponse = response.json().await.unwrap();
            println!("Создан пост с id: {}", post_response.id);
        },
        Err(e) => println!("Ошибка: {:?}", e),
    }

    Ok(())
}

