//! Display a horizontal or vertical rule for dividing content.

pub use iced_graphics::rule::{FillMode, Style, StyleSheet};

/// Display a horizontal or vertical rule for dividing content.
///
/// This is an alias of an `iced_native` rule with an `iced_glow::Renderer`.
pub type Rule<'a> = iced_native::widget::Rule<'a>;
