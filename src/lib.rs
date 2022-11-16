extern crate sdl2;
extern crate gl;
extern crate glam;
extern crate image;

pub mod system;
pub use system::{ App, Config, };

#[cfg(test)]
mod tests {
    use super::*;

    struct Game;
    impl Config for Game {
        fn startup(&mut self)  {}
        fn shutdown(&mut self)  {}
        fn update(&mut self) {}
        fn render(&mut self) {}
    }

    #[test]
    pub fn window_test() {
        App::create("Nice window!", 320, 240).start(&mut Game {}).unwrap();
    }
}