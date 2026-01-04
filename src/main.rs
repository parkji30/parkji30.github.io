mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::{BlogPage, HomePage, ResearchPage};

/// Routes for the application
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/research")]
    Research,
    #[at("/blog")]
    Blog,
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
