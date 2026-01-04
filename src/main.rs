use rand::Rng;
use yew::prelude::*;
use yew_router::prelude::*;

/// Routes for the application
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/research")]
    Research,
    #[at("/blog")]
    Blog,
}

/// A single snowflake with position and properties
#[derive(Clone, PartialEq)]
struct Snowflake {
    id: usize,
    x: f64,
    size: f64,
    duration: f64,
    delay: f64,
    opacity: f64,
    rotation_duration: f64,
}

impl Snowflake {
    fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        // Only ~40% of snowflakes rotate
        let should_rotate = rng.gen_bool(0.4);
        Self {
            id,
            x: rng.gen_range(0.0..100.0),
            size: rng.gen_range(3.0..8.0),
            duration: rng.gen_range(10.0..18.0),
            delay: rng.gen_range(0.0..12.0),
            opacity: rng.gen_range(0.6..1.0),
            // Slower rotation: 4-10 seconds per rotation, or 0 for no rotation
            rotation_duration: if should_rotate {
                rng.gen_range(4.0..10.0)
            } else {
                0.0
            },
        }
    }
}

/// Props for the snowflake component
#[derive(Properties, PartialEq)]
struct SnowflakeProps {
    snowflake: Snowflake,
}

/// Individual snowflake component - uses CSS animation
#[function_component(SnowflakeComponent)]
fn snowflake_component(props: &SnowflakeProps) -> Html {
    let sf = &props.snowflake;
    let class = if sf.rotation_duration > 0.0 {
        "snowflake rotating"
    } else {
        "snowflake"
    };
    let style = format!(
        "left: {}%; width: {}px; height: {}px; opacity: {}; --fall-duration: {}s; --rotation-duration: {}s; animation-delay: {}s;",
        sf.x, sf.size, sf.size, sf.opacity, sf.duration, sf.rotation_duration, sf.delay
    );

    html! {
        <div class={class} style={style}></div>
    }
}

/// Main snowfall component
#[function_component(Snowfall)]
fn snowfall() -> Html {
    // Create snowflakes once on mount
    let snowflakes = use_memo((), |()| (0..60).map(Snowflake::new).collect::<Vec<_>>());

    html! {
        <div class="snowfall-container">
            { for snowflakes.iter().map(|sf| {
                html! { <SnowflakeComponent key={sf.id} snowflake={sf.clone()} /> }
            })}
        </div>
    }
}

/// About Me card component
#[function_component(AboutCard)]
fn about_card() -> Html {
    html! {
        <div class="about-card">
            <h1 class="name">{"James Park"}</h1>
            <nav class="links">
                <a href="https://github.com/parkji30" target="_blank" rel="noopener noreferrer" class="link-item">
                    {"Github"}
                </a>
                <Link<Route> to={Route::Research} classes="link-item">
                    {"Research"}
                </Link<Route>>
                <Link<Route> to={Route::Blog} classes="link-item">
                    {"Blog"}
                </Link<Route>>
            </nav>
        </div>
    }
}

/// CN Tower - Toronto's iconic landmark
#[function_component(CNTower)]
fn cn_tower() -> Html {
    html! {
        <div class="cn-tower">
            <div class="cn-tower-antenna"></div>
            <div class="cn-tower-top-sphere"></div>
            <div class="cn-tower-neck-upper"></div>
            <div class="cn-tower-pod"></div>
            <div class="cn-tower-pod-lower"></div>
            <div class="cn-tower-neck-lower"></div>
            <div class="cn-tower-base"></div>
        </div>
    }
}

/// Toronto Skyline
#[function_component(TorontoSkyline)]
fn toronto_skyline() -> Html {
    html! {
        <div class="skyline-container">
            <CNTower />

            <div class="buildings-left">
                <div class="building building-1"></div>
                <div class="building building-2"></div>
                <div class="building building-3"></div>
                <div class="building building-4"></div>
                <div class="building building-5"></div>
                <div class="building building-6"></div>
                <div class="building building-7"></div>
            </div>

            <div class="buildings-right">
                <div class="building building-8"></div>
                <div class="building building-9"></div>
                <div class="building building-10"></div>
                <div class="building building-11"></div>
                <div class="building building-12"></div>
                <div class="building building-13"></div>
                <div class="building building-14"></div>
                <div class="building building-15"></div>
            </div>
        </div>
    }
}

/// Home page component
#[function_component(HomePage)]
fn home_page() -> Html {
    html! {
        <div class="app">
            <Snowfall />
            <TorontoSkyline />
            <AboutCard />
        </div>
    }
}

/// Research page component
#[function_component(ResearchPage)]
fn research_page() -> Html {
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

/// Blog page component
#[function_component(BlogPage)]
fn blog_page() -> Html {
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

/// Switch between routes
#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Research => html! { <ResearchPage /> },
        Route::Blog => html! { <BlogPage /> },
    }
}

/// Main app component with router
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
