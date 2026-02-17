//! WorkspaceMenu component for browsing tutorials, examples, and challenges.
//!
//! Provides a slide panel with tabbed navigation for different workspace categories.

use crate::slide_panel::SlidePanel;
use crate::user_level::UserLevel;
use yew::prelude::*;

/// Tab selection for the workspace menu.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum WorkspaceTab {
    #[default]
    Tutorials,
    Examples,
    Challenges,
}

impl WorkspaceTab {
    fn label(&self) -> &'static str {
        match self {
            WorkspaceTab::Tutorials => "Tutorials",
            WorkspaceTab::Examples => "Examples",
            WorkspaceTab::Challenges => "Challenges",
        }
    }
}

/// Workspace metadata for display in the list.
#[derive(Clone, PartialEq, Debug)]
pub struct WorkspaceMetadata {
    /// Unique identifier.
    pub id: String,
    /// User-provided name.
    pub name: String,
    /// User-provided description.
    pub description: String,
    /// User level when saved.
    pub user_level: String,
    /// Last modified timestamp (ISO 8601).
    pub modified_at: String,
    /// Whether this is a bundled example (read-only).
    pub is_bundled: bool,
}

/// Save form data (kept for API compatibility).
#[derive(Clone, PartialEq, Debug, Default)]
pub struct SaveFormData {
    pub name: String,
    pub description: String,
}

/// Properties for the WorkspaceMenu component.
#[derive(Properties, Clone, PartialEq)]
pub struct WorkspaceMenuProps {
    /// Whether the menu is open.
    pub is_open: bool,
    /// Callback when close is requested.
    pub on_close: Callback<()>,
    /// Callback when save is requested with form data.
    pub on_save: Callback<SaveFormData>,
    /// Callback when load is requested with workspace ID.
    pub on_load: Callback<String>,
    /// Callback when delete is requested with workspace ID.
    pub on_delete: Callback<String>,
    /// Callback when export is requested with workspace ID.
    pub on_export: Callback<String>,
    /// Callback when import file is selected.
    pub on_import: Callback<web_sys::File>,
    /// Current user level.
    pub current_level: UserLevel,
    /// List of available workspaces.
    pub workspaces: Vec<WorkspaceMetadata>,
}

/// Workspace menu slide panel with tabs.
#[function_component(WorkspaceMenu)]
pub fn workspace_menu(props: &WorkspaceMenuProps) -> Html {
    let active_tab = use_state(WorkspaceTab::default);

    // Reset tab when menu closes
    {
        let active_tab = active_tab.clone();
        let is_open = props.is_open;
        use_effect_with(is_open, move |&open| {
            if !open {
                active_tab.set(WorkspaceTab::Tutorials);
            }
            || ()
        });
    }

    let make_tab_click = |tab: WorkspaceTab| {
        let active_tab = active_tab.clone();
        Callback::from(move |_| active_tab.set(tab))
    };

    let tabs = [
        WorkspaceTab::Tutorials,
        WorkspaceTab::Examples,
        WorkspaceTab::Challenges,
    ];

    let tab_content = match *active_tab {
        WorkspaceTab::Tutorials => render_tutorials_tab(props.on_load.clone()),
        WorkspaceTab::Examples => render_examples_tab(),
        WorkspaceTab::Challenges => render_challenges_tab(),
    };

    html! {
        <SlidePanel
            is_open={props.is_open}
            on_close={props.on_close.clone()}
            title="Workspaces"
        >
            <div class="workspace-tabs">
                { for tabs.iter().map(|&tab| {
                    let is_active = *active_tab == tab;
                    let class = if is_active { "workspace-tab active" } else { "workspace-tab" };
                    html! {
                        <button
                            class={class}
                            onclick={make_tab_click(tab)}
                        >
                            { tab.label() }
                        </button>
                    }
                }) }
            </div>
            <div class="workspace-tab-content">
                { tab_content }
            </div>
        </SlidePanel>
    }
}

/// Bundled tutorial puzzle metadata.
struct TutorialPuzzle {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    difficulty: &'static str,
}

/// Two-part tutorials with Show Me examples.
const TUTORIALS: &[TutorialPuzzle] = &[
    TutorialPuzzle {
        id: "tutorial-train-robot",
        name: "Train Your First Robot",
        description: "Teach a robot by demonstration - watch it generalize!",
        difficulty: "Beginner",
    },
    TutorialPuzzle {
        id: "tutorial-fill-box",
        name: "Fill a Box",
        description: "Learn to put numbers into boxes (with Show Me)",
        difficulty: "Beginner",
    },
    TutorialPuzzle {
        id: "tutorial-add-numbers",
        name: "Add Numbers",
        description: "Learn to add numbers together (with Show Me)",
        difficulty: "Beginner",
    },
    TutorialPuzzle {
        id: "tutorial-copy-widget",
        name: "Copy with Wand",
        description: "Learn to copy widgets with the magic wand (with Show Me)",
        difficulty: "Beginner",
    },
];

