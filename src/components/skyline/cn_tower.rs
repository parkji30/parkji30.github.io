use yew::prelude::*;

/// CN Tower - Toronto's iconic landmark (pixel art style)
#[function_component(CNTower)]
pub fn cn_tower() -> Html {
    html! {
        <div class="cn-tower">
            // Antenna spike
            <div class="cn-antenna"></div>
            // Top observation sphere
            <div class="cn-top-sphere"></div>
            // Upper neck
            <div class="cn-neck-upper"></div>
            // Main observation pod (SkyPod)
            <div class="cn-pod">
                <div class="cn-pod-windows"></div>
            </div>
            // Lower pod section
            <div class="cn-pod-lower"></div>
            // Long neck/shaft
            <div class="cn-shaft"></div>
            // Base structure
            <div class="cn-base"></div>
        </div>
    }
}

