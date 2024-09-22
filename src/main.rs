use async_sqlite::{Pool, PoolBuilder};
use axum::{
    http::Method,
    routing::{get, get_service, post},
    Router,
};
use clap::Parser;
use routes::{redirect_url, shorten_url};
use std::{
    net::SocketAddr,
    path::{Path as StdPath, PathBuf},
};
use tokio::{fs, net::TcpListener};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

mod routes;

struct AppState {
    db: Pool,
}

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short, long, default_value = "127.0.0.1:18732")]
    endpoint: SocketAddr,

    #[arg(short, long, default_value = "./urls.db")]
    database: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let db = PoolBuilder::new().path(&args.database).open().await.unwrap();

    db.conn(|c| {
        c.execute(
            r#"
        CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY,
            short_code TEXT UNIQUE NOT NULL,
            original_url TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_accessed TEXT NOT NULL
        )"#,
            [],
        )
    })
    .await
    .unwrap();

    let content_dir = "content";
    if !StdPath::new(content_dir).exists() {
        fs::create_dir(content_dir).await.expect("Failed to create content directory");
    }

    let app_state = AppState { db };
    let service = ServeDir::new(content_dir).fallback(ServeFile::new(format!("{content_dir}/index.html")));

    let cors = CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_headers(Any);
    let router = Router::new()
        .route("/shorten", post(shorten_url))
        .route("/:short", get(redirect_url))
        .fallback_service(get_service(service))
        .layer(cors)
        .with_state(app_state.into());

    let listener = TcpListener::bind(args.endpoint).await.unwrap();
    axum::serve(listener, router.into_make_service()).await.unwrap();
}
