use serde_derive::Deserialize;
use warp::{Buf, Reply};
use warp::Filter;
use tera::{Tera, Context};
use serde::Serialize;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use std::collections::HashMap;
use std::str::FromStr;
use cached::proc_macro::cached;
use lazy_static::lazy_static;

const USERS: [User; 7] = [
    User {name: "shyba", initial_points: 770},
    User {name: "uncanned", initial_points: 0},
    User {name: "innng", initial_points: 3},
    User {name: "consoli", initial_points: 2},
    User {name: "luizdepra", initial_points: 65},
    User {name: "marcospb19", initial_points: 12},
    User {name: "v0idpwn", initial_points: 2}
];

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
            }
        };
        tera

        };
}

#[derive(Deserialize, Debug)]
struct CodewarsAPILanguage {
    rank: i8,
    name: String,
    color: String,
    score: i32
}

#[derive(Deserialize, Debug)]
struct CodewarsAPIRanks {
    overall: CodewarsAPILanguage,
    languages: HashMap<String, CodewarsAPILanguage>
}

#[derive(Deserialize, Debug)]
struct CodewarsAPIUser {
    username: String,
    name: Option<String>,
    honor: i32,
    clan: Option<String>,
    ranks: CodewarsAPIRanks,
}

#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    initial_points: i32,
}

#[derive(Debug, Serialize, Clone)]
struct Score {
    user: String,
    points: i32,
    initial_points: i32,
    current_points: i32
}

async fn fetch_user(user: &str) -> Result<CodewarsAPIUser, hyper::Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = format!("https://www.codewars.com/api/v1/users/{}", user);
    let res = client.get(Uri::from_str(&uri).unwrap()).await?;
    let body = hyper::body::aggregate(res).await?;
    let result: CodewarsAPIUser = serde_json::from_reader(body.reader()).unwrap();
    println!("{:?}", result);
    Ok(result)
}

#[cached(size=1, time=60)]
async fn fetch_scores() -> Vec<Score> {
    let mut scores: Vec<Score> = vec![];
    for user in USERS {
        let userdata = fetch_user(user.name).await.unwrap();
        scores.push(Score{
            user: user.name.parse().unwrap(),
            points: userdata.honor - user.initial_points,
            initial_points: user.initial_points,
            current_points: userdata.honor
        });
    }
    scores.sort_by_key(|x| {x.points.abs()});
    scores.reverse();
    scores
}

async fn default() -> Result<impl Reply, Infallible> {
    let mut context = Context::new();
    context.insert("scores", &fetch_scores().await);
    Ok(warp::reply::html(TEMPLATES.render("index.html", &context).unwrap()))
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
