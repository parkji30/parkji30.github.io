use yew::prelude::*;

/// Building component with lit windows
#[derive(Properties, PartialEq)]
pub struct BuildingProps {
    #[prop_or_default]
    pub class: String,
    #[prop_or(5)]
    pub window_cols: u32,
    #[prop_or(8)]
    pub window_rows: u32,
}

#[function_component(Building)]
fn building(props: &BuildingProps) -> Html {
    let class = format!("building {}", props.class);

    // Generate windows with random lit pattern
    let windows: Vec<Html> = (0..props.window_rows)
        .flat_map(|row| {
            (0..props.window_cols).map(move |col| {
                // Use deterministic pattern based on position for consistency
                let is_lit = ((row * 7 + col * 13) % 3) != 0;
                let window_class = if is_lit { "window lit" } else { "window" };
                html! {
                    <div class={window_class} key={format!("{}-{}", row, col)}></div>
                }
            })
        })
        .collect();

    html! {
        <div class={class}>
            <div class="building-body">
                <div class="windows-grid" style={format!("--cols: {}; --rows: {}", props.window_cols, props.window_rows)}>
                    { for windows }
                </div>
            </div>
        </div>
    }
}

/// Left side buildings cluster
#[function_component(BuildingsLeft)]
pub fn buildings_left() -> Html {
    html! {
        <div class="buildings-left">
            <Building class="bld-l1" window_cols={3} window_rows={12} />
            <Building class="bld-l2" window_cols={4} window_rows={18} />
            <Building class="bld-l3" window_cols={5} window_rows={22} />
            <Building class="bld-l4" window_cols={4} window_rows={16} />
            <Building class="bld-l5" window_cols={6} window_rows={25} />
            <Building class="bld-l6" window_cols={5} window_rows={20} />
            <Building class="bld-l7" window_cols={4} window_rows={14} />
            <Building class="bld-l8" window_cols={6} window_rows={24} />
            <Building class="bld-l9" window_cols={5} window_rows={18} />
            <Building class="bld-l10" window_cols={4} window_rows={15} />
        </div>
    }
}

/// Right side buildings cluster
#[function_component(BuildingsRight)]
pub fn buildings_right() -> Html {
    html! {
        <div class="buildings-right">
            <Building class="bld-r1" window_cols={5} window_rows={20} />
            <Building class="bld-r2" window_cols={6} window_rows={26} />
            <Building class="bld-r3" window_cols={4} window_rows={18} />
            <Building class="bld-r4" window_cols={5} window_rows={22} />
            <Building class="bld-r5" window_cols={4} window_rows={16} />
            <Building class="bld-r6" window_cols={6} window_rows={28} />
            <Building class="bld-r7" window_cols={5} window_rows={19} />
            <Building class="bld-r8" window_cols={4} window_rows={14} />
            <Building class="bld-r9" window_cols={5} window_rows={21} />
            <Building class="bld-r10" window_cols={4} window_rows={17} />
        </div>
    }
}

