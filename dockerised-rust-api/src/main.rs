use warp::Filter;

#[tokio::main]
async fn main() {
    let sense_check = warp::path::end()
        .map(|| format!("sense check on / - server running ok"));

    let another_route = warp::path("test_route")
        .map(|| format!("this is the test route"));

    let routes = sense_check.or(another_route);

    let port = 3030;

    println!("serving on {}", port);

    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}
