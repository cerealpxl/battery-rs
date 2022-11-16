extern crate sdl2;
extern crate gl;
extern crate glam;
extern crate image;

pub struct Platform {
    sdl_context: sdl2::Sdl,
    sdl_video:   sdl2::VideoSubsystem,
    sdl_window:  sdl2::video::Window,

    running: bool,
    focused: bool,
}

impl Platform {
    /// Creates a new SDL Platform.
    ///
    pub fn create(title: &str, width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video   = sdl_context.video().unwrap();
        let sdl_window  = sdl_video.window(title, width, height)
            .opengl()
            .build()
            .unwrap();

        // OpenGL Attributes.
        let gl_attrib = sdl_video.gl_attr();

        gl_attrib.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attrib.set_context_version(3, 3);

        // Return the SDL Platform.
        Self {
            sdl_context,
            sdl_video,
            sdl_window,

            running: false,
            focused: true,
        }
    }

    /// Starts running the Application.
    ///
    pub fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("App already running".to_string());
        }

        self.running = true;
        self.focused = true;

        let mut events = self.sdl_context.event_pump().unwrap();
        while self.running {
            for event in events.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { timestamp } => self.running = false,

                    _ => {},
                }
            }

            std::thread::sleep(std::time::Duration::new(0, 10000000));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn main_test() {
        let mut platform = Platform::create("Nice window!", 320, 240);

        platform.start().unwrap();
    }
}