//! Sottosistema grafico CosmicOS.
//! Espone: Color, Renderer, Font (8×8 bitmap classico).

pub mod color;
pub mod font;
pub mod renderer;

pub use color::Color;
pub use renderer::Renderer;
