//! Sottosistema di input CosmicOS.

pub mod keyboard;

/// Evento generico di input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Char(char),
    Backspace,
    Enter,
    Tab,
    Escape,
    Up,
    Down,
    Left,
    Right,
}
