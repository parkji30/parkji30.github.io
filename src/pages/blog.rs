use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Snowfall, TorontoSkyline};
use crate::Route;

/// Blog page component
#[function_component(BlogPage)]
pub fn blog_page() -> Html {
    html! {
        <div class="app research-page">
            <Snowfall />
            <TorontoSkyline />

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

                    <section class="blog-post">
                        <h2 class="blog-post-title">{"Thoughts on Modern Computing"}</h2>
                        <span class="blog-date">{"January 4, 2026"}</span>
                        <p>
                            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."}
                        </p>
                        <p>
                            {"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                        </p>
                    </section>

                    <div class="blog-divider"></div>

                    <section class="blog-post">
                        <h2 class="blog-post-title">{"Winter in Toronto"}</h2>
                        <span class="blog-date">{"December 21, 2025"}</span>
                        <div class="research-image-placeholder">
                            <span>{"[ Winter Photo ]"}</span>
                        </div>
                        <p>
                            {"Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo."}
                        </p>
                        <p>
                            {"Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt."}
                        </p>
                    </section>

                    <div class="blog-divider"></div>

                    <section class="blog-post">
                        <h2 class="blog-post-title">{"On Learning Rust"}</h2>
                        <span class="blog-date">{"November 15, 2025"}</span>
                        <p>
                            {"Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem."}
                        </p>
                        <p>
                            {"Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur."}
                        </p>
                    </section>

                    <div class="blog-divider"></div>

                    <section class="blog-post">
                        <h2 class="blog-post-title">{"First Post"}</h2>
                        <span class="blog-date">{"October 1, 2025"}</span>
                        <p>
                            {"At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident."}
                        </p>
                        <div class="research-image-placeholder">
                            <span>{"[ Welcome Image ]"}</span>
                        </div>
                        <p>
                            {"Similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio."}
                        </p>
                    </section>
                </article>
            </div>
        </div>
    }
}
