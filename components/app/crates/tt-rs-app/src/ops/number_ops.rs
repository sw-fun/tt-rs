//! Number arithmetic operations.

use tt_rs_core::WidgetId;
use tt_rs_hit_test::find_widget_at_excluding;
use tt_rs_robot::Action;

use crate::state::AppState;
use crate::widget_item::WidgetItem;

/// Handle dropping number on another number.
pub fn handle_number_on_number(state: &mut AppState, id: WidgetId, mx: f64, my: f64) -> bool {
    // Use find_widget_at_excluding to skip the dragged widget
    let target_id = match find_widget_at_excluding(mx, my, id) {
        Some((tid, false)) => tid, // false = not a box
        _ => return false,
    };

    // Verify target is a number
    if !matches!(state.widgets.get(&target_id), Some(WidgetItem::Number(_))) {
        return false;
    }

    let dropped = match state.widgets.get(&id) {
        Some(WidgetItem::Number(n)) => n.clone(),
        _ => return false,
    };

    let mut target = match state.widgets.get(&target_id) {
        Some(WidgetItem::Number(n)) if !n.is_copy_source() => n.clone(),
        _ => return false,
    };

    if target.apply(&dropped).is_none() {
        return false;
    }

    record_action(state, &dropped, target_id);
    state.widgets.insert(target_id, WidgetItem::Number(target));
    state.widgets.remove(&id);
    state.positions.remove(&id);
    true
}

fn record_action(state: &mut AppState, dropped: &tt_rs_number::Number, _target_id: WidgetId) {
    let op = dropped.operator().symbol().chars().next().unwrap_or('+');
    // Use type-based path so robot can find any number in workspace
    state.record_action(Action::ApplyArithmetic {
        operator: op,
        numerator: dropped.numerator(),
        denominator: dropped.denominator() as i64,
        target_path: "workspace:number".to_string(),
    });
}
