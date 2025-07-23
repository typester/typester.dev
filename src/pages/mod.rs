use std::sync::Arc;

use axum::http::StatusCode;
use chrono::Datelike;
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::entry_loader::Entry;

pub mod blog;
pub mod index;
pub mod well_known;

#[derive(PartialEq)]
pub enum Nav {
    Home,
    Blog,
    CV,
    None,
}

const SITE_NAME: &str = "typester.dev";

pub fn render(
    title: Option<&str>,
    selected_nav: Nav,
    entry: Option<Arc<Entry>>,
    content: Markup,
) -> Markup {
    let (title, page_title) = match title {
        Some(t) => (format!("{} - {}", t, SITE_NAME), t),
        None => (SITE_NAME.to_string(), SITE_NAME),
    };

    html! {
        (DOCTYPE);
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) }
                link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Outfit";
                link rel="stylesheet" href="/css/main.css";
                link rel="alternate" type="application/rss+xml" href="/rss" title="RSS Feed";
                link rel="me" href="https://pdx.social/@typester";
                meta name="fediverse:creator" content="@typester@pdx.social";
                meta property="og:site_name" content="typester.dev";
                meta property="og:title" content=(page_title);
                @if let Some(entry) = entry {
                    meta property="og:type" content="article";
                    meta property="og:url" content=(format!("https://typester.dev/blog{}", entry.permalink()));
                    @if let Some(image) = entry.image.as_ref().map(|i| i.clone()) {
                        meta property="og:image" content=(image);
                    }
                } @else {
                    meta property="og:type" content="website";
                }
            }
            body {
                .container {
                    (nav(selected_nav));
                    (content);
                    (footer());
                }

                @if !cfg!(debug_assertions) {
                    script defer src="https://umami.typester.dev/script.js" data-website-id="649e0ea6-cd59-453a-8ef3-7f104699aedb" {}
                }
            }
        }
    }
}

fn nav(nav: Nav) -> Markup {
    html! {
        nav #main-nav {
            ul {
                (nav_item(nav == Nav::Home, html! {
                    a href="/" { "Home" }
                }))
                (nav_item(nav == Nav::Blog, html! {
                    a href="/blog" { "Blog" }
                }))
                (nav_item(nav == Nav::CV, html! {
                    a href="/cv.html" { "CV" }
                }))
            }
        }
    }
}

fn nav_item(selected: bool, content: Markup) -> Markup {
    html! {
        @if selected {
            li .selected { (content) }
        } @else {
            li { (content) }
        }
    }
}

fn footer() -> Markup {
    let yr = chrono::Local::now().year();
    html! {
        footer {
            p {
                "Copyright " (PreEscaped("&copy;")) " 2024-" (yr) " by Daisuke Murase."
            }
            p {
                "Powered by ";
                a href="https://orgmode.org/" { "org-mode"}
                ".";
            }
        }
    }
}

pub async fn not_found() -> (StatusCode, Markup) {
    (
        StatusCode::NOT_FOUND,
        render(
            None,
            Nav::None,
            None,
            html! {
                main #index {
                    h1 {
                        "404 Not Found";
                    }
                }
            },
        ),
    )
}
