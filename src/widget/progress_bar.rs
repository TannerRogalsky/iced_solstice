//! Allow your users to visually track the progress of a computation.
//!
//! A [`ProgressBar`] has a range of possible values and a current value,
//! as well as a length, height and style.
pub use iced_graphics::progress_bar::{Style, StyleSheet};

/// A bar that displays progress.
///
/// This is an alias of an `iced_native` progress bar with an
/// `iced_wgpu::Renderer`.
pub type ProgressBar<'a> = iced_native::widget::ProgressBar<'a>;
