//! Footer component with Copyright, License Link, Repository Link, Build info.

use yew::prelude::*;

// Build info from build.rs
const BUILD_COMMIT: &str = env!("BUILD_COMMIT");
const BUILD_HOST: &str = env!("BUILD_HOST");
const BUILD_TIME: &str = env!("BUILD_TIME");

// Links
const LICENSE_URL: &str = "https://github.com/wrightmikea/tt-rs/blob/main/LICENSE";
const REPO_URL: &str = "https://github.com/wrightmikea/tt-rs";
const CHANGELOG_URL: &str = "https://github.com/wrightmikea/tt-rs/blob/main/CHANGELOG.md";
const COMMIT_URL_BASE: &str = "https://github.com/wrightmikea/tt-rs/commit/";

/// Footer component displaying build and project info.
#[function_component(Footer)]
pub fn footer() -> Html {
    let commit_url = format!("{COMMIT_URL_BASE}{BUILD_COMMIT}");
    let short_commit = &BUILD_COMMIT[..7.min(BUILD_COMMIT.len())];

    html! {
        <footer class="app-footer">
            <span>{"Copyright (c) 2025-2026 Michael A Wright"}</span>
            <span>{" | "}</span>
            <a href={LICENSE_URL}>{"BSD License"}</a>
            <span>{" | "}</span>
            <a href={REPO_URL}>{"GitHub"}</a>
            <span>{" | "}</span>
            <a href={CHANGELOG_URL}>{"Changes"}</a>
            <span>{" | Built: "}</span>
            <span>{BUILD_TIME}</span>
            <span>{" on "}</span>
            <span>{BUILD_HOST}</span>
            <span>{" "}</span>
            <a href={commit_url}>{short_commit}</a>
        </footer>
    }
}
