//! Tutorial menu component with level-based submenus.

use wasm_bindgen::{closure::Closure, JsCast};
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
    let menu_ref = use_node_ref();

    // Set up Escape key handler and click-outside handler
    {
        let on_close = props.on_close.clone();
        let menu_ref = menu_ref.clone();
        use_effect_with(on_close.clone(), move |on_close| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // Escape key handler
            let on_close_key = on_close.clone();
            let key_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Escape" {
                    on_close_key.emit(());
                }
            }) as Box<dyn FnMut(_)>);
            document
                .add_event_listener_with_callback("keydown", key_handler.as_ref().unchecked_ref())
                .unwrap();

            // Click-outside handler
            let on_close_click = on_close.clone();
            let menu_ref_click = menu_ref.clone();
            let click_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                // Check if click was outside the menu
                if let Some(menu) = menu_ref_click.cast::<web_sys::Element>() {
                    if let Some(target) = e.target() {
                        if let Ok(target_node) = target.dyn_into::<web_sys::Node>() {
                            if !menu.contains(Some(&target_node)) {
                                on_close_click.emit(());
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            // Use capture phase to catch clicks before they reach other elements
            document
                .add_event_listener_with_callback_and_bool(
                    "click",
                    click_handler.as_ref().unchecked_ref(),
                    true,
                )
                .unwrap();

            // Cleanup
            let doc = document.clone();
            move || {
                let _ = doc.remove_event_listener_with_callback(
                    "keydown",
                    key_handler.as_ref().unchecked_ref(),
                );
                let _ = doc.remove_event_listener_with_callback_and_bool(
                    "click",
                    click_handler.as_ref().unchecked_ref(),
                    true,
                );
            }
        });
    }

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
        <div class="tutorial-menu" ref={menu_ref}>
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
                        <a href="#/tutorial/bird-nest" onclick={on_tutorial_click.clone()}>{ "Bird & Nest Messaging" }</a>
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
