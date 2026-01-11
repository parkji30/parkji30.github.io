use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Publication data
struct Publication {
    title: &'static str,
    url: &'static str,
    meta: &'static str,
}

/// Research post data
struct ResearchPostData {
    slug: &'static str,
    title: &'static str,
    date: &'static str,
    description: &'static str,
    publications: &'static [Publication],
}

const RESEARCH_POSTS: &[ResearchPostData] = &[
    ResearchPostData {
        slug: "computational-astrophysics",
        title: "My Old Life (Msc. Grad School)",
        date: "2020 - 2022",
        description: "During my graduate studies at Queen's University, my research focused on the intersection of computational astrophysics and applied mathematics. I used numerical simulations to calibrate the Davis-Chandrasekhar-Fermi (DCF) method, a technique for estimating magnetic field strengths in molecular clouds from dust polarization observations.\n\nBy analyzing simulated Colliding Flow and Collapsing Cloud environments, I evaluated the accuracy of different DCF variations and identified key factors that influence their reliability.",
        publications: &[
            Publication {
                title: "Calibrating the Davis-Chandrasekhar Fermi Method With Molecular Cloud Simulations",
                url: "https://qspace.library.queensu.ca/server/api/core/bitstreams/319bb098-1b32-4487-bf2c-b0dae61cc27b/content",
                meta: "M.Sc. Thesis, Queen's University, 2022",
            },
            Publication {
                title: "The Davis–Chandrasekhar–Fermi method revisited",
                url: "https://academic.oup.com/mnras/article/514/2/1575/6604898",
                meta: "Monthly Notices of the Royal Astronomical Society, 2022",
            },
        ],
    },
];

/// Research listing page component
#[function_component(ResearchPage)]
pub fn research_page() -> Html {
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
                        <h1 class="research-title">{"Research"}</h1>
                        <div class="research-divider"></div>
                    </header>

                    <div class="posts-list">
                        { for RESEARCH_POSTS.iter().map(|post| {
                            html! {
                                <Link<Route> to={Route::ResearchPost { slug: post.slug.to_string() }} classes="post-link">
                                    <span class="post-link-title">{post.title}</span>
                                    <span class="post-link-date">{post.date}</span>
                                </Link<Route>>
                            }
                        })}
                    </div>
                </article>
            </div>
        </div>
    }
}

/// Research post page properties
#[derive(Properties, PartialEq)]
pub struct ResearchPostProps {
    pub slug: String,
}

/// Individual research post page component
#[function_component(ResearchPostPage)]
pub fn research_post_page(props: &ResearchPostProps) -> Html {
    // Find the research post by slug
    let post = RESEARCH_POSTS.iter().find(|p| p.slug == props.slug);

    let Some(post) = post else {
        return html! {
            <div class="app research-page">
                <div class="research-content">
                    <nav class="research-nav">
                        <Link<Route> to={Route::Research} classes="back-link">
                            {"← Back to Research"}
                        </Link<Route>>
                    </nav>
                    <article class="research-article">
                        <h1 class="research-title">{"Post Not Found"}</h1>
                    </article>
                </div>
            </div>
        };
    };

    html! {
        <div class="app research-page">
            <div class="research-content">
                <nav class="research-nav">
                    <Link<Route> to={Route::Research} classes="back-link">
                        {"← Back to Research"}
                    </Link<Route>>
                </nav>

                <article class="research-article">
                    <header class="research-header">
                        <h1 class="research-title">{post.title}</h1>
                        <div class="research-divider"></div>
                    </header>

                    <section class="research-section">
                        <p>{post.description}</p>

                        if !post.publications.is_empty() {
                            <h2>{"Publications"}</h2>
                            <ul class="publications-list">
                                { for post.publications.iter().map(|pub_item| {
                                    html! {
                                        <li>
                                            <a href={pub_item.url} target="_blank" class="pub-title">{pub_item.title}</a>
                                            <span class="pub-meta">{format!(" — {}", pub_item.meta)}</span>
                                        </li>
                                    }
                                })}
                            </ul>
                        }
                    </section>
                </article>
            </div>
        </div>
    }
}
