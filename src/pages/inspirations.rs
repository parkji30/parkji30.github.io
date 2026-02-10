use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Inspiration link data
struct InspirationLink {
    title: &'static str,
    url: &'static str,
    note: &'static str,
}

const INSPIRATIONS: &[InspirationLink] = &[InspirationLink {
    title: "Math and me",
    url: "https://togelius.blogspot.com/2026/02/math-and-me.html#:~:text=I%20also%20always%20hated%20math,me%20get%20better%20at%20math.",
    note: "Julian Togelius - reflections on math, programming, and research",
}];

const AI_RESEARCHERS_WITHOUT_PHDS: &[&str] = &[
    "David Ha",
    "Andy Jones",
    "Sholto Douglas",
    "Christopher Olas",
    "Jeremy Howard",
    "Aleksa Gordic",
];

/// Inspirations page component
#[function_component(InspirationsPage)]
pub fn inspirations_page() -> Html {
    html! {
        <div class="app research-page">
            <div class="research-content">
                <nav class="research-nav">
                    <Link<Route> to={Route::Home} classes="back-link">
                        {"← Back"}
                    </Link<Route>>
                </nav>

                <article class="research-article">
                    <header class="research-header">
                        <h1 class="research-title">{"Inspirations"}</h1>
                        <div class="research-divider"></div>
                    </header>

                    <section class="research-section">
                        <h2>{"Testimonies From Other Researchers"}</h2>
                        <ul class="publications-list">
                            { for INSPIRATIONS.iter().map(|item| {
                                html! {
                                    <li>
                                        <a href={item.url} target="_blank" rel="noopener noreferrer" class="pub-title">{item.title}</a>
                                        <span class="pub-meta">{format!(" — {}", item.note)}</span>
                                    </li>
                                }
                            })}
                        </ul>

                        <div class="research-divider" style="width: 100%; margin: 24px 0;"></div>

                        <h2>{"AI Researchers Without PHDs"}</h2>
                        <ul class="publications-list" style="margin-top: 8px;">
                            { for AI_RESEARCHERS_WITHOUT_PHDS.iter().map(|name| {
                                html! {
                                    <li style="padding: 4px 0; border-bottom: none;">
                                        <span class="pub-title" style="font-size: 1rem;">{*name}</span>
                                    </li>
                                }
                            })}
                        </ul>
                    </section>
                </article>
            </div>
        </div>
    }
}
