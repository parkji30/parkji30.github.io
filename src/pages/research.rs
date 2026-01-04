use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Snowfall;
use crate::Route;

/// Research page component
#[function_component(ResearchPage)]
pub fn research_page() -> Html {
    html! {
        <div class="app research-page">
            <Snowfall />

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

                    <section class="research-section">
                        <h2>{"Computational Astrophysics"}</h2>
                        <p>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."}
                        </p>
                        <p>
                            {"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium."}
                        </p>
                    </section>

                    <section class="research-section">
                        <h2>{"Machine Learning in Science"}</h2>
                        <div class="research-image-placeholder">
                            <span>{"[ Research Visualization ]"}</span>
                        </div>
                        <p>
                            {"Totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt."}
                        </p>
                        <p>
                            {"Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem."}
                        </p>
                    </section>

                    <section class="research-section">
                        <h2>{"Publications"}</h2>
                        <ul class="publications-list">
                            <li>
                                <span class="pub-title">{"\"Lorem Ipsum Dolor Sit Amet\""}</span>
                                <span class="pub-meta">{" — Journal of Computational Physics, 2024"}</span>
                            </li>
                            <li>
                                <span class="pub-title">{"\"Consectetur Adipiscing Elit\""}</span>
                                <span class="pub-meta">{" — Astrophysical Journal, 2023"}</span>
                            </li>
                            <li>
                                <span class="pub-title">{"\"Sed Do Eiusmod Tempor\""}</span>
                                <span class="pub-meta">{" — Monthly Notices of the Royal Astronomical Society, 2022"}</span>
                            </li>
                        </ul>
                    </section>

                    <section class="research-section">
                        <h2>{"Current Projects"}</h2>
                        <p>
                            {"Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur."}
                        </p>
                        <div class="research-image-placeholder">
                            <span>{"[ Project Diagram ]"}</span>
                        </div>
                        <p>
                            {"At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident."}
                        </p>
                    </section>
                </article>
            </div>
        </div>
    }
}

