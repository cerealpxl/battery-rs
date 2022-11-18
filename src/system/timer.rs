use std::time::{ SystemTime, UNIX_EPOCH};

/// Structure that provides timing functionality.
///
pub struct Timer {
    elapsed:          f64,
    previous_elapsed: f64,

    pub delta_time:        f64,
    pub frame_rate:        f64,
    pub frame_accumulator: f64,
    pub frame_counter:     u32,

    fps:        u32,
    fps_update: f64,
}

impl Timer {
    /// Creates a new Timer.
    ///
    pub fn new(frame_rate: f64) -> Self {
        Self {
            elapsed:          Self::get_time(),
            previous_elapsed: Self::get_time(),

            delta_time: 0.0,
            frame_rate,
            frame_accumulator: 0.0,
            frame_counter:     0,

            fps:        0,
            fps_update: 0.0,
        }
    }

    /// Updates the time between two frames.
    ///
    pub fn update(&mut self) {
        self.elapsed          = Self::get_time();
        self.delta_time       = self.elapsed - self.previous_elapsed;
        self.previous_elapsed = self.elapsed;

        // Handle unexpected anomalies like overflow, extremely slow frames, etc.
        if self.delta_time >= 1.0 / self.frame_rate * 8.0 {
            self.delta_time = 1.0 / self.frame_rate;
        }

        if self.delta_time < 0.0 {
            self.delta_time = 0.0;
        }

        // Snaps the delta time to a nice framerate.
        if (self.delta_time - 1.0/120.0).abs() < 0.0002 {
            self.delta_time = 1.0/120.0;
        }

        if (self.delta_time - 1.0/60.0).abs() < 0.0002 {
            self.delta_time = 1.0/60.0;
        }

        if (self.delta_time - 1.0/30.0).abs() < 0.0002 {
            self.delta_time = 1.0/30.0;
        }

        if (self.delta_time - 1.0/15.0).abs() < 0.0002 {
            self.delta_time = 1.0/15.0;
        }

        self.frame_accumulator += self.delta_time;

        // Prevent from crashes.
        if self.frame_accumulator >= 1.0 / self.frame_rate * 8.0 {
            self.delta_time        = 1.0 / self.frame_rate;
            self.frame_accumulator = 0.0;
        }

        // Updates the FPS.
        if self.elapsed - self.fps_update > 1.0 {
            self.fps           = ((self.frame_counter as f64)/(self.elapsed - self.fps_update)).round() as u32;
            self.fps_update    = self.elapsed;
            self.frame_counter = 0;
        }
    }

    /// Returns the current number of frames per second.
    ///
    pub fn get_fps(&mut self) -> u32 {
        self.fps
    }
    
    /// Returns the actual time based in the PC Clock.
    ///
    pub fn get_time() -> f64 {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        time.as_secs_f64()
    }
}