use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

/// About Me card component
#[function_component(AboutCard)]
pub fn about_card() -> Html {
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

