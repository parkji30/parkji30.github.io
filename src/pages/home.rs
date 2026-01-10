use yew::prelude::*;

use crate::components::AboutCard;

/// Home page component
#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <div class="app research-page">
            <AboutCard />
        </div>
    }
}