/// Classic ToonTalk puzzles from ToonTalk Reborn 2017.
const CLASSIC_PUZZLES: &[TutorialPuzzle] = &[
    TutorialPuzzle {
        id: "puzzle-fill-box",
        name: "Fill a Box (Classic)",
        description: "Put numbers 1 and 2 into a box",
        difficulty: "Easy",
    },
    TutorialPuzzle {
        id: "puzzle-make-four",
        name: "Make a 4",
        description: "Create 4 by combining two 2s",
        difficulty: "Easy",
    },
    TutorialPuzzle {
        id: "puzzle-make-nine",
        name: "Make a 9",
        description: "Create 9 using only 3s",
        difficulty: "Easy-Medium",
    },
];

/// Render the Tutorials tab content.
fn render_tutorials_tab(on_load: Callback<String>) -> Html {
    html! {
        <div class="workspace-category">
            <div class="workspace-category-header">
                <span class="category-icon">{ "📚" }</span>
                <h3>{ "Tutorials" }</h3>
            </div>
            <p class="workspace-category-desc">
                { "Step-by-step guided lessons to learn tt-rs concepts." }
            </p>

            <div class="workspace-section">
                <h4>{ "Getting Started" }</h4>
                <p class="section-desc">{ "Two-part tutorials with Show Me examples" }</p>
                <ul class="workspace-list">
                    { for TUTORIALS.iter().map(|puzzle| {
                        let puzzle_id = puzzle.id.to_string();
                        let on_click = {
                            let on_load = on_load.clone();
                            let id = puzzle_id.clone();
                            Callback::from(move |_| on_load.emit(id.clone()))
                        };
                        html! {
                            <li class="workspace-item" onclick={on_click}>
                                <div class="workspace-item-header">
                                    <span class="workspace-item-name">{ puzzle.name }</span>
                                    <span class="workspace-item-difficulty">{ puzzle.difficulty }</span>
                                </div>
                                <p class="workspace-item-desc">{ puzzle.description }</p>
                            </li>
                        }
                    }) }
                </ul>
            </div>

            <div class="workspace-section">
                <h4>{ "Classic ToonTalk Puzzles" }</h4>
                <p class="section-desc">{ "Introductory puzzles from ToonTalk Reborn (2017)" }</p>
                <ul class="workspace-list">
                    { for CLASSIC_PUZZLES.iter().map(|puzzle| {
                        let puzzle_id = puzzle.id.to_string();
                        let on_click = {
                            let on_load = on_load.clone();
                            let id = puzzle_id.clone();
                            Callback::from(move |_| on_load.emit(id.clone()))
                        };
                        html! {
                            <li class="workspace-item" onclick={on_click}>
                                <div class="workspace-item-header">
                                    <span class="workspace-item-name">{ puzzle.name }</span>
                                    <span class="workspace-item-difficulty">{ puzzle.difficulty }</span>
                                </div>
                                <p class="workspace-item-desc">{ puzzle.description }</p>
                            </li>
                        }
                    }) }
                </ul>
            </div>

            <div class="workspace-coming-soon">
                <span class="coming-soon-icon">{ "🚧" }</span>
                <p>{ "More tutorials coming soon..." }</p>
                <p class="coming-soon-hint">
                    { "Including robots, scales, birds & nests, and more!" }
                </p>
            </div>
        </div>
    }
}

/// Render the Examples tab content.
fn render_examples_tab() -> Html {
    html! {
        <div class="workspace-category">
            <div class="workspace-category-header">
                <span class="category-icon">{ "💡" }</span>
                <h3>{ "Examples" }</h3>
            </div>
            <p class="workspace-category-desc">
                { "Pre-built workspaces demonstrating tt-rs features and patterns." }
            </p>
            <div class="workspace-coming-soon">
                <span class="coming-soon-icon">{ "🚧" }</span>
                <p>{ "Coming soon..." }</p>
                <p class="coming-soon-hint">
                    { "Examples will show arithmetic operations, message passing, robot training, and more!" }
                </p>
            </div>
        </div>
    }
}

/// Render the Challenges tab content.
fn render_challenges_tab() -> Html {
    html! {
        <div class="workspace-category">
            <div class="workspace-category-header">
                <span class="category-icon">{ "🏆" }</span>
                <h3>{ "Challenges" }</h3>
            </div>
            <p class="workspace-category-desc">
                { "Test your skills with programming puzzles and problems to solve." }
            </p>
            <div class="workspace-coming-soon">
                <span class="coming-soon-icon">{ "🚧" }</span>
                <p>{ "Coming soon..." }</p>
                <p class="coming-soon-hint">
                    { "Challenges will include sorting, searching, math puzzles, and logic problems!" }
                </p>
            </div>
        </div>
    }
}
