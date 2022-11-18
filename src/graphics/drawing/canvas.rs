use crate::graphics::{ open_gl, Texture };
use image::DynamicImage;

/// Used for off-screen rendering, a invisible surface that you can draw to but that will be 
/// invisible until you draw it(Confusing?).
///
pub struct Canvas {
    pub handle:  u32,
    pub texture: Texture,
}

impl Canvas {
    /// Creates a new Canvas.
    ///
    pub fn new(width: u32, height: u32) -> Self {
        let bitmap  = DynamicImage::new_rgba8(width, height);
        let texture = Texture::new().from_dynamic_image(bitmap);

        // Gera o framebuffer.
        let handle = open_gl::gen_framebuffer().unwrap();
        open_gl::bind_framebuffer(open_gl::FramebufferTarget::Framebuffer, handle).unwrap();
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.handle,
                0,
            );

            gl::DrawBuffer(gl::COLOR_ATTACHMENT0);
        }
        open_gl::bind_framebuffer(open_gl::FramebufferTarget::Framebuffer, 0).unwrap();

        Self {
            handle,
            texture,
        }
    }

    /// Returns the width of the Canvas.
    ///
    pub fn get_width(&self) -> u32 {
        self.texture.get_width()
    }

    /// Returns the height of the Canvas.
    ///
    pub fn get_height(&self) -> u32 {
        self.texture.get_height()
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        open_gl::delete_framebuffer(self.handle).unwrap();
    }
}