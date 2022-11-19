use crate::graphics::open_gl;
use super::{ Timer, Input };
use sdl2;
use sdl2::event::{ Event, WindowEvent };

/// Trait that implements main loop callbacks.
///
pub trait Configuration {
    /// Called when starting the Application.
    /// 
    fn startup(&mut self, app: &mut App);

    /// Called when closing the Application.
    ///
    fn shutdown(&mut self, app: &mut App);

    /// Do logic here. Called in every frame of the Application.
    ///
    fn update(&mut self, app: &mut App);
    
    /// Called in Application's Rendering state.
    ///
    fn render(&mut self, app: &mut App);
}

/// Battery Application.
///
pub struct App {
    sdl_context: sdl2::Sdl,
    sdl_video:   sdl2::VideoSubsystem,
    sdl_window:  sdl2::video::Window,

    running:             bool,
    focused:             bool,
    run_while_unfocused: bool,

    title:  String,
    width:  u32,
    height: u32,

    pub input: Input,
    pub timer: Timer,
}

impl App {
    /// Creates a new Application.
    ///
    pub fn new(title: &str, width: u32, height: u32, framerate: f64) -> Self {
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

            running:             false,
            focused:             true,
            run_while_unfocused: false,

            title: String::from(title),
            width,
            height,

            input: Input::new(),
            timer: Timer::new(framerate),
        }
    }

    /// Starts running the Application.
    ///
    pub fn start(mut self, config: &mut impl Configuration) -> Result<(), String> {
        if self.running {
            return Err("App already running".to_string());
        }

        // Load OpenGL Function pointers.
        let _context = self.sdl_window.gl_create_context().unwrap();
        let _gl      = open_gl::load(|s| self.sdl_video.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // Starting up the Application.
        self.running = true;
        self.focused = true;
        config.startup(&mut self);

        // Main loop.
        let mut events = self.sdl_context.event_pump().unwrap();

        while self.running {
            self.input.update();

            // Process Sdl Events.
            for event in events.poll_iter() {
                self.poll_event(event);
            }

            if self.focused || self.run_while_unfocused {
                self.timer.update();

                // Updates the Application when our accumulator exceeds the frame duration.
                while self.timer.frame_accumulator >= 1.0 / self.timer.frame_rate {
                    self.timer.frame_counter     += 1;
                    self.timer.frame_accumulator -= 1.0 / self.timer.frame_rate;

                    config.update(&mut self);
                }

                // Renders the Application.
                if self.running {
                    open_gl::clear(open_gl::ClearMode::Color).unwrap();
                    open_gl::clear_color(0.0, 0.0, 0.0, 1.0).unwrap();

                    config.render(&mut self);
                    self.sdl_window.gl_swap_window();
                }
            }

            // It avoids the high CPU consumption.
            std::thread::sleep(std::time::Duration::new(0, 10000000));
        }

        // Closing the Application.
        config.shutdown(&mut self);

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
            Event::KeyDown { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                match scancode {
                    Some(code) => self.input.do_key_down(code),
                    None => {},
                }
            },
            Event::KeyUp { timestamp: _, window_id: _, keycode: _, scancode, keymod: _, repeat: _ } => {
                match scancode {
                    Some(code) => self.input.do_key_up(code),
                    None => {},
                }
            },
            Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x: _, y: _ } => {
                self.input.do_mouse_down(mouse_btn);
            },
            Event::MouseButtonUp { timestamp: _, window_id: _, which: _, mouse_btn, clicks: _, x: _, y: _ } => {
                self.input.do_mouse_up(mouse_btn);
            },
            Event::MouseMotion { timestamp: _, window_id: _, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                self.input.do_mouse_move((x as f32, y as f32));
            },
            Event::ControllerDeviceAdded { timestamp: _, which } => {
                self.input.do_controller_added(which);
            },
            Event::ControllerDeviceRemoved { timestamp: _, which } => {
                self.input.do_controller_removed(which);
            },
            Event::ControllerButtonDown { timestamp: _, which, button } => {
                self.input.do_controller_down(which, button);
            },
            Event::ControllerButtonUp { timestamp: _, which, button } => {
                self.input.do_controller_up(which, button);
            },

            _ => {},
        }
    }

    /// Sets the size of the window.
    ///
    pub fn set_size(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            self.width  = width;
            self.height = height;

            self.sdl_window.set_size(width, height).unwrap();
        }
    }

    /// Returns the size of the window in a tuple.
    ///
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Returns the width of the window.
    ///
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the window.
    ///
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Sets the title of the Main Window.
    ///
    pub fn set_title(&mut self, title: &str) {
        if self.title != String::from(title) {
            self.title = String::from(title);

            self.sdl_window.set_title(title).unwrap();
        }
    }

    /// Returns the title of the Main Window.
    ///
    pub fn get_title(&mut self) -> &str {
        self.title.as_ref()
    }
}
