use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Research page component
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

                    <section class="research-section">
                        <h2>{"Computational Astrophysics (My Old Life)"}</h2>
                        <p>
                            {"During my graduate studies at Queen's University, my research focused on the intersection of computational astrophysics and applied mathematics. I used numerical simulations to calibrate the Davis-Chandrasekhar-Fermi (DCF) method, a technique for estimating magnetic field strengths in molecular clouds from dust polarization observations. By analyzing simulated Colliding Flow and Collapsing Cloud environments, I evaluated the accuracy of different DCF variations and identified key factors that influence their reliability."}
                        </p>
                        <ul class="publications-list">
                            <li>
                                <a href="https://qspace.library.queensu.ca/server/api/core/bitstreams/319bb098-1b32-4487-bf2c-b0dae61cc27b/content" target="_blank" class="pub-title">{"\"Calibrating the Davis-Chandrasekhar Fermi Method With Molecular Cloud Simulations\""}</a>
                                <span class="pub-meta">{" — M.Sc. Thesis, Queen's University, 2022"}</span>
                            </li>
                            <li>
                                <a href="https://academic.oup.com/mnras/article/514/2/1575/6604898" target="_blank" class="pub-title">{"\"Calibrating the Davis-Chandrasekhar-Fermi method with numerically simulated clouds\""}</a>
                                <span class="pub-meta">{" — Monthly Notices of the Royal Astronomical Society, 2022"}</span>
                            </li>
                        </ul>
                    </section>
                </article>
            </div>
        </div>
    }
}
