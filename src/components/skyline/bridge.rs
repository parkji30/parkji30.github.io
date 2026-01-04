use yew::prelude::*;

/// Bridge element (left side of skyline)
#[function_component(Bridge)]
pub fn bridge() -> Html {
    html! {
        <div class="bridge">
            <div class="bridge-deck"></div>
            <div class="bridge-arch arch-1"></div>
            <div class="bridge-arch arch-2"></div>
            <div class="bridge-support support-1"></div>
            <div class="bridge-support support-2"></div>
            <div class="bridge-support support-3"></div>
        </div>
    }
}

