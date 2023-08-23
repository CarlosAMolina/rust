use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));
    let ip = [127, 0 , 0, 1];
    let port = 3030;
    println!(
        "Listening on {}.{}.{}.{}:{}",
        ip[0], ip[1], ip[2], ip[3], port
    );
    warp::serve(hello).run((ip, port)).await;
}
