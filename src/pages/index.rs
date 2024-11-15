use std::sync::Arc;

use axum::extract::State;
use maud::{html, Markup, PreEscaped};

use crate::EntryLoaders;

use super::{render, Nav};

pub async fn index(State(loader): State<Arc<EntryLoaders>>) -> Markup {
    let profile = loader.top_loader.get_entry_for_slug("profile");
    let notice = loader.top_loader.get_entry_for_slug("notice");

    render(
        None,
        Nav::Home,
        None,
        html! {
            main #index {
                section #avatar {
                    img src="/images/me.jpg";
                }

                section #name {
                    h1 {
                        "Daisuke Murase"
                    }
                    h2 {
                        "@typester"
                    }
                }

                @if let Some(notice) = notice {
                    section #notice {
                        (PreEscaped(notice.content.clone()))
                    }
                }

                @if let Some(profile) = profile {
                    section #profile {
                        (PreEscaped(profile.content.clone()))
                    }
                }
            }
        },
    )
}
