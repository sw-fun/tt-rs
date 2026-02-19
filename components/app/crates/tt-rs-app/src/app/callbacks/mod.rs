//! Callback handler creation.

mod box_handlers;
mod widget_handlers;

use std::cell::RefCell;
use std::rc::Rc;
use tt_rs_core::WidgetId;
use tt_rs_drag::{CopySourceClickEvent, DragEndEvent, DragStartEvent, DropEvent, Position};
use tt_rs_ui::{SaveFormData, UserLevel};
use yew::prelude::*;

use super::PendingAction;
use crate::demo_runner::{resolve_steps, DemoState, WORKSPACE_OFFSET_Y};
use crate::routing::{set_route, Route};
use crate::state::{default_notes_for_level, AppState};

pub struct Callbacks {
    pub on_help_open: Callback<()>,
    pub on_help_close: Callback<()>,
    pub on_tutorial_open: Callback<()>,
    pub on_tutorial_close: Callback<()>,
    pub on_level_change: Callback<UserLevel>,
    pub on_box_drag_start: Callback<DragStartEvent>,
    pub on_box_drag_end: Callback<DragEndEvent>,
    pub on_box_drop: Callback<DropEvent>,
    pub on_copy_source_click: Callback<CopySourceClickEvent>,
    pub on_move: Callback<(WidgetId, Position)>,
    pub on_drop: Callback<DropEvent>,
    // Workspace callbacks (WorkspaceButton is hidden but menu still needs these)
    pub on_workspace_close: Callback<()>,
    pub on_workspace_save: Callback<SaveFormData>,
    pub on_workspace_load: Callback<String>,
    pub on_workspace_delete: Callback<String>,
    pub on_workspace_export: Callback<String>,
    pub on_workspace_import: Callback<web_sys::File>,
    // TextPane callbacks
    pub on_text_pane_change: Callback<String>,
    pub on_text_pane_resize: Callback<(f64, f64)>,
    pub on_text_pane_move: Callback<Position>,
    // Tutorial action callbacks
    pub on_show_me: Option<Callback<()>>,
    pub on_reset: Option<Callback<()>>,
    // Inspection modal callback
    pub on_close_inspection: Callback<()>,
}

/// Configuration for creating callbacks.
pub struct CallbackConfig {
    pub state: UseStateHandle<AppState>,
    pub help_open: UseStateHandle<bool>,
    pub tutorial_open: UseStateHandle<bool>,
    pub user_level: UseStateHandle<UserLevel>,
    pub workspace_open: UseStateHandle<bool>,
    pub dragged_box_id: Rc<RefCell<Option<WidgetId>>>,
    pub pending_new_box: Rc<RefCell<Option<usize>>>,
    pub dirty: UseStateHandle<bool>,
    pub pending_action: UseStateHandle<Option<PendingAction>>,
    pub demo_state: UseStateHandle<DemoState>,
}

