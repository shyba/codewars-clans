use warp::Filter;
use tera::{Tera, Context};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Score {
    user: String,
    points: i32,
    initial_points: i32,
    current_points: i32
}

fn default() -> String {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("scores", &[Score {user: "Victor".to_string(), points: 0, initial_points: 0, current_points: 0}]);
    tera.render("index.html", &context).unwrap()
}

#[tokio::main]
async fn main() {
    let hello = warp::any().map(default);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
