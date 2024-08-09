mod utils;

#[tokio::main]
async fn main() {
    log4rs::init_file("./log4rs.yml", Default::default()).unwrap();
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, new_app()).await.unwrap();
}