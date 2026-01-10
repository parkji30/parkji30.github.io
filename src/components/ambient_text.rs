use rand::seq::SliceRandom;
use yew::prelude::*;

/// Collection of ambient phrases for the site
const PHRASES: &[&str] = &[
    "Soft snow falling on a cold night in Toronto",
    "Quiet moments between the city lights",
    "Winter whispers through empty streets",
    "The hush before dawn in a sleeping city",
    "Stillness found in the space between thoughts",
    "Gentle flurries dancing past streetlamps",
    "A calm night wrapped in white silence",
    "Footprints disappearing in fresh snow",
];

/// Props for ambient text - route is used to trigger re-randomization
#[derive(Properties, PartialEq)]
pub struct AmbientTextProps {
    /// Current route path - changes trigger new phrase selection
    pub route_key: AttrValue,
}

/// Ambient text component that displays a random phrase
#[function_component(AmbientText)]
pub fn ambient_text(props: &AmbientTextProps) -> Html {
    // Re-select phrase when route changes
    let phrase = use_memo(props.route_key.clone(), |_| {
        let mut rng = rand::thread_rng();
        PHRASES.choose(&mut rng).unwrap_or(&PHRASES[0]).to_string()
    });

    html! {
        <div class="ambient-text">
            { (*phrase).clone() }
        </div>
    }
}
