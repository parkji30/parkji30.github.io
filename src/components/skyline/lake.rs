use yew::prelude::*;

/// Lake Ontario with subtle reflections
#[function_component(Lake)]
pub fn lake() -> Html {
    html! {
        <div class="lake">
            // Water surface with subtle ripples
            <div class="lake-surface">
                <div class="ripple ripple-1"></div>
                <div class="ripple ripple-2"></div>
                <div class="ripple ripple-3"></div>
                <div class="ripple ripple-4"></div>
                <div class="ripple ripple-5"></div>
            </div>
            // City reflections
            <div class="lake-reflections"></div>
        </div>
    }
}

