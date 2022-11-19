use std::collections::HashSet;

pub type KeyCode       = sdl2::keyboard::Scancode;
pub type MouseButton   = sdl2::mouse::MouseButton;
pub type GamepadButton = sdl2::controller::Button;

/// Structure that handles gamepad button states.
/// 
pub struct Gamepad {
    down:     HashSet<GamepadButton>,
    pressed:  HashSet<GamepadButton>,
    released: HashSet<GamepadButton>,

    guid: u32,
}

/// Structure that handle the Input behavior.
///
pub struct Input {
    keyboard_down:     HashSet<KeyCode>,
    keyboard_pressed:  HashSet<KeyCode>,
    keyboard_released: HashSet<KeyCode>,

    mouse_down:     HashSet<MouseButton>,
    mouse_pressed:  HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    gamepads:      Vec<Gamepad>,
    gamepad_limit: u32,

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

            gamepads:      Vec::new(),
            gamepad_limit: 0,

            mouse_position: (0.0, 0.0),
        }
    }

    /// Updates the input state.
    ///
    pub fn update(&mut self) {
        self.keyboard_pressed.clear();
        self.keyboard_released.clear();

        self.mouse_pressed.clear();
        self.mouse_released.clear();

        if self.gamepads.len() > 0 {
            for gamepad in self.gamepads.iter_mut() {
                gamepad.pressed.clear();
                gamepad.released.clear();
            }
        }
    }

    //
    // >> Keyboard
    //

    /// Check if the key is held down.
    ///
    pub fn key_down(&mut self, key: KeyCode) -> bool {
        self.keyboard_down.contains(&key)
    }

    /// Check if the key has been pressed.
    ///
    pub fn key_pressed(&mut self, key: KeyCode) -> bool {
        self.keyboard_pressed.contains(&key)
    }

    /// Check if the key has been released.
    ///
    pub fn key_released(&mut self, key: KeyCode) -> bool {
        self.keyboard_released.contains(&key)
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

    //
    // >> Mouse
    //

    /// Check if the button is held down.
    ///
    pub fn mouse_down(&mut self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    /// Check if the button has been pressed.
    ///
    pub fn mouse_pressed(&mut self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    /// Checks if the button has been released.
    ///
    pub fn mouse_released(&mut self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    /// Returns the mouse position.
    ///
    pub fn get_mouse_position(&mut self) -> (f32, f32) {
        self.mouse_position
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

    //
    // >> Gamepad
    //

    /// Check if the button is held down.
    ///
    pub fn button_down(&mut self, button: GamepadButton) -> bool {
        for gamepad in self.gamepads.iter(){
            if gamepad.down.contains(&button) {
                return true;
            }
        }

        false
    }

    /// Check if the button has been pressed.
    ///
    pub fn button_pressed(&mut self, button: GamepadButton) -> bool {
        for gamepad in self.gamepads.iter(){
            if gamepad.pressed.contains(&button) {
                return true;
            }
        }

        false
    }

    /// Checks if the button has been released.
    ///
    pub fn button_released(&mut self, button: GamepadButton) -> bool {
        for gamepad in self.gamepads.iter(){
            if gamepad.released.contains(&button) {
                return true;
            }
        }

        false
    }

    /// Gets a compatible gamepad.
    ///
    fn get_gamepad(&mut self, guid: u32) -> usize {
        if self.gamepads.len() > 0 {
            for i in 0..(self.gamepads.len() - 1) {
                if self.gamepads[i].guid == guid {
                    return i;
                }
            }
        }

        self.gamepad_limit as usize
    }

    /// Called when a gamepad is being added.
    ///
    pub fn do_gamepad_added(&mut self, guid: u32) {
        self.gamepads.push(Gamepad {
            down:     HashSet::new(),
            pressed:  HashSet::new(),
            released: HashSet::new(),

            guid
        });
    }

    /// Called when a gamepad is being removed.
    ///
    pub fn do_gamepad_removed(&mut self, guid: u32) {
        let gamepad = self.get_gamepad(guid);

        if gamepad < self.gamepads.len() {
            self.gamepads.remove(gamepad);
        }
    }

    /// Called when a gamepad button is held down.
    ///
    pub fn do_gamepad_down(&mut self, guid: u32, button: GamepadButton) {
        let gamepad = self.get_gamepad(guid);

        if gamepad != self.gamepad_limit as usize {
            self.gamepads[gamepad].down.insert(button);
            self.gamepads[gamepad].pressed.insert(button);
        }
    }

    /// Called when a gamepad button is being released.
    ///
    pub fn do_gamepad_up(&mut self, guid: u32, button: GamepadButton) {
        let gamepad = self.get_gamepad(guid);

        if gamepad != self.gamepad_limit as usize {
            self.gamepads[gamepad].down.remove(&button);
            self.gamepads[gamepad].released.insert(button);
        }
    }
}