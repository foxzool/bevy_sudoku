use crate::game::control_tab::ToggleTab;
use crate::game::MoveSelectCell;
use crate::{
    game::NewDigit,
    game::{CleanCell, NewCandidate, SelectedCell},
};
use bevy::prelude::*;

pub(crate) fn keyboard_input(
    mut commands: Commands,
    mut keyboard_input: Res<ButtonInput<KeyCode>>,
    mut selected_cell: Single<Entity, With<SelectedCell>>,
) {
    if keyboard_input.just_pressed(KeyCode::Delete) {
        commands.trigger_targets(CleanCell, vec![*selected_cell]);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        commands.send_event(ToggleTab::Any);
        return;
    }
    if keyboard_input.any_just_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
        commands.send_event(ToggleTab::Any);
        return;
    }

    if keyboard_input.any_just_released([KeyCode::AltLeft, KeyCode::AltRight]) {
        commands.send_event(ToggleTab::Any);
        return;
    }

    let press_0 = keyboard_input.any_just_pressed([KeyCode::Digit0, KeyCode::Numpad0]);
    let press_1 = keyboard_input.any_just_pressed([KeyCode::Digit1, KeyCode::Numpad1]);
    let press_2 = keyboard_input.any_just_pressed([KeyCode::Digit2, KeyCode::Numpad2]);
    let press_3 = keyboard_input.any_just_pressed([KeyCode::Digit3, KeyCode::Numpad3]);
    let press_4 = keyboard_input.any_just_pressed([KeyCode::Digit4, KeyCode::Numpad4]);
    let press_5 = keyboard_input.any_just_pressed([KeyCode::Digit5, KeyCode::Numpad5]);
    let press_6 = keyboard_input.any_just_pressed([KeyCode::Digit6, KeyCode::Numpad6]);
    let press_7 = keyboard_input.any_just_pressed([KeyCode::Digit7, KeyCode::Numpad7]);
    let press_8 = keyboard_input.any_just_pressed([KeyCode::Digit8, KeyCode::Numpad8]);
    let press_9 = keyboard_input.any_just_pressed([KeyCode::Digit9, KeyCode::Numpad9]);

    let alt = keyboard_input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);

    let num = if press_1 {
        Some(1)
    } else if press_2 {
        Some(2)
    } else if press_3 {
        Some(3)
    } else if press_4 {
        Some(4)
    } else if press_5 {
        Some(5)
    } else if press_6 {
        Some(6)
    } else if press_7 {
        Some(7)
    } else if press_8 {
        Some(8)
    } else if press_9 {
        Some(9)
    } else if press_0 {
        Some(0)
    } else {
        None
    };

    if let Some(num) = num {
        if alt {
            commands.trigger_targets(NewCandidate::new(num), vec![*selected_cell]);
        } else {
            commands.trigger_targets(NewDigit::new(num), vec![*selected_cell]);
        }
    }
}

#[derive(Resource)]
pub struct MoveTimer {
    timer: Timer,
}

impl Default for MoveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        }
    }
}

pub(crate) fn keyboard_move_cell(
    mut commands: Commands,
    mut keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: Local<MoveTimer>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        commands.trigger(MoveSelectCell::Up);
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        commands.trigger(MoveSelectCell::Down);
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        commands.trigger(MoveSelectCell::Left);
        return;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        commands.trigger(MoveSelectCell::Right);
        return;
    }

    if keyboard_input.any_pressed([
        KeyCode::ArrowUp,
        KeyCode::ArrowDown,
        KeyCode::ArrowLeft,
        KeyCode::ArrowRight,
    ]) {
        if timer.timer.tick(time.delta()).just_finished() {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                commands.trigger(MoveSelectCell::Up);
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                commands.trigger(MoveSelectCell::Down);
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                commands.trigger(MoveSelectCell::Left);
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                commands.trigger(MoveSelectCell::Right);
            }
        }
    } else {
        timer.timer.reset();
    }
}
