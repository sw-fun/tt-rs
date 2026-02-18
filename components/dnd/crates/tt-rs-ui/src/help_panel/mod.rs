//! HelpPanel component with tutorial accordion sections.
//!
//! Provides a comprehensive help panel for new users.
//! Content is completely different based on the current user level.

mod advanced;
mod basics;
mod messaging;
mod sensors;
mod tutorials;

use crate::accordion::{Accordion, AccordionSection};
use crate::slide_panel::SlidePanel;
use crate::user_level::UserLevel;
use yew::prelude::*;

/// Properties for the HelpPanel component.
#[derive(Properties, Clone, PartialEq)]
pub struct HelpPanelProps {
    /// Whether the help panel is open.
    pub is_open: bool,
    /// Callback when close is requested.
    pub on_close: Callback<()>,
    /// Current user level for contextual help.
    #[prop_or_default]
    pub level: UserLevel,
}

/// Help panel with tutorial sections.
#[function_component(HelpPanel)]
pub fn help_panel(props: &HelpPanelProps) -> Html {
    let title = format!("Help & Tutorials ({})", props.level.name());

    html! {
        <SlidePanel
            is_open={props.is_open}
            on_close={props.on_close.clone()}
            title={title}
        >
            <Accordion>
                { render_level_content(props.level) }
                <AccordionSection title="About ToonTalk">
                    { advanced::about_content() }
                </AccordionSection>
            </Accordion>
        </SlidePanel>
    }
}

/// Render content specific to the user level.
fn render_level_content(level: UserLevel) -> Html {
    match level {
        UserLevel::Tt1 => render_tt1_content(),
        UserLevel::Tt2 => render_tt2_content(),
        UserLevel::Tt3 => render_tt3_content(),
    }
}

/// tt1 content: Basic features (numbers, boxes, tools, robots).
fn render_tt1_content() -> Html {
    html! {
        <>
            <AccordionSection title="Tutorials" default_open={true}>
                { tutorials::tutorials_content(UserLevel::Tt1) }
            </AccordionSection>
            <AccordionSection title="Getting Started">
                { basics::getting_started_content(UserLevel::Tt1) }
            </AccordionSection>
            <AccordionSection title="Numbers & Arithmetic">
                { basics::numbers_content() }
            </AccordionSection>
            <AccordionSection title="Boxes & Organization">
                { basics::boxes_content() }
            </AccordionSection>
            <AccordionSection title="Tools">
                { advanced::tools_content() }
            </AccordionSection>
            <AccordionSection title="Training Robots">
                { advanced::robots_content() }
            </AccordionSection>
            <AccordionSection title="Tips & Tricks">
                { advanced::tips_content(UserLevel::Tt1) }
            </AccordionSection>
        </>
    }
}

/// tt2 content: Messaging features + tutorials.
fn render_tt2_content() -> Html {
    html! {
        <>
            <AccordionSection title="Tutorials" default_open={true}>
                { tutorials::tutorials_content(UserLevel::Tt2) }
            </AccordionSection>
            <AccordionSection title="Birds & Nests (Messaging)">
                { messaging::messaging_content() }
            </AccordionSection>
        </>
    }
}

/// tt3 content: Sensors features + tutorials.
fn render_tt3_content() -> Html {
    html! {
        <>
            <AccordionSection title="Tutorials" default_open={true}>
                { tutorials::tutorials_content(UserLevel::Tt3) }
            </AccordionSection>
            <AccordionSection title="Sensors (Time & Random)">
                { sensors::sensors_content() }
            </AccordionSection>
        </>
    }
}
