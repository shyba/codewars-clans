use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::any().map(|| {"olá"});

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
