mod bridge;
mod buildings;
mod cn_tower;
mod lake;
mod trees;

use yew::prelude::*;

use bridge::Bridge;
use buildings::{BuildingsLeft, BuildingsRight};
use cn_tower::CNTower;
use lake::Lake;
use trees::Trees;

/// Toronto Skyline - Pixel art style matching the reference image
#[function_component(TorontoSkyline)]
pub fn toronto_skyline() -> Html {
    html! {
        <div class="skyline-container">
            // Sky gradient background
            <div class="sky-gradient"></div>

            // City buildings layer
            <div class="city-layer">
                <BuildingsLeft />
                <CNTower />
                <BuildingsRight />
            </div>

            // Bridge on the left
            <Bridge />

            // Lake with reflections
            <Lake />

            // Foreground trees and path
            <Trees />
        </div>
    }
}

