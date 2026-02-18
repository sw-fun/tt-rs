//! Widget trait implementation for Magnifier.

use crate::magnifier::Magnifier;
use tt_rs_core::{MatchResult, Widget, WidgetId};
use yew::prelude::*;

impl Widget for Magnifier {
    fn type_name(&self) -> &'static str {
        "magnifier"
    }

    fn id(&self) -> WidgetId {
        self.id
    }

    fn copy(&self) -> Box<dyn Widget> {
        Box::new(self.copy_magnifier())
    }

    fn matches(&self, other: &dyn Widget) -> MatchResult {
        if other.type_name() == "magnifier" {
            MatchResult::Match
        } else {
            MatchResult::NoMatch
        }
    }

    fn render(&self) -> Html {
        html! {
            <div class="widget magnifier tool"
                 data-widget-id={self.id.to_string()}>
                <img src="images/tt-magnifier.svg" alt="magnifier" class="magnifier-image" />
            </div>
        }
    }

    fn description(&self) -> String {
        "magnifier tool".to_string()
    }
}
