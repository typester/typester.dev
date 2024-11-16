use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use maud::{html, Markup, PreEscaped};

use crate::{
    entry_loader::Entry,
    pages::{self, render, Nav},
    EntryLoaders,
};

pub async fn index(State(loader): State<Arc<EntryLoaders>>) -> Markup {
    let entries = loader.blog_loader.get_entries_by_year();

    render(
        Some("Blog"),
        Nav::Blog,
        None,
        html! {
            main #blog {
                @for (yr, entries) in entries.iter().rev() {
                    section .year {
                        .year { (yr) }
                        section {
                            @for entry in entries.iter() {
                                article data-eid=(entry.eid) {
                                    .month { (entry.date.format("%b %d")) }
                                    h2 {
                                        a href=(format!("/blog{}", entry.permalink())) {
                                            (entry.title)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}

pub async fn tag_index(
    Path(tag): Path<String>,
    State(loader): State<Arc<EntryLoaders>>,
) -> (StatusCode, Markup) {
    let entries = match loader.blog_loader.get_entries_by_tag(&tag) {
        Some(entries) => entries,
        None => {
            return pages::not_found().await;
        }
    };

    (
        StatusCode::OK,
        render(
            Some(&format!("#{}", tag)),
            Nav::Blog,
            None,
            html! {
                main #blog {
                    section .tag {
                        h2 .tag { "#" (tag) }
                        section {
                            @for entry in entries.iter() {
                                article data-eid=(entry.eid) {
                                    .month { (entry.date.format("%b %m, %Y")) }
                                    h2 {
                                        a href=(format!("/blog{}", entry.permalink())) {
                                            (entry.title)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
        ),
    )
}

pub async fn permalink(
    State(loader): State<Arc<EntryLoaders>>,
    Path(path): Path<String>,
) -> (StatusCode, Markup) {
    tracing::debug!(%path, "get permalink");
    let Some(entry) = loader
        .blog_loader
        .get_entry_for_path(&format!("/blog/{}", path))
    else {
        return pages::not_found().await;
    };

    (
        StatusCode::OK,
        render(
            Some(&entry.title),
            Nav::Blog,
            Some(entry.clone()),
            blog_content(&entry),
        ),
    )
}

fn blog_content(entry: &Entry) -> Markup {
    html! {
        main #blog {
            article .permalink data-eid=(entry.eid) {
                h1 {
                    a href=(format!("/blog{}",entry.permalink())) {
                        (entry.title)
                    }
                }

                .meta {
                    .date { (entry.date.format("%Y-%m-%dT%H:%M:%S%z")) }
                    @if entry.tags.len() > 0 {
                        ul .tags {
                            @for tag in &entry.tags {
                                li {
                                    a href=(format!("/blog/tags/{}", tag)) {
                                        "#" (tag)
                                    }
                                }
                            }
                        }
                    }
                }

                .content {
                    (PreEscaped(entry.content.clone()));
                }
            }
        }
    }
}
