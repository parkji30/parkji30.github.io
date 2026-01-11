use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Blog post data
struct BlogPostData {
    slug: &'static str,
    title: &'static str,
    date: &'static str,
    description: &'static str,
    photos: &'static [&'static str],
}

const BLOG_POSTS: &[BlogPostData] = &[
    BlogPostData {
        slug: "sauble-beach",
        title: "Sauble Beach",
        date: "January 2025",
        description: "We might not have salt ocean beaches with tropical weather, but our beaches shouldn't be overlooked!\n\nThree hours northwest from the great city of Toronto, off the coast of Lake Huron, lies Sauble Beach.\n\nI'll let the photos speak for themselves.",
        photos: &[
            "/assets/photos/sauble_beach/background.jpeg",
            "/assets/photos/sauble_beach/clear water.jpeg",
            "/assets/photos/sauble_beach/seagull.jpeg",
            "/assets/photos/sauble_beach/us.jpeg",
        ],
    },
];

/// Blog listing page component
#[function_component(BlogPage)]
pub fn blog_page() -> Html {
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
                        <h1 class="research-title">{"Blog"}</h1>
                        <div class="research-divider"></div>
                    </header>

                    <div class="posts-list">
                        { for BLOG_POSTS.iter().map(|post| {
                            html! {
                                <Link<Route> to={Route::BlogPost { slug: post.slug.to_string() }} classes="post-link">
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

/// Blog post page properties
#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub slug: String,
}

/// Individual blog post page component
#[function_component(BlogPostPage)]
pub fn blog_post_page(props: &BlogPostProps) -> Html {
    let selected_index = use_state(|| None::<usize>);

    // Find the blog post by slug
    let post = BLOG_POSTS.iter().find(|p| p.slug == props.slug);

    let Some(post) = post else {
        return html! {
            <div class="app research-page">
                <div class="research-content">
                    <nav class="research-nav">
                        <Link<Route> to={Route::Blog} classes="back-link">
                            {"← Back to Blog"}
                        </Link<Route>>
                    </nav>
                    <article class="research-article">
                        <h1 class="research-title">{"Post Not Found"}</h1>
                    </article>
                </div>
            </div>
        };
    };

    let photos = post.photos;

    let close_lightbox = {
        let selected_index = selected_index.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            selected_index.set(None)
        })
    };

    let open_lightbox = |index: usize| {
        let selected_index = selected_index.clone();
        Callback::from(move |_| selected_index.set(Some(index)))
    };

    let go_prev = {
        let selected_index = selected_index.clone();
        let len = photos.len();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            if let Some(idx) = *selected_index {
                let new_idx = if idx == 0 { len - 1 } else { idx - 1 };
                selected_index.set(Some(new_idx));
            }
        })
    };

    let go_next = {
        let selected_index = selected_index.clone();
        let len = photos.len();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            if let Some(idx) = *selected_index {
                let new_idx = if idx >= len - 1 { 0 } else { idx + 1 };
                selected_index.set(Some(new_idx));
            }
        })
    };

    html! {
        <div class="app research-page">

            // Lightbox modal
            if let Some(idx) = *selected_index {
                <div class="lightbox-overlay" onclick={close_lightbox.clone()}>
                    <button class="lightbox-nav lightbox-prev" onclick={go_prev}>{"‹"}</button>
                    <img src={photos[idx]} class="lightbox-image" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())} />
                    <button class="lightbox-nav lightbox-next" onclick={go_next}>{"›"}</button>
                    <span class="lightbox-close" onclick={close_lightbox.clone()}>{"×"}</span>
                </div>
            }

            <div class="research-content">
                <nav class="research-nav">
                    <Link<Route> to={Route::Blog} classes="back-link">
                        {"← Back to Blog"}
                    </Link<Route>>
                </nav>

                <article class="research-article">
                    <header class="research-header">
                        <h1 class="research-title">{post.title}</h1>
                        <div class="research-divider"></div>
                    </header>

                    <section class="blog-post">
                        <p>{post.description}</p>
                        <div class="blog-photo-grid">
                            { for photos.iter().enumerate().map(|(i, src)| {
                                html! {
                                    <img src={*src} alt={format!("{} photo", post.title)} class="blog-photo" onclick={open_lightbox(i)} />
                                }
                            })}
                        </div>
                    </section>
                </article>
            </div>
        </div>
    }
}
