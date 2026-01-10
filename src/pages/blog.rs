use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// Blog page component
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

                </article>
            </div>
        </div>
    }
}
