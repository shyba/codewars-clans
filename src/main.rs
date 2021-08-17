use warp::Buf;
use warp::Filter;
use tera::{Tera, Context};
use serde::Serialize;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use serde_json::Value;

#[derive(Debug)]
struct User {
    name: String,
    initial_points: i32,
}

#[derive(Debug, Serialize, Clone)]
struct Score {
    user: String,
    points: i32,
    initial_points: i32,
    current_points: i32
}

async fn fetch_scores() -> Result<Vec<Score>, hyper::Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(Uri::from_static("https://www.codewars.com/api/v1/users/shyba")).await?;
    let body = hyper::body::aggregate(res).await?;
    let result: Value = serde_json::from_reader(body.reader()).unwrap();
    println!("{:?}", result);
    Ok(vec![])
}

async fn default() -> Result<String, Infallible> {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("scores", &fetch_scores().await.unwrap_or_else(|what| {
        println!("{:?}", what);
        [].to_vec()}));
    Ok(tera.render("index.html", &context).unwrap())
}

#[tokio::main]
async fn main() {
    let hello = warp::any().and_then(
        || async {
            default().await
        });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
