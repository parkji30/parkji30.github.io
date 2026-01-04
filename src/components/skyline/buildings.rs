use yew::prelude::*;

/// Simple building silhouette (no lit windows)
#[derive(Properties, PartialEq)]
pub struct BuildingProps {
    #[prop_or_default]
    pub class: String,
}

#[function_component(Building)]
fn building(props: &BuildingProps) -> Html {
    let class = format!("building {}", props.class);

    html! {
        <div class={class}></div>
    }
}

/// Left side buildings cluster
#[function_component(BuildingsLeft)]
pub fn buildings_left() -> Html {
    html! {
        <div class="buildings-left">
            <Building class="bld-l1" />
            <Building class="bld-l2" />
            <Building class="bld-l3" />
            <Building class="bld-l4" />
            <Building class="bld-l5" />
            <Building class="bld-l6" />
            <Building class="bld-l7" />
            <Building class="bld-l8" />
            <Building class="bld-l9" />
            <Building class="bld-l10" />
        </div>
    }
}

/// Right side buildings cluster
#[function_component(BuildingsRight)]
pub fn buildings_right() -> Html {
    html! {
        <div class="buildings-right">
            <Building class="bld-r1" />
            <Building class="bld-r2" />
            <Building class="bld-r3" />
            <Building class="bld-r4" />
            <Building class="bld-r5" />
            <Building class="bld-r6" />
            <Building class="bld-r7" />
            <Building class="bld-r8" />
            <Building class="bld-r9" />
            <Building class="bld-r10" />
        </div>
    }
}
