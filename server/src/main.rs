mod setup_logger;

use dotenv::EnvLoader;
use server::api;

#[tokio::main]
async fn main() {
    setup_logger::setup();

    let app = api::routes();

    let env = EnvLoader::new().required(false).load().unwrap();

    let listen_addr = env.var("SULFATE_SERVER_LISTEN_ADDR").unwrap();

    let listener = tokio::net::TcpListener::bind(listen_addr).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
