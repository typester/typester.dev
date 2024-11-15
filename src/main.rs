use std::sync::Arc;

use axum::{
    extract::State,
    handler::HandlerWithoutStateExt,
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use entry_loader::EntryLoader;
use rss::{Channel, ChannelBuilder, ItemBuilder};
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;

mod entry_loader;
mod pages;

pub struct EntryLoaders {
    pub top_loader: Arc<EntryLoader>,
    pub blog_loader: Arc<EntryLoader>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger();

    let data_dir = std::env::var("DATA_DIR")
        .ok()
        .unwrap_or("./entries-json".into());

    let top_loader = EntryLoader::load("".into(), data_dir.clone(), false)?;
    let blog_loader = EntryLoader::load("/blog".into(), format!("{}/blog", data_dir), true)?;

    let loader = Arc::new(EntryLoaders {
        top_loader,
        blog_loader,
    });

    let router = Router::new()
        .route("/", get(pages::index::index))
        .route("/blog", get(pages::blog::index))
        .route("/blog/tags/:tag", get(pages::blog::tag_index))
        .route("/blog/*path", get(pages::blog::permalink))
        .route("/rss", get(rss))
        .with_state(loader)
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

async fn rss(State(loader): State<Arc<EntryLoaders>>) -> (StatusCode, RssChannel) {
    let entries = loader.blog_loader.get_entries();

    let mut channel = ChannelBuilder::default()
        .title("typester.dev")
        .link("https://typester.dev")
        .description("Random thoughts from Daisuke Murase")
        .pub_date(entries[0].date.to_rfc2822())
        .build();

    let mut items = vec![];
    for entry in entries.iter() {
        let item = ItemBuilder::default()
            .title(entry.title.clone())
            .link(format!("https://typester.dev/blog{}", entry.permalink()))
            .pub_date(entry.date.to_rfc2822())
            .content(entry.content.clone())
            .build();
        items.push(item);
    }
    channel.set_items(items);

    (StatusCode::OK, RssChannel(channel))
}

struct RssChannel(Channel);

impl IntoResponse for RssChannel {
    fn into_response(self) -> axum::response::Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/rss+xml; charset=utf-8")
            .body(self.0.to_string().into())
            .unwrap()
    }
}
