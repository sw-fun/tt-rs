//! Advanced tutorial content - tools, robots, tips, about.

use yew::prelude::*;

pub fn tools_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"Scales"}</h4>
            <p>
                {"Drop numbers on the left or right pan to compare them. \
                 The scales will tip toward the heavier side."}
            </p>

            <h4>{"Vacuum"}</h4>
            <p>
                {"The vacuum erases things! Drag it onto:"
            }</p>
            <ul>
                <li>{"A box hole to erase its contents"}</li>
                <li>{"A free number to delete it"}</li>
            </ul>

            <h4>{"Magic Wand"}</h4>
            <p>
                {"The wand copies things! Drag it onto any item to create \
                 a duplicate."}
            </p>
        </div>
    }
}

pub fn robots_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"What are Robots?"}</h4>
            <p>
                {"Robots can learn by watching you! They remember your actions \
                 and can repeat them automatically."}
            </p>

            <h4>{"Training a Robot"}</h4>
            <ol>
                <li>{"Click the robot to start training (it turns yellow)"}</li>
                <li>{"Perform actions - the robot watches and records them"}</li>
                <li>{"Click the robot again to stop training"}</li>
            </ol>

            <h4>{"Running a Trained Robot"}</h4>
            <p>
                {"Once trained, click the robot to run its recorded actions. \
                 It will turn green while working."}
            </p>

            <h4>{"Copying Robots"}</h4>
            <p>
                {"Use the magic wand on a trained robot to create a copy \
                 with the same training!"}
            </p>
        </div>
    }
}

pub fn tips_content(level: crate::user_level::UserLevel) -> Html {
    html! {
        <div class="help-section">
            <ul>
                <li>
                    <strong>{"Undo mistakes: "}</strong>
                    {"Use the wand to copy before experimenting"}
                </li>
                <li>
                    <strong>{"Precise placement: "}</strong>
                    {"Drop items carefully - they land where you release"}
                </li>
                <li>
                    <strong>{"Tools are persistent: "}</strong>
                    {"The vacuum and wand stay where you drop them"}
                </li>
                { level_specific_tips(level) }
                <li>
                    <strong>{"Try things! "}</strong>
                    {"The best way to learn is to experiment"}
                </li>
            </ul>
        </div>
    }
}

fn level_specific_tips(level: crate::user_level::UserLevel) -> yew::Html {
    use crate::user_level::UserLevel;
    match level {
        UserLevel::Tt1 => yew::html! {},
        UserLevel::Tt2 => yew::html! {
            <li>
                <strong>{"Bird pairing: "}</strong>
                {"Pair birds with nests before trying to send messages"}
            </li>
        },
        UserLevel::Tt3 => yew::html! {
            <li>
                <strong>{"Sensors: "}</strong>
                {"Click a sensor to produce a number - time or random"}
            </li>
        },
    }
}

pub fn about_content() -> Html {
    html! {
        <div class="help-section">
            <h4>{"History"}</h4>
            <p>
                {"ToonTalk is a visual programming environment originally created by \
                 Ken Kahn. It teaches programming through animated metaphors where \
                 robots learn by watching."}
            </p>

            <h4>{"Previous Versions"}</h4>
            <ul>
                <li>
                    <strong>{"ToonTalk (C++, 1992-2009)"}</strong>
                    {" - The original Windows application"}
                </li>
                <li>
                    <strong>{"ToonTalk Reborn (JavaScript, 2014-2017)"}</strong>
                    {" - Browser-based reimplementation using jQuery"}
                </li>
                <li>
                    <strong>{"tt-rs (Rust/WebAssembly, 2025)"}</strong>
                    {" - This modern reimplementation"}
                </li>
            </ul>

            <h4>{"Learn More"}</h4>
            <p>
                <a href="https://www.toontalk.com/" target="_blank" rel="noopener">
                    {"ToonTalk Website"}
                </a>
                {" - Official site by Ken Kahn"}
            </p>
            <p>
                <a href="https://en.wikipedia.org/wiki/ToonTalk" target="_blank" rel="noopener">
                    {"Wikipedia Article"}
                </a>
                {" - Background and history"}
            </p>
            <p>
                <a href="https://github.com/ToonTalk/ToonTalk" target="_blank" rel="noopener">
                    {"ToonTalk Reborn (GitHub)"}
                </a>
                {" - JavaScript version source code"}
            </p>
        </div>
    }
}
