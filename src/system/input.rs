use std::collections::HashSet;

pub type KeyCode     = sdl2::keyboard::Scancode;
pub type MouseButton = sdl2::mouse::MouseButton;

/// Structure that handle the Input behavior.
///
pub struct Input {
    keyboard_down:     HashSet<KeyCode>,
    keyboard_pressed:  HashSet<KeyCode>,
    keyboard_released: HashSet<KeyCode>,

    mouse_down:     HashSet<MouseButton>,
    mouse_pressed:  HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    pub mouse_position: (f32, f32),
}

impl Input {
    /// Starts the Input.
    ///
    pub fn new() -> Self {
        Input {
            keyboard_down:     HashSet::new(),
            keyboard_pressed:  HashSet::new(),
            keyboard_released: HashSet::new(),

            mouse_down:     HashSet::new(),
            mouse_pressed:  HashSet::new(),
            mouse_released: HashSet::new(),

            mouse_position: (0.0, 0.0),
        }
    }

    /// Updates the input state.
    ///
    pub fn update(&mut self) {
        self.keyboard_pressed  = HashSet::new();
        self.keyboard_released = HashSet::new();

        self.mouse_pressed  = HashSet::new();
        self.mouse_released = HashSet::new();
    }

    /// Checks whether the given key is being pressed.
    ///
    pub fn key_down(&mut self, key: KeyCode) -> bool {
        self.keyboard_down.contains(&key)
    }

    /// Checks whether the given key has been pressed.
    ///
    pub fn key_pressed(&mut self, key: KeyCode) -> bool {
        self.keyboard_pressed.contains(&key)
    }

    /// Checks whether the given key has been released.
    ///
    pub fn key_released(&mut self, key: KeyCode) -> bool {
        self.keyboard_released.contains(&key)
    }

    /// Checks whether the given button is being pressed.
    ///
    pub fn mouse_down(&mut self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    /// Checks whether the given button has been pressed.
    ///
    pub fn mouse_pressed(&mut self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    /// Checks whether the given button has been released.
    ///
    pub fn mouse_released(&mut self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    /// Returns the mouse position.
    ///
    pub fn get_mouse_position(&mut self) -> (f32, f32) {
        self.mouse_position
    }

    /// Updates the given key when pressed.
    ///
    pub fn do_key_down(&mut self, key: KeyCode) {
        if !self.keyboard_down.contains(&key) {
            self.keyboard_down.insert(key);
            self.keyboard_pressed.insert(key);
        }
    }

    /// Updates the given key when released.
    ///
    pub fn do_key_up(&mut self, key: KeyCode) {
        self.keyboard_down.remove(&key);

        if !self.keyboard_released.contains(&key) {
            self.keyboard_released.insert(key);
        }
    }

    /// Updates the given button when pressed.
    ///
    pub fn do_mouse_down(&mut self, button: MouseButton) {
        if !self.mouse_down.contains(&button) {
            self.mouse_down.insert(button);
            self.mouse_pressed.insert(button);
        }
    }

    /// Updates the given button when released.
    ///
    pub fn do_mouse_up(&mut self, button: MouseButton) {
        self.mouse_down.remove(&button);

        if !self.mouse_released.contains(&button) {
            self.mouse_released.insert(button);
        }
    }

    /// Assign the mouse position.
    ///
    pub fn do_mouse_move(&mut self, position: (f32, f32)) {
        self.mouse_position = position.clone();
    }
}