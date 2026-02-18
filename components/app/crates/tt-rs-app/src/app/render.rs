//! Rendering functions for the app.
//!
//! Uses a z-plane architecture for guaranteed widget stacking order.
//! Each plane is a transparent container with pointer-events: none,
//! allowing clicks to pass through to elements on lower planes.
//! Widgets within each plane have pointer-events: auto to receive events.

use tt_rs_core::WidgetId;
use tt_rs_drag::{CopySource, Draggable, DropEvent, Position};
use tt_rs_ui::{
    Footer, HelpButton, HelpPanel, TextPane, Tooltip, TooltipLayer, TooltipPosition, TutorialMenu,
    UserLevel, UserLevelSelector, WorkspaceButton, WorkspaceMenu, WorkspaceMetadata,
};
use wasm_bindgen::JsCast;
use yew::prelude::*;

use super::callbacks::Callbacks;
use super::ZPlanes;
use crate::box_state::render_box;
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Z-plane indices for guaranteed stacking order.
/// Higher numbers appear on top.
/// Help panel uses CSS z-index: 600-650 (see main.css).
const Z_PLANE_STACKS: i32 = 0;
const Z_PLANE_DROPZONES: i32 = 50; // Drop targets below draggable items
const Z_PLANE_BOXES: i32 = 100;
const Z_PLANE_VALUES: i32 = 200;
const Z_PLANE_AGENTS: i32 = 300;
const Z_PLANE_TOOLS: i32 = 400;
const Z_PLANE_TEXT_PANE: i32 = 450;
const Z_PLANE_TOOLTIPS: i32 = 500;

