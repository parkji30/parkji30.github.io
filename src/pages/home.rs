use yew::prelude::*;

use crate::components::{AboutCard, Snowfall};

/// Home page component
#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="app">
            <Snowfall />
            <AboutCard />
        </div>
    }
}

