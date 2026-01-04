use rand::Rng;
use yew::prelude::*;

/// A single snowflake with position and properties
#[derive(Clone, PartialEq)]
pub struct Snowflake {
    id: usize,
    x: f64,
    size: f64,
    duration: f64,
    delay: f64,
    opacity: f64,
    rotation_duration: f64,
}

impl Snowflake {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        // Only ~40% of snowflakes rotate
        let should_rotate = rng.gen_bool(0.4);
        Self {
            id,
            x: rng.gen_range(0.0..100.0),
            size: rng.gen_range(3.0..8.0),
            duration: rng.gen_range(10.0..18.0),
            delay: rng.gen_range(0.0..12.0),
            opacity: rng.gen_range(0.6..1.0),
            // Slower rotation: 4-10 seconds per rotation, or 0 for no rotation
            rotation_duration: if should_rotate {
                rng.gen_range(4.0..10.0)
            } else {
                0.0
            },
        }
    }
}

/// Props for the snowflake component
#[derive(Properties, PartialEq)]
struct SnowflakeProps {
    snowflake: Snowflake,
}

/// Individual snowflake component - uses CSS animation
#[function_component(SnowflakeComponent)]
fn snowflake_component(props: &SnowflakeProps) -> Html {
    let sf = &props.snowflake;
    let class = if sf.rotation_duration > 0.0 {
        "snowflake rotating"
    } else {
        "snowflake"
    };
    let style = format!(
        "left: {}%; width: {}px; height: {}px; opacity: {}; --fall-duration: {}s; --rotation-duration: {}s; animation-delay: {}s;",
        sf.x, sf.size, sf.size, sf.opacity, sf.duration, sf.rotation_duration, sf.delay
    );

    html! {
        <div class={class} style={style}></div>
    }
}

/// Main snowfall component
#[function_component(Snowfall)]
pub fn snowfall() -> Html {
    // Create snowflakes once on mount
    let snowflakes = use_memo((), |()| (0..60).map(Snowflake::new).collect::<Vec<_>>());

    html! {
        <div class="snowfall-container">
            { for snowflakes.iter().map(|sf| {
                html! { <SnowflakeComponent key={sf.id} snowflake={sf.clone()} /> }
            })}
        </div>
    }
}

