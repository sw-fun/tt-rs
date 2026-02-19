//! Tutorial links organized by user level.

use crate::user_level::UserLevel;
use yew::prelude::*;

/// Tutorial links for the current user level.
pub fn tutorials_content(level: UserLevel) -> Html {
    html! {
        <div class="help-section">
            <h3>{ "Interactive Tutorials" }</h3>
            <p>{ "Click a tutorial to try it. Use 'Show Me' to watch a demonstration." }</p>

            { tt1_tutorials() }

            if level >= UserLevel::Tt2 {
                { tt2_tutorials() }
            }

            if level >= UserLevel::Tt3 {
                { tt3_tutorials() }
            }
        </div>
    }
}

fn tt1_tutorials() -> Html {
    html! {
        <>
            <h4>{ "tt1 - Basic Tutorials" }</h4>
            <ul class="tutorial-list">
                <li>
                    <a href="#/tutorial/fill-box">{ "Fill the Box" }</a>
                    { " - Learn to drag numbers into boxes" }
                </li>
                <li>
                    <a href="#/tutorial/add-numbers">{ "Add Numbers" }</a>
                    { " - Combine numbers using arithmetic" }
                </li>
                <li>
                    <a href="#/tutorial/copy-widget">{ "Copy Widgets" }</a>
                    { " - Use the wand to make copies" }
                </li>
                <li>
                    <a href="#/tutorial/train-robot">{ "Train a Robot" }</a>
                    { " - Teach a robot by showing it what to do" }
                </li>
            </ul>
        </>
    }
}

fn tt2_tutorials() -> Html {
    html! {
        <>
            <h4>{ "tt2 - Messaging Tutorials" }</h4>
            <ul class="tutorial-list">
                <li>
                    <a href="#/tutorial/bird-nest">{ "Bird & Nest Messaging" }</a>
                    { " - Send messages between birds and nests" }
                </li>
            </ul>
        </>
    }
}

fn tt3_tutorials() -> Html {
    html! {
        <>
            <h4>{ "tt3 - Sensor Tutorials" }</h4>
            <ul class="tutorial-list">
                <li>
                    <a href="#/tutorial/robot-sensor">{ "Robot with Sensors" }</a>
                    { " - Combine robots, sensors, modulo, and bird messaging" }
                </li>
            </ul>
        </>
    }
}
