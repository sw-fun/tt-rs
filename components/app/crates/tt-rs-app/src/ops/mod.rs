//! Widget and box operations organized by concern.

mod bird_ops;
mod box_hole;
mod box_ops;
mod dropzone_ops;
mod magnifier_ops;
mod number_ops;
mod robot_ops;
mod scales_ops;
mod sensor_ops;
mod vacuum_ops;
mod wand_ops;

pub use bird_ops::{handle_bird_drop, handle_drop_on_bird, handle_nest_click, handle_nest_drop};
pub use box_hole::handle_box_hole_drop;
pub use box_ops::handle_box_drop;
pub use dropzone_ops::handle_dropzone_drop;
pub use magnifier_ops::handle_magnifier_drop;
pub use number_ops::handle_number_on_number;
pub use robot_ops::handle_robot_click;
pub use scales_ops::handle_scales_drop;
pub use sensor_ops::handle_sensor_click;
pub use vacuum_ops::handle_vacuum_drop;
pub use wand_ops::handle_wand_drop;
