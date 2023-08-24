use warp::{http::StatusCode, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let routes = build_routes().await;
    let ip = [0, 0 , 0, 0];
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
        .and(warp::header("user-agent"))
        .and(warp::path::end())
        .and_then(greet);
    let routes = get_hello.recover(return_error);
    routes
}

async fn greet(name: String, user_agent: String) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Init greet");
    let response = format!("Hello {}! User agent: {}", name, user_agent);
    Ok(warp::reply::json(&response))
}


pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if r.is_not_found() {
        println!("Requested route was not found");
        Ok(warp::reply::with_status(
            "Requested route was not found",
            StatusCode::NOT_FOUND,
        ))
    } else {
        println!("Unknown error: {:?}", r);
        Ok(warp::reply::with_status(
            "Unknown error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
