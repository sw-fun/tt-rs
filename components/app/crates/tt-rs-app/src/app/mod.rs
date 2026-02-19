//! Main application component.

mod callbacks;
mod render;

use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use tt_rs_ui::{ConfirmDialog, DemoCursor, TooltipLayerProvider, UserLevel, WorkspaceMetadata};
use wasm_bindgen::{closure::Closure, JsCast};
use yew::prelude::*;

use crate::demo_runner::DemoState;
use crate::routing::{current_route, Route};
use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Pending action that requires user confirmation.
#[derive(Clone, PartialEq)]
pub enum PendingAction {
    /// Reset the workspace (reload current puzzle/tutorial or sandbox default)
    Reset,
    /// Change to a different user level
    LevelChange(UserLevel),
}

/// Load puzzle/tutorial based on route.
/// Returns (AppState, UserLevel) if found.
fn load_route(route: &Route) -> Option<(AppState, UserLevel)> {
    match route {
        Route::Puzzle(id) => {
            // Try multiple variations of the puzzle ID
            let variations = [
                id.clone(),                                   // exact: fill-a-box
                format!("puzzle-{id}"),                       // with prefix: puzzle-fill-a-box
                id.replace("-a-", "-"),                       // normalized: fill-box
                format!("puzzle-{}", id.replace("-a-", "-")), // puzzle-fill-box
            ];

            for puzzle_id in &variations {
                if let Some(workspace) = crate::workspace::load_bundled_puzzle(puzzle_id) {
                    log::info!("Loaded puzzle from URL: {}", puzzle_id);
                    let level = crate::workspace::workspace_user_level(&workspace);
                    return Some((crate::workspace::from_workspace(&workspace), level));
                }
            }

            log::warn!("Puzzle not found: {} (tried: {:?})", id, variations);
            None
        }
        Route::Tutorial(id) => {
            // Try multiple variations of the tutorial ID
            let variations = [
                format!("tutorial-{id}"), // with prefix: tutorial-fill-box
                id.clone(),               // exact: fill-box
                format!("tutorial-{}", id.replace("-a-", "-")), // normalized
            ];

            for tutorial_id in &variations {
                if let Some(workspace) = crate::workspace::load_bundled_puzzle(tutorial_id) {
                    log::info!("Loaded tutorial from URL: {}", tutorial_id);
                    let level = crate::workspace::workspace_user_level(&workspace);
                    return Some((crate::workspace::from_workspace(&workspace), level));
                }
            }

            log::warn!("Tutorial not found: {} (tried: {:?})", id, variations);
            None
        }
        Route::Sandbox => None,
    }
}

