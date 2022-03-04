use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::path!("hello" / String).map(|name| format!("Hello, world!, {}", name));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
