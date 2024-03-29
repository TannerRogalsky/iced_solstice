use crate::{Backend, Color, Error, Renderer, Settings, Viewport};

use core::ffi::c_void;
use iced_graphics::{Antialiasing, Size};
use iced_native::mouse;

/// A window graphics backend for iced powered by `glow`.
#[allow(missing_debug_implementations)]
pub struct Compositor {
    ctx: solstice::Context,
}

impl iced_graphics::window::GLCompositor for Compositor {
    type Renderer = Renderer;
    type Settings = Settings;

    unsafe fn new(
        settings: Self::Settings,
        loader_function: impl FnMut(&str) -> *const c_void,
    ) -> Result<(Self, Self::Renderer), Error> {
        let gl = solstice::glow::Context::from_loader_function(loader_function);
        let mut ctx = solstice::Context::new(gl);

        // // Enable auto-conversion from/to sRGB
        // gl.enable(glow::FRAMEBUFFER_SRGB);
        //
        // // Enable alpha blending
        // gl.enable(glow::BLEND);
        // gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        //
        // // Disable multisampling by default
        // gl.disable(glow::MULTISAMPLE);

        let renderer = Renderer::new(Backend::new(&mut ctx, settings));

        Ok((Self { ctx }, renderer))
    }

    fn sample_count(settings: &Settings) -> u32 {
        settings
            .antialiasing
            .map(Antialiasing::sample_count)
            .unwrap_or(0)
    }

    fn resize_viewport(&mut self, physical_size: Size<u32>) {
        self.ctx.set_viewport(
            0,
            0,
            physical_size.width as i32,
            physical_size.height as i32,
        );
    }

    fn draw<T: AsRef<str>>(
        &mut self,
        renderer: &mut Self::Renderer,
        viewport: &Viewport,
        color: Color,
        output: &<Self::Renderer as iced_native::Renderer>::Output,
        overlay: &[T],
    ) -> mouse::Interaction {
        let gl = &mut self.ctx;

        let [red, green, blue, alpha] = color.into_linear();

        let clear_color = solstice::Color {
            red,
            blue,
            green,
            alpha,
        };
        solstice::Renderer::clear(
            gl,
            solstice::ClearSettings {
                color: Some(clear_color.into()),
                scissor: None,
                ..Default::default()
            },
        );

        renderer.backend_mut().draw(gl, viewport, output, overlay)
    }
}