#[allow(clippy::too_many_arguments)]
pub fn render_app(
    state: &AppState,
    help_open: bool,
    tutorial_open: bool,
    workspace_open: bool,
    user_level: UserLevel,
    cbs: &Callbacks,
    planes: &ZPlanes<'_>,
    workspaces: &[WorkspaceMetadata],
) -> Html {
    html! {
        <div class="workspace">
            <div class="workspace-header">
                <span class="header-title">{"tt-rs - Visual Programming Environment"}</span>
                <WorkspaceButton on_click={cbs.on_workspace_open.clone()} />
                <div class="tutorial-container">
                    <button class="tutorial-button" onclick={cbs.on_tutorial_open.reform(|_| ())}>
                        {"Tutorials"}
                    </button>
                    if tutorial_open {
                        <TutorialMenu on_close={cbs.on_tutorial_close.clone()} />
                    }
                </div>
                <UserLevelSelector level={user_level} on_change={cbs.on_level_change.clone()} />
            </div>
            <HelpButton on_click={cbs.on_help_open.clone()} />
            <HelpPanel is_open={help_open} on_close={cbs.on_help_close.clone()} level={user_level} />
            <WorkspaceMenu
                is_open={workspace_open}
                on_close={cbs.on_workspace_close.clone()}
                on_save={cbs.on_workspace_save.clone()}
                on_load={cbs.on_workspace_load.clone()}
                on_delete={cbs.on_workspace_delete.clone()}
                on_export={cbs.on_workspace_export.clone()}
                on_import={cbs.on_workspace_import.clone()}
                current_level={user_level}
                workspaces={workspaces.to_vec()}
            />
            <div class="workspace-content">
                // Z-plane 0: Copy source stacks (lowest)
                { render_z_plane(Z_PLANE_STACKS, render_copy_sources(&planes.copy_sources, state, &cbs.on_copy_source_click, &cbs.on_move)) }
                // Z-plane 50: Drop zones (below draggable items)
                { render_z_plane(Z_PLANE_DROPZONES, render_widgets(&planes.dropzones, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 100: Boxes
                { render_z_plane(Z_PLANE_BOXES, render_boxes(state, cbs)) }
                // Z-plane 200: Values (numbers, text)
                { render_z_plane(Z_PLANE_VALUES, render_widgets(&planes.values, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 300: Agents (robot, bird, nest, scales)
                { render_z_plane(Z_PLANE_AGENTS, render_widgets(&planes.agents, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 400: Tools (vacuum, wand)
                { render_z_plane(Z_PLANE_TOOLS, render_widgets(&planes.tools, state, &cbs.on_move, &cbs.on_drop)) }
                // Z-plane 450: Text pane (workspace notes)
                { render_z_plane(Z_PLANE_TEXT_PANE, render_text_pane(state, cbs)) }
                // Z-plane 500: Tooltips (highest)
                { render_z_plane(Z_PLANE_TOOLTIPS, html! { <TooltipLayer /> }) }
            </div>
            // Status message bar (shows robot errors, etc.)
            if let Some(ref msg) = state.status_message {
                <div class="status-bar status-error">
                    <span class="status-icon">{"⚠️"}</span>
                    <span class="status-message">{ msg }</span>
                </div>
            }
            // Inspection modal (shows widget details when magnifier is used)
            if let Some(ref content) = state.inspection_modal {
                { render_inspection_modal(content, cbs.on_close_inspection.clone()) }
            }
            <Footer />
        </div>
    }
}

/// Renders a z-plane container with the given z-index.
/// The plane itself is transparent and doesn't capture mouse events,
/// but its children (the widgets) do.
fn render_z_plane(z_index: i32, children: Html) -> Html {
    let style = format!(
        "position: absolute; top: 0; left: 0; width: 100%; height: 100%; z-index: {}; pointer-events: none;",
        z_index
    );
    html! {
        <div class="z-plane" style={style}>
            { children }
        </div>
    }
}

fn render_boxes(state: &AppState, cbs: &Callbacks) -> Html {
    state.boxes.iter().map(|(id, b)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        html! {
            <Draggable widget_id={*id} position={pos} on_move={cbs.on_move.clone()} on_drag_start={cbs.on_box_drag_start.clone()} on_drag_end={cbs.on_box_drag_end.clone()} on_drop={cbs.on_box_drop.clone()}>
                <Tooltip title="Box" description="A container with holes for storing items." hint="Drag items into holes. Drop on number to split. Press 0-9 while dragging to create copy with that many holes." position={TooltipPosition::Right}>
                    { render_box(b, &state.widgets) }
                </Tooltip>
            </Draggable>
        }
    }).collect()
}

fn render_copy_sources(
    srcs: &[(&WidgetId, &WidgetItem)],
    state: &AppState,
    on_click: &Callback<tt_rs_drag::CopySourceClickEvent>,
    on_move: &Callback<(WidgetId, Position)>,
) -> Html {
    srcs.iter().map(|(id, w)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        let tip = w.tooltip_info();
        html! {
            <CopySource widget_id={**id} position={pos} on_click={on_click.clone()} on_move={on_move.clone()}>
                <Tooltip title={tip.title} description={tip.description} hint={tip.hint} position={TooltipPosition::Right}>{ w.render() }</Tooltip>
            </CopySource>
        }
    }).collect()
}

fn render_widgets(
    ws: &[(&WidgetId, &WidgetItem)],
    state: &AppState,
    on_move: &Callback<(WidgetId, Position)>,
    on_drop: &Callback<DropEvent>,
) -> Html {
    ws.iter().map(|(id, w)| {
        let pos = state.positions.get(id).copied().unwrap_or_default();
        let tip = w.tooltip_info();
        html! {
            <Draggable widget_id={**id} position={pos} on_move={on_move.clone()} on_drop={on_drop.clone()}>
                <Tooltip title={tip.title} description={tip.description} hint={tip.hint} position={TooltipPosition::Right}>{ w.render() }</Tooltip>
            </Draggable>
        }
    }).collect()
}

/// Renders the draggable text pane for workspace notes.
fn render_text_pane(state: &AppState, cbs: &Callbacks) -> Html {
    let pos = state.text_pane_position;
    let (width, height) = state.text_pane_size;
    let style = format!(
        "position: absolute; left: {x}px; top: {y}px; pointer-events: auto;",
        x = pos.x,
        y = pos.y
    );

    // Create a wrapper for dragging the text pane
    html! {
        <DraggableTextPane
            position={pos}
            on_move={cbs.on_text_pane_move.clone()}
            style={style}
        >
            <TextPane
                content={state.text_pane_content.clone()}
                on_change={cbs.on_text_pane_change.clone()}
                width={width}
                height={height}
                on_resize={cbs.on_text_pane_resize.clone()}
                title="Workspace Notes"
                on_show_me={cbs.on_show_me.clone()}
                on_reset={cbs.on_reset.clone()}
            />
        </DraggableTextPane>
    }
}

/// Props for draggable text pane wrapper.
#[derive(Properties, Clone, PartialEq)]
struct DraggableTextPaneProps {
    position: Position,
    on_move: Callback<Position>,
    style: String,
    children: Children,
}

/// Draggable wrapper for the text pane (draggable by header).
#[function_component(DraggableTextPane)]
fn draggable_text_pane(props: &DraggableTextPaneProps) -> Html {
    let is_dragging = use_state(|| false);
    let drag_start = use_state(|| Position::new(0.0, 0.0));
    let start_pos = use_state(|| Position::new(0.0, 0.0));
    let current_pos = use_state(|| props.position);

    // Update position when props change
    {
        let current_pos = current_pos.clone();
        let prop_pos = props.position;
        use_effect_with(prop_pos, move |pos| {
            current_pos.set(*pos);
            || ()
        });
    }

    let on_mouse_down = {
        let is_dragging = is_dragging.clone();
        let drag_start = drag_start.clone();
        let start_pos = start_pos.clone();
        let current_pos = current_pos.clone();

        Callback::from(move |e: MouseEvent| {
            // Only start drag on header (check class)
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<web_sys::Element>() {
                    let class = element.class_name();
                    if class.contains("text-pane-header") || class.contains("text-pane-title") {
                        e.prevent_default();
                        is_dragging.set(true);
                        let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);
                        drag_start.set(mouse_pos);
                        start_pos.set(*current_pos);
                    }
                }
            }
        })
    };

    let on_mouse_move = {
        let is_dragging = is_dragging.clone();
        let drag_start = drag_start.clone();
        let start_pos = start_pos.clone();
        let current_pos = current_pos.clone();

        Callback::from(move |e: MouseEvent| {
            if *is_dragging {
                let mouse_pos = Position::new(e.client_x() as f64, e.client_y() as f64);
                let delta_x = mouse_pos.x - drag_start.x;
                let delta_y = mouse_pos.y - drag_start.y;
                let new_pos = Position::new(start_pos.x + delta_x, start_pos.y + delta_y);
                current_pos.set(new_pos);
            }
        })
    };

    let on_mouse_up = {
        let is_dragging = is_dragging.clone();
        let current_pos = current_pos.clone();
        let on_move = props.on_move.clone();

        Callback::from(move |_: MouseEvent| {
            if *is_dragging {
                is_dragging.set(false);
                on_move.emit(*current_pos);
            }
        })
    };

    let style = format!(
        "position: absolute; left: {x}px; top: {y}px; pointer-events: auto;",
        x = current_pos.x,
        y = current_pos.y
    );

    html! {
        <div
            class="draggable-text-pane"
            style={style}
            onmousedown={on_mouse_down}
            onmousemove={on_mouse_move}
            onmouseup={on_mouse_up.clone()}
            onmouseleave={on_mouse_up}
        >
            { for props.children.iter() }
        </div>
    }
}

/// Renders the inspection modal showing widget details.
fn render_inspection_modal(content: &str, on_close: Callback<()>) -> Html {
    // Split content into title (first line) and body (rest)
    let lines: Vec<&str> = content.lines().collect();
    let title = lines.first().copied().unwrap_or("Inspection");
    let body = lines.get(1..).map(|l| l.join("\n")).unwrap_or_default();

    let on_overlay_click = {
        let on_close = on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    let on_close_click = {
        let on_close = on_close.clone();
        Callback::from(move |_: MouseEvent| {
            on_close.emit(());
        })
    };

    // Prevent clicks on modal content from closing
    let on_modal_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class="inspection-modal-overlay" onclick={on_overlay_click}>
            <div class="inspection-modal" onclick={on_modal_click}>
                <div class="inspection-modal-header">
                    <h3 class="inspection-modal-title">{ title }</h3>
                    <button class="inspection-modal-close" onclick={on_close_click}>{ "\u{00D7}" }</button>
                </div>
                <pre class="inspection-modal-content">{ body }</pre>
            </div>
        </div>
    }
}
