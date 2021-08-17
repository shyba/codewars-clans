use warp::Filter;
use tera::{Tera, Context};

fn default() -> String {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    tera.render("index.html", &context).unwrap()
}

#[tokio::main]
async fn main() {
    let hello = warp::any().map(default);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
