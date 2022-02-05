//! Display an interactive selector of a single value from a range of values.
//!
//! A [`Slider`] has some local [`State`].
pub use iced_graphics::slider::{Handle, HandleShape, Style, StyleSheet};
pub use iced_native::widget::slider::State;

/// An horizontal bar and a handle that selects a single value from a range of
/// values.
///
/// This is an alias of an `iced_native` slider with an `iced_wgpu::Renderer`.
pub type Slider<'a, T, Message> = iced_native::widget::Slider<'a, T, Message>;
