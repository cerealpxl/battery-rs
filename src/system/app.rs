use sdl2;
use sdl2::event::{ Event, WindowEvent };

/// Trait that implements main loop callbacks.
///
pub trait Config {
    fn startup(&mut self);
    fn shutdown(&mut self);
    fn update(&mut self);
    fn render(&mut self);
}

/// Battery Application.
///
pub struct App {
    sdl_context: sdl2::Sdl,
    sdl_video:   sdl2::VideoSubsystem,
    sdl_window:  sdl2::video::Window,

    running: bool,
    focused: bool,
}

impl App {
    /// Creates a new Application.
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
    pub fn start(&mut self, config: &mut impl Config) -> Result<(), String> {
        if self.running {
            return Err("App already running".to_string());
        }

        // Load OpenGL Function pointers.
        let _context = self.sdl_window.gl_create_context().unwrap();
        let _gl      = gl::load_with(|s| self.sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // Starting up the Application.
        self.running = true;
        self.focused = true;
        config.startup();

        // Main loop.
        let mut events = self.sdl_context.event_pump().unwrap();

        while self.running {
            for event in events.poll_iter() {
                self.poll_event(event);
            }

            if self.focused {
                config.update();

                // Renders the Application.
                if self.running {
                    unsafe {
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                    }

                    config.render();
    
                    self.sdl_window.gl_swap_window();
                }
            }

            // It avoids the high CPU consumption.
            std::thread::sleep(std::time::Duration::new(0, 10000000));
        }

        // Closing the Application.
        config.shutdown();

        Ok(())
    }

    /// Closes the Window.
    ///
    pub fn close(&mut self) {
        self.running = false;
        self.focused = false;
    }

    /// Process Window events.
    ///
    pub fn poll_event(&mut self, event: Event) {
        match event {
            Event::Window { timestamp: _, window_id: _, win_event} => {
                match win_event {
                    WindowEvent::Close       { .. } => self.close(),
                    WindowEvent::FocusGained { .. } => self.focused = true,
                    WindowEvent::FocusLost   { .. } => self.focused = false,

                    _ => {},
                }
            },
            Event::Quit { .. } => self.close(),

            _ => {},
        }
    }
}
