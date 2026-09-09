use std::error::Error;
use http_body_util::{Empty, BodyExt};
use hyper::Request;
use hyper::body::{Bytes, Body};
use hyper::client::conn::http1::SendRequest;
use hyper_util::client::legacy::connect::Connected;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio::stream;
use tokio::io::{AsyncWriteExt as _, self};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    
    //парсим URL
    let url = "https://jsonplaceholder.typicode.com/posts/1".parse::<hyper::Uri>()?;
    
    //получаем host и port
    let host = url.host().expect("Uri has no host");
    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);
    
    //открываем TCP соединение с удалённым host
    let stream = TcpStream::connect(address).await?;

    //используем адаптер для доступа к некотрым возможностям 
    //tokio::io trait hyper::rt IO traits
    let io = TokioIo::new(stream);

    //Создаём Hyper клиента
    //let (mut sender: SendRequest<{unknown}>, conn: Connection<TokioIo<TcpStream, {unknown}>>) = hyper::client::conn::http1::handshake(io).await?;
    let (mut sender, conn) = 
        hyper::client::conn::http1::handshake::<TokioIo<tokio::net::TcpStream>, Empty<Bytes>>(io).await?;
    
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed {:?}", err)
        }
    });

    //авторизация нашего URL будет удалённое имя хоста httpbin
    let authoruty = url.authority().unwrap().clone();

    //создаём HTTP запрос с пустым телом и заголовком HOST
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authoruty.as_str())
        .body(Empty::<Bytes>::new())?;

    //ожидаем ответа ...
    let mut res = sender.send_request(req).await?;
    println!("Response status {}", res.status());


    while let Some(next) = res.frame().await  {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(chunk).await?
        }        
    }

    Ok(())
}
