//! Basic tutorial content - getting started, numbers, boxes.

use crate::user_level::UserLevel;
use yew::prelude::*;

pub fn getting_started_content(level: UserLevel) -> Html {
    html! {
        <div class="help-section">
            <p>{"Welcome to tt-rs, a visual programming environment!"}</p>
            <p>{"Everything is done by dragging and dropping:"}</p>
            <ul>
                <li>{"Click and drag any item to move it"}</li>
                <li>{"Drop items onto each other to combine them"}</li>
                <li>{"Use tools to modify or copy items"}</li>
            </ul>
            { level_specific_intro(level) }
            <p class="help-tip">
                {"Try dragging the \"+1\" onto the \"0\" to add them together!"}
            </p>
        </div>
    }
}

fn level_specific_intro(level: UserLevel) -> Html {
    match level {
        UserLevel::Tt1 => html! {
            <p>
                {"You're in "}<strong>{"tt1"}</strong>{" mode - learn the basics of \
                 numbers, boxes, and tools."}
            </p>
        },
        UserLevel::Tt2 => html! {
            <p>
                {"You're in "}<strong>{"tt2"}</strong>{" mode - explore messaging \
                 with birds and nests!"}
            </p>
        },
        UserLevel::Tt3 => html! {
            <p>
                {"You're in "}<strong>{"tt3"}</strong>{" mode - use sensors for \
                 time and random values!"}
            </p>
        },
    }
}

pub fn numbers_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"Number Values"}</h4>
            <p>{"Yellow boxes with numbers are values you can manipulate."}</p>

            <h4>{"Arithmetic Tools"}</h4>
            <p>{"Tools show an operator (+, -, \u{00D7}, \u{00F7}) and a value:"}</p>
            <ul>
                <li><strong>{"+1"}</strong>{" - Adds 1 to a number"}</li>
                <li><strong>{"+5"}</strong>{" - Adds 5 to a number"}</li>
                <li><strong>{"-1"}</strong>{" - Subtracts 1 from a number"}</li>
                <li><strong>{"\u{00D7}2"}</strong>{" - Multiplies by 2"}</li>
                <li><strong>{"\u{00F7}2"}</strong>{" - Divides by 2"}</li>
            </ul>

            <h4>{"Copy Sources"}</h4>
            <p>
                {"Stacked items are \"copy sources\" - click them to create copies \
                 that you can drag around."}
            </p>
        </div>
    }
}

pub fn boxes_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Boxes?"}</h4>
            <p>{"Boxes have holes that can hold items. Use them to organize values."}</p>

            <h4>{"Placing Items"}</h4>
            <p>{"Drag any item onto a box hole to place it inside."}</p>

            <h4>{"Splitting Boxes"}</h4>
            <p>
                {"Drag a box onto a number to split it! The number determines \
                 where to split (e.g., drop on \"2\" splits after the 2nd hole)."}
            </p>

            <h4>{"Joining Boxes"}</h4>
            <p>{"Drag one box onto another to join them into a larger box."}</p>

            <h4>{"Creating New Boxes"}</h4>
            <p>
                {"While dragging a box, press a number key (0-9) then drop - \
                 a new box with that many holes will appear!"}
            </p>
        </div>
    }
}
