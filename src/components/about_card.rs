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
                    <Link<Route> to={Route::Research} classes="home-link-item">
                        {"Research"}
                    </Link<Route>>
                    <Link<Route> to={Route::Blog} classes="home-link-item">
                        {"Blog"}
                    </Link<Route>>
                </nav>

                <section class="research-section">
                    <h2>{"About"}</h2>
                    <p>
                        {"Welcome to my personal website. I am currently a Staff ML Scientist at Xanadu, focusing on Automation and Optimization."}
                    </p>
                    <p>
                        {"I currently lead the Agentic AI team to build the best possible Quantum Computing AI Scientist. On the side, I work closely with some of our physicists to make their software more performant using Jax, NumPy and other high performant packages in Python, and Rust for well defined static computations."}
                    </p>
                    <p>
                        {"I currently live in Toronto, an underrated city that doesn't receive as much appreciation as it should from its residents."}
                    </p>
                    <p>
                        {"This website (i.e. the soft snowfall in the background), along with my blog and research page, was made to represent different aspects of my life."}
                    </p>
                    <p>
                        {"Thank you for dropping by and I hope I can share something meaningful with you."}
                    </p>
                </section>
            </article>
        </div>
    }
}

