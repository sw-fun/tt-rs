//! Rendering functions for widget items.

use tt_rs_core::Widget;
use tt_rs_scales::CompareResult;
use yew::prelude::*;

use super::WidgetItem;

/// Render a widget item at full size.
pub fn render(item: &WidgetItem) -> Html {
    match item {
        WidgetItem::Number(n) => n.render(),
        WidgetItem::Text(t) => t.render(),
        WidgetItem::Scales(s) => s.render(),
        WidgetItem::Sensor(s) => s.render(),
        WidgetItem::Vacuum(v) => v.render(),
        WidgetItem::Wand(w) => w.render(),
        WidgetItem::Magnifier(m) => m.render(),
        WidgetItem::Robot(r) => r.render(),
        WidgetItem::Nest(nest) => nest.render(),
        WidgetItem::Bird(bird) => bird.render(),
        WidgetItem::DropZone(dz) => dz.render(),
    }
}

/// Render a smaller version for inside box holes.
pub fn render_small(widget: &WidgetItem) -> Html {
    match widget {
        WidgetItem::Number(n) => render_number_small(n),
        WidgetItem::Text(t) => render_text_small(t),
        WidgetItem::Scales(s) => render_scales_small(s),
        WidgetItem::Sensor(_) => html! { <div class="widget sensor in-hole">{"[sensor]"}</div> },
        WidgetItem::Vacuum(_) => html! { <div class="widget vacuum in-hole">{"[vacuum]"}</div> },
        WidgetItem::Wand(_) => html! { <div class="widget wand in-hole">{"[wand]"}</div> },
        WidgetItem::Magnifier(_) => {
            html! { <div class="widget magnifier in-hole">{"[magnifier]"}</div> }
        }
        WidgetItem::Robot(_) => html! { <div class="widget robot in-hole">{"[robot]"}</div> },
        WidgetItem::Nest(_) => html! { <div class="widget nest in-hole">{"[nest]"}</div> },
        WidgetItem::Bird(_) => html! { <div class="widget bird in-hole">{"[bird]"}</div> },
        WidgetItem::DropZone(_) => {
            html! { <div class="widget dropzone in-hole">{"[dropzone]"}</div> }
        }
    }
}

fn render_number_small(n: &tt_rs_number::Number) -> Html {
    // Only show operator for tools (non-Add operators)
    let show_op = n.is_tool();
    html! {
        <div class="widget number in-hole">
            if show_op {
                <span class="number-operator">{ n.operator().symbol() }</span>
            }
            <span class="number-value">{ n.display_value() }</span>
        </div>
    }
}

fn render_text_small(t: &tt_rs_text::Text) -> Html {
    html! {
        <div class="widget text in-hole">
            <span class="text-value">{ format!("\"{}\"", t.value()) }</span>
        </div>
    }
}

fn render_scales_small(s: &tt_rs_scales::Scales) -> Html {
    let (class_modifier, image_src) = scales_render_info(s.result());
    html! {
        <div class={format!("widget scales in-hole {class_modifier}")}>
            <img src={image_src} alt="scales" class="scales-image-small" />
        </div>
    }
}

fn scales_render_info(result: CompareResult) -> (&'static str, &'static str) {
    match result {
        CompareResult::Indeterminate => ("wobbling", "images/tt-scales.svg"),
        CompareResult::Balanced => ("balanced", "images/tt-scales.svg"),
        CompareResult::LeftHeavier => ("left-heavy", "images/tt-scales-left.svg"),
        CompareResult::RightHeavier => ("right-heavy", "images/tt-scales-right.svg"),
    }
}