#[allow(clippy::too_many_arguments)]
pub fn create_callbacks(cfg: CallbackConfig) -> Callbacks {
    let CallbackConfig {
        state,
        help_open,
        tutorial_open,
        user_level,
        workspace_open,
        dragged_box_id,
        pending_new_box,
        dirty,
        pending_action,
        demo_state,
    } = cfg;
    Callbacks {
        on_help_open: {
            let h = help_open.clone();
            Callback::from(move |_| h.set(true))
        },
        on_help_close: {
            let h = help_open;
            Callback::from(move |_| h.set(false))
        },
        on_tutorial_open: {
            let t = tutorial_open.clone();
            Callback::from(move |_| t.set(true))
        },
        on_tutorial_close: {
            let t = tutorial_open.clone();
            Callback::from(move |_| t.set(false))
        },
        on_level_change: {
            let dirty = dirty.clone();
            let pending_action = pending_action.clone();
            let user_level = user_level.clone();
            let s = state.clone();
            let t = tutorial_open; // Close tutorial menu on level change
            Callback::from(move |level: UserLevel| {
                t.set(false); // Close tutorial menu
                if *dirty {
                    // Ask for confirmation before changing level
                    pending_action.set(Some(PendingAction::LevelChange(level)));
                } else {
                    // Not dirty - change immediately and load sandbox
                    user_level.set(level);
                    // Create fresh sandbox state with appropriate notes
                    let mut new_state = AppState::new();
                    new_state.text_pane_content = default_notes_for_level(level).to_string();
                    s.set(new_state);
                    // Update URL to sandbox
                    set_route(&Route::Sandbox);
                }
            })
        },
        on_box_drag_start: box_handlers::create_box_drag_start(
            dragged_box_id.clone(),
            pending_new_box.clone(),
        ),
        on_box_drag_end: box_handlers::create_box_drag_end(dragged_box_id, pending_new_box.clone()),
        on_box_drop: box_handlers::create_box_drop(state.clone(), pending_new_box, dirty.clone()),
        on_copy_source_click: widget_handlers::create_copy_source(state.clone(), dirty.clone()),
        on_move: widget_handlers::create_move(state.clone(), dirty.clone()),
        on_drop: widget_handlers::create_widget_drop(state.clone(), dirty.clone()),
        // Workspace callbacks (WorkspaceButton is hidden but menu still needs close)
        on_workspace_close: {
            let w = workspace_open.clone();
            Callback::from(move |_| w.set(false))
        },
        on_workspace_save: Callback::from(|data: SaveFormData| {
            log::info!("Save workspace: {} - {}", data.name, data.description);
            // TODO: Implement actual save with storage backend
        }),
        on_workspace_load: {
            let s = state.clone();
            let w = workspace_open;
            let dirty = dirty.clone();
            Callback::from(move |id: String| {
                log::info!("Load workspace: {}", id);
                if let Some(workspace) = crate::workspace::load_bundled_puzzle(&id) {
                    let new_state = crate::workspace::from_workspace(&workspace);
                    s.set(new_state);
                    w.set(false); // Close workspace menu after loading
                    dirty.set(false); // Fresh load is not dirty

                    // Update URL to reflect loaded puzzle
                    // Strip "puzzle-" prefix for cleaner URLs
                    let url_id = id.strip_prefix("puzzle-").unwrap_or(&id);
                    set_route(&Route::Puzzle(url_id.to_string()));
                } else {
                    log::warn!("Puzzle not found: {}", id);
                }
            })
        },
        on_workspace_delete: Callback::from(|id: String| {
            log::info!("Delete workspace: {}", id);
            // TODO: Implement actual delete with storage backend
        }),
        on_workspace_export: Callback::from(|id: String| {
            log::info!("Export workspace: {}", id);
            // TODO: Implement actual export to file
        }),
        on_workspace_import: Callback::from(|_file: web_sys::File| {
            log::info!("Import workspace from file");
            // TODO: Implement actual import from file
        }),
        on_text_pane_change: {
            let s = state.clone();
            let dirty = dirty.clone();
            Callback::from(move |content: String| {
                let mut new_state = (*s).clone();
                new_state.text_pane_content = content;
                s.set(new_state);
                dirty.set(true);
            })
        },
        on_text_pane_resize: {
            let s = state.clone();
            Callback::from(move |size: (f64, f64)| {
                let mut new_state = (*s).clone();
                new_state.text_pane_size = size;
                s.set(new_state);
                // Resizing doesn't make it dirty - it's layout, not content
            })
        },
        on_text_pane_move: {
            let s = state.clone();
            Callback::from(move |pos: Position| {
                let mut new_state = (*s).clone();
                new_state.text_pane_position = pos;
                s.set(new_state);
                // Moving doesn't make it dirty - it's layout, not content
            })
        },
        // Show Me callback - only present if demo_steps exist
        on_show_me: if state.demo_steps.is_empty() {
            None
        } else {
            let demo_state_clone = demo_state.clone();
            let app_state_clone = state.clone();
            let steps = state.demo_steps.clone();
            Some(Callback::from(move |_| {
                log::info!("Show Me button clicked - starting demo animation");

                // Don't start if already playing
                if demo_state_clone.is_playing {
                    log::info!("Demo already playing, ignoring click");
                    return;
                }

                // Resolve any semantic targets in the steps to concrete coordinates
                let resolved_steps = resolve_steps(&steps, &app_state_clone);
                log::info!(
                    "Demo: {} steps to play ({} after resolving)",
                    steps.len(),
                    resolved_steps.len()
                );

                // Set the demo state with resolved steps - the use_effect hook will drive the animation
                demo_state_clone.set(DemoState {
                    is_playing: true,
                    cursor_x: 100.0, // Start at a visible position
                    cursor_y: 100.0 + WORKSPACE_OFFSET_Y,
                    is_dragging: false,
                    transition_ms: 0,
                    step_index: 0,
                    steps: resolved_steps,
                    dragged_widget_id: None,
                    dragged_is_box: false,
                });
            }))
        },
        // Inspection modal close callback
        on_close_inspection: {
            let s = state.clone();
            Callback::from(move |_| {
                let mut new_state = (*s).clone();
                new_state.inspection_modal = None;
                s.set(new_state);
            })
        },
        // Reset callback - always available (sandbox resets to default, puzzles reload)
        on_reset: Some({
            let dirty = dirty.clone();
            let pending_action = pending_action.clone();
            let s = state;
            Callback::from(move |_| {
                log::info!("Reset button clicked");
                if *dirty {
                    // Ask for confirmation
                    pending_action.set(Some(PendingAction::Reset));
                } else {
                    // Not dirty - reset immediately
                    // Get the current route at reset time (not at callback creation time)
                    let current_id = match crate::routing::current_route() {
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
                    log::info!("Reset (not dirty): current_id = {:?}", current_id);

                    if let Some(ref id) = current_id {
                        if let Some(workspace) = crate::workspace::load_bundled_puzzle(id) {
                            let new_state = crate::workspace::from_workspace(&workspace);
                            s.set(new_state);
                        }
                    } else {
                        // Sandbox - reset to default
                        s.set(AppState::new());
                    }
                }
            })
        }),
    }
}
