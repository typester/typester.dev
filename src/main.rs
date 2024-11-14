use axum::{handler::HandlerWithoutStateExt, routing::get, Router};
use entry_loader::EntryLoader;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

mod entry_loader;
mod pages;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let data_dir = std::env::var("DATA_DIR")
        .ok()
        .unwrap_or("./entries-json".into());

    let blog_entries = EntryLoader::load("/blog".into(), format!("{}/blog", data_dir))?;

    let router = Router::new()
        .route("/", get(pages::index::index))
        .route("/blog", get(pages::blog::index))
        .route("/blog/tags/:tag", get(pages::blog::tag_index))
        .route("/blog/*path", get(pages::blog::permalink))
        .with_state(blog_entries)
        .fallback_service(
            ServeDir::new("public").not_found_service(pages::not_found.into_service()),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}

fn init_logger() {
    let log = tracing_subscriber::fmt().with_env_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("blog=debug")),
    );
    if let Some(_) = std::env::var("LOG_FORMAT_JSON").ok() {
        log.json().init();
    } else {
        log.compact().init();
    }
}
