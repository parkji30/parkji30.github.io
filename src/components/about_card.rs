use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// About Me card component
#[function_component(AboutCard)]
pub fn about_card() -> Html {
    html! {
        <div class="research-content home-content">
            <article class="research-article">
                <header class="research-header">
                    <h1 class="research-title">{"James Park"}</h1>
                    <div class="research-divider"></div>
                </header>

                <section class="research-section">
                    <h2>{"About"}</h2>
                    <p>
                        {"Welcome to my personal website. I am a researcher and developer interested in computational science and software engineering."}
                    </p>
                </section>

                <section class="research-section">
                    <h2>{"Links"}</h2>
                    <nav class="home-links">
                        <a href="https://github.com/parkji30" target="_blank" rel="noopener noreferrer" class="home-link-item">
                            {"Github"}
                        </a>
                        <Link<Route> to={Route::Research} classes="home-link-item">
                            {"Research"}
                        </Link<Route>>
                        <Link<Route> to={Route::Blog} classes="home-link-item">
                            {"Blog"}
                        </Link<Route>>
                    </nav>
                </section>
            </article>
        </div>
    }
}

