//! A [`glow`] renderer for [`iced_native`].
//!
//! ![The native path of the Iced ecosystem](https://github.com/hecrj/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/native.png?raw=true)
//!
//! [`glow`]: https://github.com/grovesNL/glow
//! [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod backend;
mod program;
mod quad;
mod text;
mod triangle;

pub mod settings;

pub use backend::Backend;
pub use settings::Settings;

pub(crate) use iced_graphics::Transformation;

pub use iced_graphics::{Error, Viewport};
pub use iced_native::{
    alignment::{Horizontal as HorizontalAlignment, Vertical as VerticalAlignment},
    Background, Color, Command, Length, Vector,
};

/// A [`glow`] graphics renderer for [`iced`].
///
/// [`glow`]: https://github.com/grovesNL/glow
/// [`iced`]: https://github.com/hecrj/iced
pub type Renderer = iced_graphics::Renderer<Backend>;
