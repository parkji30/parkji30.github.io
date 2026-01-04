use yew::prelude::*;

/// Foreground trees and greenery (pixel art style)
#[function_component(Trees)]
pub fn trees() -> Html {
    html! {
        <div class="trees-container">
            // Left side trees
            <div class="tree-cluster tree-cluster-left">
                <div class="tree tree-1"></div>
                <div class="tree tree-2"></div>
                <div class="tree tree-3"></div>
                <div class="bush bush-1"></div>
                <div class="bush bush-2"></div>
            </div>

            // Waterfront walkway/path
            <div class="waterfront-path">
                <div class="path-lamp lamp-1"></div>
                <div class="path-lamp lamp-2"></div>
                <div class="path-lamp lamp-3"></div>
                <div class="path-lamp lamp-4"></div>
                <div class="path-lamp lamp-5"></div>
            </div>

            // Right side trees
            <div class="tree-cluster tree-cluster-right">
                <div class="tree tree-4"></div>
                <div class="tree tree-5"></div>
                <div class="bush bush-3"></div>
                <div class="bush bush-4"></div>
            </div>

            // Foreground grass/bushes
            <div class="foreground-foliage">
                <div class="foliage foliage-1"></div>
                <div class="foliage foliage-2"></div>
                <div class="foliage foliage-3"></div>
            </div>
        </div>
    }
}

