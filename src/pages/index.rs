use maud::{html, Markup};

use super::{render, Nav};

pub async fn index() -> Markup {
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

                section #profile {
                    ul {
                        li {
                            "Software Engineer with extensive experience across various languages and fields, specializes in building robust, scalable, and high-performance network applications."
                        }
                        li {
                            "Also known as " strong { "typester" } " on the internet."
                        }
                        li {
                            "Loves " strong { "Rust" } ", Long-time " strong { "Emacs" } " user."
                        }
                    }
                }
            }
        },
    )
}
