use std::fmt::Display;

use bevy::prelude::KeyCode;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ControlDevice {
    Gamepad {
        gamepad_id: usize,
    },
    Keyboard {
        keyboard_id: usize,
        left: KeyCode,
        right: KeyCode,
        top: KeyCode,
        bottom: KeyCode,
        action1: KeyCode,
        action2: KeyCode,
        start: KeyCode,
        cancel: KeyCode,
    },
}

impl ControlDevice {
    pub fn id(&self) -> usize {
        match self {
            ControlDevice::Gamepad { gamepad_id } => *gamepad_id,
            ControlDevice::Keyboard { keyboard_id, .. } => *keyboard_id,
        }
    }
}

impl Display for ControlDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlDevice::Gamepad { gamepad_id } => {
                f.write_fmt(format_args!("Gamepad {gamepad_id}"))
            }
            ControlDevice::Keyboard { keyboard_id, .. } => {
                f.write_fmt(format_args!("Keyboard {keyboard_id}"))
            }
        }
    }
}

impl ControlDevice {
    pub fn keyboard1() -> ControlDevice {
        ControlDevice::Keyboard {
            keyboard_id: usize::MAX - 32,
            left: KeyCode::Left,
            right: KeyCode::Right,
            top: KeyCode::Up,
            bottom: KeyCode::Down,
            action1: KeyCode::Space,
            action2: KeyCode::G,
            start: KeyCode::Return,
            cancel: KeyCode::Escape,
        }
    }

    pub fn keyboard2() -> ControlDevice {
        ControlDevice::Keyboard {
            keyboard_id: usize::MAX - 31,
            left: KeyCode::A,
            right: KeyCode::D,
            top: KeyCode::W,
            bottom: KeyCode::S,
            action1: KeyCode::C,
            action2: KeyCode::X,
            start: KeyCode::V,
            cancel: KeyCode::B,
        }
    }
    pub fn keyboard3() -> ControlDevice {
        ControlDevice::Keyboard {
            keyboard_id: usize::MAX - 30,
            left: KeyCode::P,
            right: KeyCode::P,
            top: KeyCode::P,
            bottom: KeyCode::P,
            action1: KeyCode::P,
            action2: KeyCode::P,
            start: KeyCode::P,
            cancel: KeyCode::P,
        }
    }
    pub fn keyboard4() -> ControlDevice {
        ControlDevice::Keyboard {
            keyboard_id: usize::MAX - 29,
            left: KeyCode::P,
            right: KeyCode::P,
            top: KeyCode::P,
            bottom: KeyCode::P,
            action1: KeyCode::P,
            action2: KeyCode::P,
            start: KeyCode::P,
            cancel: KeyCode::P,
        }
    }
}
