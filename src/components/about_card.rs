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

                <nav class="home-links-horizontal" style="display: flex; flex-direction: row; justify-content: space-between; width: 100%;">
                    <a href="https://github.com/parkji30" target="_blank" rel="noopener noreferrer" class="home-link-item">
                        {"Github"}
                    </a>
                    <Link<Route> to={Route::Blog} classes="home-link-item">
                        {"Toronto"}
                    </Link<Route>>
                    <Link<Route> to={Route::Inspirations} classes="home-link-item">
                        {"Learnings"}
                    </Link<Route>>
                </nav>

                <section class="research-section">
                    <h2>{"About"}</h2>
                    <p>
                        {"My motivation for writing this blog/journal is to keep track of my learnings and refer back to it when I've achieved my goal of becoming the best scientific research engineer within my limits."}
                    </p>
                    <p>
                        {"I'm currently a Research Technical Lead at Xanadu, leading the automation team is creating Agentic workflows for accelerating scientific tasks."}
                    </p>
                    <p>
                        {"I live in Toronto, an underrated city that doesn't receive as much appreciation as it should from its residents."}
                    </p>
                </section>
            </article>
        </div>
    }
}

