use crate::graphics::open_gl;

/// A 2D texture used for rendering.
/// 
pub struct Texture {
    width:  u32,
    height: u32,

    pub handle: u32,
}

impl Texture {
    /// Creates an empty Texture, used as a placeholder.
    /// 
    pub fn empty() -> Self {
        Self {
            width:  0,
            height: 0,
            handle: 0,
        }
    }

    /// Creates a new Texture.
    /// 
    pub fn new() -> Self {
        let handle = open_gl::gen_texture().unwrap();

        Self {
            width:  0,
            height: 0,
            handle,
        }
    }

    /// Loads the image stored in the given path.
    /// 
    pub fn from_path(self, location: &str) -> Result<Self, image::ImageError> {
        let raw_image = image::open(std::path::Path::new(location))?;

        Ok(self.from_dynamic_image(raw_image))
    }

    /// Loads the image from a DynamicImage.
    pub fn from_dynamic_image(mut self, raw_image: image::DynamicImage) -> Self {     
        self.width  = raw_image.width();
        self.height = raw_image.height();

        // Gen texture.
        open_gl::bind_texture(open_gl::TextureTarget::Texture2D, self.handle).unwrap();
        open_gl::tex_image_2d(
            open_gl::TextureTarget::Texture2D,
            0,
            open_gl::TextureFormat::RGBA,
            raw_image.width()  as i32,
            raw_image.height() as i32,
            0,
            open_gl::PixelFormat::RGBA,
            open_gl::PixelType::UnsignedByte,
            raw_image.as_bytes()
        ).unwrap();

        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        self
    }

    /// Returns the width of the Texture.
    /// 
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the Texture.
    /// 
    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        open_gl::delete_texture(self.handle).unwrap();
    }
}