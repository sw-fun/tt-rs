//! Tutorial menu component with level-based submenus.

use yew::prelude::*;

/// Properties for the TutorialMenu component.
#[derive(Properties, Clone, PartialEq)]
pub struct TutorialMenuProps {
    /// Callback when menu is closed (user selected a tutorial or clicked away).
    #[prop_or_default]
    pub on_close: Callback<()>,
}

/// Tutorial menu with tt1/tt2/tt3 submenus.
#[function_component(TutorialMenu)]
pub fn tutorial_menu(props: &TutorialMenuProps) -> Html {
    let expanded_level = use_state(|| Option::<&'static str>::None);

    let toggle_level = {
        let expanded_level = expanded_level.clone();
        Callback::from(move |level: &'static str| {
            expanded_level.set(if *expanded_level == Some(level) {
                None
            } else {
                Some(level)
            });
        })
    };

    let on_tutorial_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="tutorial-menu">
            <div class="tutorial-menu-header">{ "Tutorials" }</div>

            // tt1 - Basic Tutorials
            <div class="tutorial-submenu">
                <button
                    class={classes!("tutorial-level-btn", (*expanded_level == Some("tt1")).then_some("expanded"))}
                    onclick={let toggle = toggle_level.clone(); Callback::from(move |_| toggle.emit("tt1"))}
                >
                    { "tt1 - Basic" }
                    <span class="arrow">{ if *expanded_level == Some("tt1") { "▼" } else { "▶" } }</span>
                </button>
                if *expanded_level == Some("tt1") {
                    <div class="tutorial-items">
                        <a href="#/tutorial/fill-box" onclick={on_tutorial_click.clone()}>{ "Fill the Box" }</a>
                        <a href="#/tutorial/add-numbers" onclick={on_tutorial_click.clone()}>{ "Add Numbers" }</a>
                        <a href="#/tutorial/copy-widget" onclick={on_tutorial_click.clone()}>{ "Copy Widgets" }</a>
                        <a href="#/tutorial/train-robot" onclick={on_tutorial_click.clone()}>{ "Train a Robot" }</a>
                    </div>
                }
            </div>

            // tt2 - Messaging Tutorials
            <div class="tutorial-submenu">
                <button
                    class={classes!("tutorial-level-btn", (*expanded_level == Some("tt2")).then_some("expanded"))}
                    onclick={let toggle = toggle_level.clone(); Callback::from(move |_| toggle.emit("tt2"))}
                >
                    { "tt2 - Messaging" }
                    <span class="arrow">{ if *expanded_level == Some("tt2") { "▼" } else { "▶" } }</span>
                </button>
                if *expanded_level == Some("tt2") {
                    <div class="tutorial-items">
                        <span class="coming-soon">{ "(Coming soon)" }</span>
                    </div>
                }
            </div>

            // tt3 - Sensor & Robot Tutorials
            <div class="tutorial-submenu">
                <button
                    class={classes!("tutorial-level-btn", (*expanded_level == Some("tt3")).then_some("expanded"))}
                    onclick={let toggle = toggle_level.clone(); Callback::from(move |_| toggle.emit("tt3"))}
                >
                    { "tt3 - Sensors" }
                    <span class="arrow">{ if *expanded_level == Some("tt3") { "▼" } else { "▶" } }</span>
                </button>
                if *expanded_level == Some("tt3") {
                    <div class="tutorial-items">
                        <a href="#/tutorial/train-robot" onclick={on_tutorial_click.clone()}>{ "Train a Robot" }</a>
                        <a href="#/tutorial/robot-sensor" onclick={on_tutorial_click.clone()}>{ "Robot with Sensors" }</a>
                    </div>
                }
            </div>
        </div>
    }
}
