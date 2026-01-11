mod components;
mod pages;

use yew::prelude::*;
use yew_router::prelude::*;

use components::{AmbientText, Snowfall};
use pages::{BlogPage, BlogPostPage, HomePage, ResearchPage, ResearchPostPage};

/// Routes for the application
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/research")]
    Research,
    #[at("/research/:slug")]
    ResearchPost { slug: String },
    #[at("/blog")]
    Blog,
    #[at("/blog/:slug")]
    BlogPost { slug: String },
}

/// Switch between routes
#[allow(clippy::needless_pass_by_value)]
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Research => html! { <ResearchPage /> },
        Route::ResearchPost { slug } => html! { <ResearchPostPage slug={slug} /> },
        Route::Blog => html! { <BlogPage /> },
        Route::BlogPost { slug } => html! { <BlogPostPage slug={slug} /> },
    }
}

/// Inner app component that has access to router context
#[function_component(AppContent)]
fn app_content() -> Html {
    let route = use_route::<Route>().unwrap_or(Route::Home);
    let route_key = match route {
        Route::Home => "home",
        Route::Research | Route::ResearchPost { .. } => "research",
        Route::Blog | Route::BlogPost { .. } => "blog",
    };

    html! {
        <>
            <Snowfall />
            <Switch<Route> render={switch} />
            <AmbientText route_key={route_key} />
        </>
    }
}

/// Main app component with router
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppContent />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
