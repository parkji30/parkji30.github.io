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
    spin_duration: f64,
    drift: f64,
    counter_drift: f64,
    blur: f64,
    resting_y: f64,
    depth_class: &'static str,
}

impl Snowflake {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        let depth = rng.gen_range(0.0..1.0);
        let (depth_class, size, duration, opacity, blur) = if depth < 0.5 {
            (
                "snowflake--far",
                rng.gen_range(1.5..3.0),
                rng.gen_range(26.0..36.0),
                rng.gen_range(0.18..0.3),
                rng.gen_range(0.4..0.9),
            )
        } else if depth < 0.85 {
            (
                "snowflake--mid",
                rng.gen_range(2.5..5.0),
                rng.gen_range(20.0..29.0),
                rng.gen_range(0.3..0.48),
                rng.gen_range(0.1..0.45),
            )
        } else {
            (
                "snowflake--near",
                rng.gen_range(4.5..7.5),
                rng.gen_range(15.0..23.0),
                rng.gen_range(0.45..0.65),
                rng.gen_range(0.0..0.25),
            )
        };
        let drift = rng.gen_range(-32.0..32.0);

        Self {
            id,
            x: rng.gen_range(0.0..100.0),
            size,
            duration,
            delay: rng.gen_range(-duration..0.0),
            opacity,
            spin_duration: rng.gen_range(8.0..15.0),
            drift,
            counter_drift: drift * -0.65,
            blur,
            resting_y: rng.gen_range(2.0..98.0),
            depth_class,
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
    let style = format!(
        "left: {}%; width: {}px; height: {}px; opacity: {}; filter: blur({}px); --fall-duration: {}s; --spin-duration: {}s; --drift: {}px; --counter-drift: {}px; --resting-y: {}vh; animation-delay: {}s;",
        sf.x,
        sf.size,
        sf.size,
        sf.opacity,
        sf.blur,
        sf.duration,
        sf.spin_duration,
        sf.drift,
        sf.counter_drift,
        sf.resting_y,
        sf.delay
    );

    html! {
        <div class={classes!("snowflake", sf.depth_class)} style={style}></div>
    }
}

/// Main snowfall component
#[function_component(Snowfall)]
pub fn snowfall() -> Html {
    // Create snowflakes once on mount
    let snowflakes = use_memo((), |()| (0..48).map(Snowflake::new).collect::<Vec<_>>());

    html! {
        <div class="snowfall-container">
            { for snowflakes.iter().map(|sf| {
                html! { <SnowflakeComponent key={sf.id} snowflake={sf.clone()} /> }
            })}
        </div>
    }
}
