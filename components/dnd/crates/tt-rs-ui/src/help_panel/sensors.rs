//! Help content for Sensors (tt3 level).

use yew::prelude::*;

/// Content explaining sensors and their uses.
pub fn sensors_content() -> Html {
    html! {
        <div class="help-section">
            <h3>{ "Welcome to tt-rs Sensors Mode!" }</h3>
            <p>{ "This level introduces Sensors for generating time and random values." }</p>

            <h4>{ "TIME SENSOR (Clock)" }</h4>
            <p>{ "The clock sensor produces the current time as a number." }</p>
            <ul>
                <li>{ "Click the clock to generate a number with the current time (in milliseconds)" }</li>
                <li>{ "Useful for timestamps, measuring elapsed time, or creating unique IDs" }</li>
                <li>{ "Each click produces a new number with the time at that moment" }</li>
            </ul>

            <h4>{ "RANDOM SENSOR (Die)" }</h4>
            <p>{ "The die sensor produces random numbers." }</p>
            <ul>
                <li>{ "Click the die to generate a random number (0 to 999,999)" }</li>
                <li>{ "Useful for games, simulations, or any situation requiring randomness" }</li>
                <li>{ "Each click produces a different random value" }</li>
            </ul>

            <h4>{ "MODULO: GETTING SMALL NUMBERS" }</h4>
            <p>{ "Sensor values are often large. Use the modulo tool (%10) to get smaller numbers:" }</p>
            <ul>
                <li>{ "345678 % 10 = 8 (just the last digit!)" }</li>
                <li>{ "345678 % 100 = 78 (last two digits)" }</li>
                <li>{ "Perfect for reducing random numbers to a usable range" }</li>
            </ul>

            <h4>{ "USING SENSORS WITH ROBOTS" }</h4>
            <p>{ "Sensors become powerful when combined with trained robots:" }</p>
            <ul>
                <li>{ "Train a robot to apply modulo and process the resulting number" }</li>
                <li>{ "Use birds to send results to a nest for collection" }</li>
                <li>{ "Create programs that react to time or make random choices" }</li>
            </ul>

            <h4>{ "TRY THE TUTORIAL" }</h4>
            <p>
                <a href="#/tutorial/robot-sensor">{ "Robot with Sensors Tutorial" }</a>
                { " - Learn to combine robots, sensors, modulo, and bird/nest messaging!" }
            </p>

            <p class="help-tip">
                <strong>{ "Tip: " }</strong>
                { "Drag sensors from the palette to create copies you can position anywhere." }
            </p>
        </div>
    }
}