/// Main application component.
#[function_component(App)]
pub fn app() -> Html {
    // Load initial state and user level together from URL
    let (initial_state, initial_level) = {
        let route = current_route();
        load_route(&route).unwrap_or_else(|| (AppState::default(), UserLevel::default()))
    };

    let state = use_state(|| initial_state);
    let help_open = use_state(|| false);
    let workspace_open = use_state(|| false);
    let tutorial_open = use_state(|| false);
    let user_level = use_state(|| initial_level);
    let dragged_box_id = use_mut_ref(|| None::<WidgetId>);
    let pending_new_box = use_mut_ref(|| None::<usize>);

    // Track whether the workspace has been modified
    let dirty = use_state(|| false);

    // Track pending action that needs confirmation
    let pending_action: UseStateHandle<Option<PendingAction>> = use_state(|| None);

    // Demo animation state
    let demo_state = use_state(DemoState::default);

    // Demo animation effect - runs when demo is playing
    // This effect handles both cursor animation AND actual widget operations
    {
        let demo_state = demo_state.clone();
        let app_state = state.clone();
        let dirty_for_demo = dirty.clone();
        let ds_clone = (*demo_state).clone();
        use_effect_with(ds_clone.clone(), move |ds| {
            // Store timeout handle for cleanup (None if no timeout scheduled)
            let cleanup_handle: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(None));

            if ds.is_playing && ds.step_index < ds.steps.len() {
                log::info!(
                    "Demo effect: playing step {}/{}",
                    ds.step_index,
                    ds.steps.len()
                );

                let delay = crate::demo_runner::get_step_delay(ds);
                let ds_for_timeout = ds.clone();
                let demo_state_for_timeout = demo_state.clone();
                let app_state_for_timeout = app_state.clone();
                let dirty_for_timeout = dirty_for_demo.clone();

                let window = web_sys::window().unwrap();
                let closure = Closure::once(Box::new(move || {
                    // Process demo step for cursor animation
                    if let Some(mut new_demo_state) =
                        crate::demo_runner::process_next_step(&ds_for_timeout)
                    {
                        log::info!(
                            "Demo: processed step, cursor at ({}, {}), dragging={}",
                            new_demo_state.cursor_x,
                            new_demo_state.cursor_y,
                            new_demo_state.is_dragging
                        );

                        // Handle actual widget operations based on current step
                        let step_index = ds_for_timeout.step_index;
                        if step_index < ds_for_timeout.steps.len() {
                            let step = &ds_for_timeout.steps[step_index];
                            match step {
                                crate::workspace::DemoStep::DragStart => {
                                    // Find widget at cursor position using hit testing
                                    let cursor_x = ds_for_timeout.cursor_x;
                                    let cursor_y = ds_for_timeout.cursor_y;
                                    if let Some((widget_id, is_box)) =
                                        crate::demo_ops::find_widget_at(cursor_x, cursor_y)
                                    {
                                        log::info!(
                                            "Demo DragStart: found widget {:?} (is_box={})",
                                            widget_id,
                                            is_box
                                        );

                                        // If widget is a copy source, create a copy first
                                        let mut new_app_state = (*app_state_for_timeout).clone();
                                        let drag_id = if let Some(widget) =
                                            new_app_state.widgets.get(&widget_id)
                                        {
                                            if widget.is_copy_source() {
                                                // Create a copy of the widget
                                                let copy = crate::demo_ops::copy_widget(
                                                    widget, &widget_id,
                                                );
                                                let copy_id = copy.id();
                                                let pos = new_app_state
                                                    .positions
                                                    .get(&widget_id)
                                                    .copied()
                                                    .unwrap_or_default();
                                                new_app_state.positions.insert(copy_id, pos);
                                                new_app_state.widgets.insert(copy_id, copy);
                                                app_state_for_timeout.set(new_app_state);
                                                log::info!(
                                                    "Demo DragStart: created copy {:?} from copy source {:?}",
                                                    copy_id,
                                                    widget_id
                                                );
                                                copy_id
                                            } else {
                                                widget_id
                                            }
                                        } else {
                                            widget_id
                                        };

                                        new_demo_state.dragged_widget_id = Some(drag_id);
                                        new_demo_state.dragged_is_box = is_box;
                                    }
                                }
                                crate::workspace::DemoStep::MoveTo { x, y, .. } => {
                                    // If dragging a widget, update its position
                                    if new_demo_state.is_dragging {
                                        if let Some(widget_id) = new_demo_state.dragged_widget_id {
                                            let mut new_app_state =
                                                (*app_state_for_timeout).clone();
                                            let pos = tt_rs_drag::Position::new(*x, *y);
                                            new_app_state.positions.insert(widget_id, pos);
                                            app_state_for_timeout.set(new_app_state);
                                            log::info!(
                                                "Demo MoveTo: moved widget {:?} to ({}, {})",
                                                widget_id,
                                                x,
                                                y
                                            );
                                        }
                                    }
                                }
                                crate::workspace::DemoStep::DragEnd => {
                                    // Perform drop operation
                                    if let Some(widget_id) = ds_for_timeout.dragged_widget_id {
                                        let cursor_x = new_demo_state.cursor_x;
                                        let cursor_y = new_demo_state.cursor_y;
                                        log::info!(
                                            "Demo DragEnd: dropping widget {:?} at ({}, {})",
                                            widget_id,
                                            cursor_x,
                                            cursor_y
                                        );
                                        crate::demo_ops::perform_drop(
                                            &app_state_for_timeout,
                                            &dirty_for_timeout,
                                            widget_id,
                                            ds_for_timeout.dragged_is_box,
                                            cursor_x,
                                            cursor_y,
                                        );
                                    }
                                    new_demo_state.dragged_widget_id = None;
                                    new_demo_state.dragged_is_box = false;
                                }
                                crate::workspace::DemoStep::Wait { .. } => {
                                    // No widget operation needed
                                }
                                crate::workspace::DemoStep::MoveToTarget { .. } => {
                                    // Should be resolved before playback - skip
                                    log::warn!("MoveToTarget not resolved - skipping");
                                }
                                crate::workspace::DemoStep::MoveRelative { .. } => {
                                    // If dragging a widget, update its position based on cursor
                                    if new_demo_state.is_dragging {
                                        if let Some(widget_id) = new_demo_state.dragged_widget_id {
                                            let mut new_app_state =
                                                (*app_state_for_timeout).clone();
                                            // Use the updated cursor position from demo_runner
                                            let x = new_demo_state.cursor_x;
                                            let y = new_demo_state.cursor_y
                                                - crate::demo_runner::WORKSPACE_OFFSET_Y;
                                            let pos = tt_rs_drag::Position::new(x, y);
                                            new_app_state.positions.insert(widget_id, pos);
                                            app_state_for_timeout.set(new_app_state);
                                            log::info!(
                                                "Demo MoveRelative: moved widget {:?} to ({}, {})",
                                                widget_id,
                                                x,
                                                y
                                            );
                                        }
                                    }
                                }
                                crate::workspace::DemoStep::Click => {
                                    // Perform click operation at cursor position
                                    let cursor_x = ds_for_timeout.cursor_x;
                                    let cursor_y = ds_for_timeout.cursor_y;
                                    log::info!(
                                        "Demo Click: clicking at ({}, {})",
                                        cursor_x,
                                        cursor_y
                                    );
                                    crate::demo_ops::perform_click(
                                        &app_state_for_timeout,
                                        &dirty_for_timeout,
                                        cursor_x,
                                        cursor_y,
                                    );
                                }
                            }
                        }

                        demo_state_for_timeout.set(new_demo_state);
                    }
                }) as Box<dyn FnOnce()>);

                let handle = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        delay as i32,
                    )
                    .unwrap();
                closure.forget();

                *cleanup_handle.borrow_mut() = Some(handle);
            }

            // Single cleanup closure - handles both cases
            let cleanup_handle_clone = cleanup_handle.clone();
            move || {
                if let Some(h) = cleanup_handle_clone.borrow_mut().take() {
                    if let Some(window) = web_sys::window() {
                        window.clear_timeout_with_handle(h);
                    }
                }
            }
        });
    }

    // Set up hashchange listener for URL navigation
    {
        let state = state.clone();
        let dirty = dirty.clone();
        let user_level = user_level.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let cb = Closure::wrap(Box::new(move || {
                let route = current_route();
                if let Some((new_state, new_level)) = load_route(&route) {
                    state.set(new_state);
                    user_level.set(new_level);
                    dirty.set(false); // Fresh load is not dirty
                }
            }) as Box<dyn FnMut()>);
            window
                .add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref())
                .unwrap();
            let w = window.clone();
            let c = cb;
            move || {
                let _ =
                    w.remove_event_listener_with_callback("hashchange", c.as_ref().unchecked_ref());
            }
        });
    }

    // Set up keydown listener using use_effect_with directly in the component
    // This ensures the hook is properly registered with Yew's hook system
    {
        let dragged = dragged_box_id.clone();
        let pending = pending_new_box.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();
            let cb = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
                if dragged.borrow().is_some() {
                    if let Some(c) = e.key().chars().next().filter(|c| c.is_ascii_digit()) {
                        *pending.borrow_mut() = Some(c.to_digit(10).unwrap() as usize);
                        e.prevent_default();
                    }
                }
            }) as Box<dyn FnMut(_)>);
            window
                .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
                .unwrap();
            let w = window.clone();
            let c = cb;
            move || {
                let _ =
                    w.remove_event_listener_with_callback("keydown", c.as_ref().unchecked_ref());
            }
        });
    }

    let cbs = callbacks::create_callbacks(callbacks::CallbackConfig {
        state: state.clone(),
        help_open: help_open.clone(),
        tutorial_open: tutorial_open.clone(),
        user_level: user_level.clone(),
        workspace_open: workspace_open.clone(),
        dragged_box_id: dragged_box_id.clone(),
        pending_new_box: pending_new_box.clone(),
        dirty: dirty.clone(),
        pending_action: pending_action.clone(),
        demo_state: demo_state.clone(),
    });

    let planes = partition_into_planes(&state, *user_level);

    // Placeholder workspace list - will be populated from storage in Part 2
    let workspaces: Vec<WorkspaceMetadata> = vec![];

    // Confirmation dialog callbacks
    let on_confirm = {
        let pending_action = pending_action.clone();
        let state = state.clone();
        let dirty = dirty.clone();
        let user_level = user_level.clone();
        Callback::from(move |_| {
            if let Some(action) = (*pending_action).clone() {
                match action {
                    PendingAction::Reset => {
                        // Get the current route at reset time (not at callback creation time)
                        let current_id = match current_route() {
                            Route::Puzzle(id) => {
                                // Don't double-prefix if ID already has a prefix
                                if id.starts_with("puzzle-") || id.starts_with("tutorial-") {
                                    Some(id)
                                } else {
                                    Some(format!("puzzle-{id}"))
                                }
                            }
                            Route::Tutorial(id) => {
                                if id.starts_with("tutorial-") {
                                    Some(id)
                                } else {
                                    Some(format!("tutorial-{id}"))
                                }
                            }
                            Route::Sandbox => None,
                        };
                        log::info!("Reset confirm: current_id = {:?}", current_id);

                        // Reset to initial state
                        if let Some(ref id) = current_id {
                            // Reload puzzle/tutorial
                            if let Some(workspace) = crate::workspace::load_bundled_puzzle(id) {
                                log::info!("Loaded workspace for reset: {}", id);
                                let new_state = crate::workspace::from_workspace(&workspace);
                                state.set(new_state);
                            } else {
                                log::warn!("Failed to load workspace: {}", id);
                            }
                        } else {
                            // Sandbox mode - reset to default state
                            log::info!("Reset to sandbox default state");
                            state.set(AppState::new());
                        }
                        dirty.set(false);
                    }
                    PendingAction::LevelChange(level) => {
                        user_level.set(level);
                        // Reset to default state for the new level
                        let mut new_state = AppState::new();
                        new_state.text_pane_content =
                            crate::state::default_notes_for_level(level).to_string();
                        state.set(new_state);
                        dirty.set(false);
                        // Update URL to sandbox
                        crate::routing::set_route(&Route::Sandbox);
                    }
                }
                pending_action.set(None);
            }
        })
    };

    let on_cancel = {
        let pending_action = pending_action.clone();
        Callback::from(move |_| {
            pending_action.set(None);
        })
    };

    // Generate dialog message based on pending action
    let dialog_message = (*pending_action).as_ref().map(|action| match action {
        PendingAction::Reset => {
            "You have unsaved changes. Reset will discard all changes. Continue?"
        }
        PendingAction::LevelChange(_) => {
            "You have unsaved changes. Changing level will discard all changes. Continue?"
        }
    });

    // Get demo state values for rendering
    let ds = (*demo_state).clone();

    html! {
        <TooltipLayerProvider>
            { render::render_app(&state, *help_open, *tutorial_open, *workspace_open, *user_level, &cbs, &planes, &workspaces) }
            if let Some(message) = dialog_message {
                <ConfirmDialog
                    title="Discard Changes?"
                    message={message}
                    confirm_label="Discard"
                    cancel_label="Keep Working"
                    on_confirm={on_confirm}
                    on_cancel={on_cancel}
                />
            }
            // Show demo cursor when demo is playing
            if ds.is_playing {
                <DemoCursor
                    x={ds.cursor_x}
                    y={ds.cursor_y}
                    is_dragging={ds.is_dragging}
                    transition_duration={ds.transition_ms}
                />
            }
        </TooltipLayerProvider>
    }
}

