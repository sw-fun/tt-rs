//! Widget-related callback handlers.

use tt_rs_bird::Bird;
use tt_rs_core::{Widget, WidgetId};
use tt_rs_drag::{CopySourceClickEvent, DropEvent, Position};
use yew::prelude::*;

use crate::ops::{
    handle_bird_drop, handle_box_hole_drop, handle_drop_on_bird, handle_dropzone_drop,
    handle_magnifier_drop, handle_nest_click, handle_nest_drop, handle_number_on_number,
    handle_robot_click, handle_scales_drop, handle_sensor_click, handle_vacuum_drop,
    handle_wand_drop,
};
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle copying from copy sources.
/// Special case: copying a Nest "hatches" a paired Bird.
pub fn create_copy_source(
    state: UseStateHandle<AppState>,
    dirty: UseStateHandle<bool>,
) -> Callback<CopySourceClickEvent> {
    Callback::from(move |e: CopySourceClickEvent| {
        let mut s = (*state).clone();
        let source = s.widgets.get(&e.source_id).cloned();
        let mut made_change = false;

        match source {
            Some(WidgetItem::Number(n)) => {
                let copy = n.copy_number();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Number(copy));
                made_change = true;
            }
            Some(WidgetItem::Nest(n)) => {
                // ToonTalk "hatching": copying a nest creates nest + paired bird
                let nest = n.copy_nest();
                let nest_id = nest.id();
                let bird = Bird::with_nest(nest_id, nest.color().into());
                let bird_id = bird.id();

                // Position nest at click location
                s.positions.insert(nest_id, e.position);
                s.widgets.insert(nest_id, WidgetItem::Nest(nest));

                // Position bird next to nest
                s.positions
                    .insert(bird_id, Position::new(e.position.x + 60.0, e.position.y));
                s.widgets.insert(bird_id, WidgetItem::Bird(bird));

                log::info!("Hatched: Nest {} with Bird {}", nest_id, bird_id);
                made_change = true;
            }
            Some(WidgetItem::Bird(b)) => {
                // Copying a bird alone creates an unpaired bird (sink)
                let copy = b.copy_bird();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Bird(copy));
                made_change = true;
            }
            Some(WidgetItem::Scales(sc)) => {
                let copy = sc.copy_scales();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Scales(copy));
                made_change = true;
            }
            Some(WidgetItem::Vacuum(v)) => {
                let copy = v.copy_vacuum();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Vacuum(copy));
                made_change = true;
            }
            Some(WidgetItem::Wand(w)) => {
                let copy = w.copy_wand();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Wand(copy));
                made_change = true;
            }
            Some(WidgetItem::Magnifier(m)) => {
                let copy = m.copy_magnifier();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Magnifier(copy));
                made_change = true;
            }
            Some(WidgetItem::Robot(r)) => {
                let copy = r.copy_robot();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Robot(copy));
                made_change = true;
            }
            Some(WidgetItem::Text(t)) => {
                let copy = t.copy_text();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Text(copy));
                made_change = true;
            }
            Some(WidgetItem::Sensor(se)) => {
                let copy = se.copy_sensor();
                s.positions.insert(copy.id(), e.position);
                s.widgets.insert(copy.id(), WidgetItem::Sensor(copy));
                made_change = true;
            }
            Some(WidgetItem::DropZone(_)) => {
                // DropZones are not copy sources
            }
            None => {}
        }
        state.set(s);
        if made_change {
            dirty.set(true);
        }
    })
}

pub fn create_move(
    state: UseStateHandle<AppState>,
    _dirty: UseStateHandle<bool>,
) -> Callback<(WidgetId, Position)> {
    Callback::from(move |(id, pos): (WidgetId, Position)| {
        let mut s = (*state).clone();
        s.positions.insert(id, pos);
        state.set(s);
        // Moving widgets doesn't make workspace dirty - it's layout, not content
    })
}

pub fn create_widget_drop(
    state: UseStateHandle<AppState>,
    dirty: UseStateHandle<bool>,
) -> Callback<DropEvent> {
    Callback::from(move |e: DropEvent| {
        let mut s = (*state).clone();
        let id = e.widget_id;
        let (mx, my) = (e.mouse_position.x, e.mouse_position.y);

        // These operations modify content, so they make the workspace dirty
        if handle_robot_click(&mut s, id, &e)
            || handle_sensor_click(&mut s, id, &e)  // Click on sensor to produce number
            || handle_nest_click(&mut s, id, &e)  // Click on nest to take message
            || handle_vacuum_drop(&mut s, id, mx, my, &e)
            || handle_wand_drop(&mut s, id, mx, my, &e)
            || handle_magnifier_drop(&mut s, id, mx, my, &e)  // Inspect widget with magnifier
            || handle_drop_on_bird(&mut s, id, mx, my)  // Drop widget ON bird for delivery
            || handle_bird_drop(&mut s, id, mx, my)
            || handle_nest_drop(&mut s, id, mx, my)
            || handle_scales_drop(&mut s, id, mx, my)
            || handle_dropzone_drop(&mut s, id, mx, my)  // Drop widget ON dropzone for puzzle verification
            || handle_number_on_number(&mut s, id, mx, my)
            || handle_box_hole_drop(&mut s, id, mx, my, &e)
        {
            state.set(s);
            dirty.set(true);
        } else {
            s.positions.insert(id, e.position);
            state.set(s);
            // Just moving doesn't make it dirty
        }
    })
}
