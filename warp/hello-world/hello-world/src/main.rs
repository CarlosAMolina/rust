use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let routes = build_routes().await;
    let ip = [127, 0 , 0, 1];
    let port = 3030;
    println!(
        "Listening on {}.{}.{}.{}:{}",
        ip[0], ip[1], ip[2], ip[3], port
    );
    warp::serve(routes).run((ip, port)).await;
}

// Reason to use `(impl Reply,)`: https://github.com/seanmonstar/warp/issues/1012
async fn build_routes() -> impl Filter<Extract = (impl Reply,)> + Clone {
    let get_hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(greet);
    let routes = get_hello;
    routes
}

async fn greet(name: String) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Params: name = {}", name);
    let response = format!("Hello, {}!", name);
    Ok(warp::reply::json(&response))
}

