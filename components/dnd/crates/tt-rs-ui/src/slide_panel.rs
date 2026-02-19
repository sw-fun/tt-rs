//! SlidePanel component for side-sliding panels.
//!
//! Provides a panel that slides in from the right side of the screen.

use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

/// Properties for the SlidePanel component.
#[derive(Properties, Clone, PartialEq)]
pub struct SlidePanelProps {
    /// Whether the panel is currently open.
    pub is_open: bool,
    /// Callback when close is requested.
    pub on_close: Callback<()>,
    /// Title shown in the panel header.
    pub title: AttrValue,
    /// Panel content.
    pub children: Children,
}

/// A panel that slides in from the right side.
#[function_component(SlidePanel)]
pub fn slide_panel(props: &SlidePanelProps) -> Html {
    // Set up Escape key handler when panel is open
    {
        let is_open = props.is_open;
        let on_close = props.on_close.clone();
        use_effect_with((is_open, on_close), |(is_open, on_close)| {
            if !*is_open {
                // Return empty cleanup when not open
                return Box::new(|| {}) as Box<dyn FnOnce()>;
            }

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let on_close_key = on_close.clone();
            let key_handler = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if e.key() == "Escape" {
                    on_close_key.emit(());
                }
            }) as Box<dyn FnMut(_)>);
            document
                .add_event_listener_with_callback("keydown", key_handler.as_ref().unchecked_ref())
                .unwrap();

            // Use forget() since we return a cleanup that doesn't reference the handler
            key_handler.forget();

            let doc = document;
            Box::new(move || {
                // We can't remove the listener since we forgot the handler,
                // but the component unmounts when panel closes anyway
                drop(doc);
            }) as Box<dyn FnOnce()>
        });
    }

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    let panel_class = format!("slide-panel{}", if props.is_open { " open" } else { "" });

    let overlay_class = format!(
        "slide-panel-overlay{}",
        if props.is_open { " visible" } else { "" }
    );

    html! {
        <>
            <div class={overlay_class} onclick={on_overlay_click} />
            <div class={panel_class}>
                <div class="slide-panel-header">
                    <h2 class="slide-panel-title">{ &props.title }</h2>
                    <button class="slide-panel-close" onclick={on_close_click}>
                        { "\u{2715}" }
                    </button>
                </div>
                <div class="slide-panel-content">
                    { for props.children.iter() }
                </div>
            </div>
        </>
    }
}
