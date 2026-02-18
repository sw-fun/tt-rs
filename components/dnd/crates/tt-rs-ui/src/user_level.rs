//! User level selector component.
//!
//! Dropdown for selecting UI complexity level (tt1, tt2, etc.).

use yew::prelude::*;

/// User interface complexity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum UserLevel {
    /// Basic level: numbers, boxes, arithmetic, scales, tools.
    #[default]
    Tt1,
    /// Intermediate level: adds Bird/Nest messaging.
    Tt2,
    /// Advanced level: adds Sensors for time and random values.
    Tt3,
}

impl UserLevel {
    /// Display name for the level.
    pub fn name(&self) -> &'static str {
        match self {
            UserLevel::Tt1 => "tt1",
            UserLevel::Tt2 => "tt2",
            UserLevel::Tt3 => "tt3",
        }
    }

    /// Description of what's available at this level.
    pub fn description(&self) -> &'static str {
        match self {
            UserLevel::Tt1 => "Basic: Numbers, Boxes, Tools",
            UserLevel::Tt2 => "Messaging: Birds & Nests",
            UserLevel::Tt3 => "Sensors: Time & Random",
        }
    }
}

/// Properties for the UserLevelSelector component.
#[derive(Properties, Clone, PartialEq)]
pub struct UserLevelSelectorProps {
    /// Current selected level.
    pub level: UserLevel,
    /// Callback when level changes.
    pub on_change: Callback<UserLevel>,
}

/// Dropdown selector for user level.
#[function_component(UserLevelSelector)]
pub fn user_level_selector(props: &UserLevelSelectorProps) -> Html {
    let on_change = {
        let callback = props.on_change.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let level = match target.value().as_str() {
                "tt2" => UserLevel::Tt2,
                "tt3" => UserLevel::Tt3,
                _ => UserLevel::Tt1,
            };
            callback.emit(level);
        })
    };

    html! {
        <div class="user-level-selector" title="Select feature level. tt1=Basic, tt2=Messaging, tt3=Sensors.">
            <select onchange={on_change} value={props.level.name()}>
                <option value="tt1" selected={props.level == UserLevel::Tt1}>
                    { "tt1 - Basic" }
                </option>
                <option value="tt2" selected={props.level == UserLevel::Tt2}>
                    { "tt2 - Messaging" }
                </option>
                <option value="tt3" selected={props.level == UserLevel::Tt3}>
                    { "tt3 - Sensors" }
                </option>
            </select>
        </div>
    }
}
