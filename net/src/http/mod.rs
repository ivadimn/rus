use std::fmt::format;

use serde::{Serialize, Deserialize};
use reqwest::{Client, Error, Response, StatusCode, header};

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

pub async fn headers(http: &str) -> Result<(), Error> {

    let client = Client::new();
    
    //Создание пользовательских заголовков
    let result = client.get(http)
        .header(header::CONTENT_TYPE, "application/json")
        .header("X-Custom-Header", "Custom-value")
        .send()
        .await;

    match result {
        Ok(response) => {
            println!("Статус: {}", response.status());
            println!("Заголовки ответа:");
            for (name, value) in response.headers() {
                println!("{}: {}", name.as_str(), value.to_str().unwrap_or("[невалидное значение]"))
            }
        },
        Err(e) => println!("Ошибка получения заголовков: {:?}", e),
    }

    Ok(())
} 

pub async fn headers_default(http: &str) ->Result<(), Error> {

    //создаём карту заголовков
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str("Bearer YOUR_API_TOKEN").unwrap()
    );

    //добавляем заголовок типа контента
    headers.insert(
        header::CONTENT_TYPE, 
        header::HeaderValue::from_static("application/json")
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let result = client.get(http)
                    .send()
                    .await;

    match result {
        Ok(response) => {
            println!("Статус: {}", response.status());
        },
        Err(e) => println!("INTO FUNC ERROR: {:?}", e),
    }

    Ok(())
}

pub async fn fetch_post(client: &Client, http: &str, id: i32) -> Result<String, String> {

    let url = format!("{}/{}", http, id);

    let response = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => return Err(format!("Ошибка при чтении ответа: {}", e)),
    };

    match response.status() {
        StatusCode::OK => {
            match response.text().await {
                Ok(text) => Ok(text),
                Err(e) => Err(format!("Ошибка при чтении ответа: {}", e)),
            }
        }
        StatusCode::NOT_FOUND => {
            Err(format!("Пост с id {} не найден", id))
        },
        StatusCode::UNAUTHORIZED => {
            Err("Требуется авторизация".to_string())
        },
        status => {
            Err(format!("Неожиданный статус {}", status))
        },
    }

}

pub async fn get_posts(http: &str) {
    let client = Client::new();

    //получаем существующий пост
    match fetch_post(&client, http, 1).await {
        Ok(content) => println!("Получен пост: {}", content),
        Err(e) => eprintln!("Ошибка: {}", e),
    }

    //получаем несуществующий пост
    match fetch_post(&client, http, 9999).await {
        Ok(content) => println!("Получен пост: {}", content),
        Err(e) => eprintln!("Ошибка: {}", e),
    }
} 