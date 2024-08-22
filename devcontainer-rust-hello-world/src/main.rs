use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the route
    let hello = warp::path::end()
        .map(|| warp::reply::html("<html><body><h1>Hello, World!</h1></body></html>"));

    // Start the warp server on port 3030
    warp::serve(hello)
        .run(([127, 0, 0, 1], 8080))
        .await;
}