type WidgetRefs<'a> = Vec<(&'a WidgetId, &'a WidgetItem)>;

/// Widgets partitioned into z-planes for guaranteed stacking order.
pub struct ZPlanes<'a> {
    /// Plane 0: Copy source stacks (lowest z-index)
    pub copy_sources: WidgetRefs<'a>,
    /// Plane 0.5: Drop zones (below draggable items)
    pub dropzones: WidgetRefs<'a>,
    /// Plane 1: Values (numbers, text)
    pub values: WidgetRefs<'a>,
    /// Plane 2: Agents and comparison (robot, bird, nest, scales)
    pub agents: WidgetRefs<'a>,
    /// Plane 3: Tools (vacuum, wand) - highest z-index for regular widgets
    pub tools: WidgetRefs<'a>,
}

fn partition_into_planes(state: &AppState, level: UserLevel) -> ZPlanes<'_> {
    let is_visible = |w: &WidgetItem| -> bool {
        match level {
            UserLevel::Tt1 => !matches!(
                w,
                WidgetItem::Bird(_)
                    | WidgetItem::Nest(_)
                    | WidgetItem::Sensor(_)
                    | WidgetItem::Magnifier(_)
            ),
            UserLevel::Tt2 => !matches!(w, WidgetItem::Sensor(_)),
            UserLevel::Tt3 => true,
        }
    };

    let not_in_box = |id: &WidgetId| !state.widget_in_box.contains_key(id);

    let copy_sources: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| not_in_box(id) && is_visible(w) && w.is_copy_source())
        .collect();

    let dropzones: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(w, WidgetItem::DropZone(_))
        })
        .collect();

    let values: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(w, WidgetItem::Number(_) | WidgetItem::Text(_))
        })
        .collect();

    let agents: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(
                    w,
                    WidgetItem::Robot(_)
                        | WidgetItem::Bird(_)
                        | WidgetItem::Nest(_)
                        | WidgetItem::Scales(_)
                )
        })
        .collect();

    let tools: Vec<_> = state
        .widgets
        .iter()
        .filter(|(id, w)| {
            not_in_box(id)
                && is_visible(w)
                && !w.is_copy_source()
                && matches!(
                    w,
                    WidgetItem::Vacuum(_)
                        | WidgetItem::Wand(_)
                        | WidgetItem::Magnifier(_)
                        | WidgetItem::Sensor(_)
                )
        })
        .collect();

    ZPlanes {
        copy_sources,
        dropzones,
        values,
        agents,
        tools,
    }
}
