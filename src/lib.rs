extern crate sdl2;
extern crate gl;
extern crate glam;
extern crate image;

pub mod graphics;
pub use graphics::{ Shader, Batcher, };

pub mod system;
pub use system::{ App, Configuration, };


#[cfg(test)]
mod tests {
    use super::*;

    struct Game;
    impl Configuration for Game {
        fn startup(&mut self)  {}
        fn shutdown(&mut self)  {}
        fn update(&mut self) {}
        fn render(&mut self, _app: &mut App) {}
    }

    #[test]
    pub fn window_test() {
        App::new("Nice window!", 320, 240).start(&mut Game {}).unwrap();
    }

    struct BatcherGame {
        batcher: Option<graphics::Batcher>,
        texture: graphics::Texture,
    }

    impl Configuration for BatcherGame {
        fn startup(&mut self)  {
            self.batcher = Some(graphics::Batcher::new());
            self.texture = graphics::Texture::new().from_path("ferris.png").unwrap();
        }

        fn render(&mut self, app: &mut App) {
            let batcher = self.batcher.as_mut().expect("Osh?");

            batcher.origin(app);
            batcher.rectangle(16.0, 8.0, 16.0, 16.0);
            batcher.texture(
                &self.texture, 
                (16.0, 16.0),
                Some(0.4),
                Some((1.4, 1.0)),
                None,
            );
            batcher.set_color(1.0, 0.0, 0.0, 1.0);
            batcher.triangle(
                (16.0, 32.0),
                (32.0, 32.0),
                (32.0, 48.0),
            );
            batcher.set_color(1.0, 1.0, 1.0, 1.0);
            batcher.present();
        }

        fn shutdown(&mut self)  {}
        fn update(&mut self) {}
    }

    #[test]
    pub fn batcher_test() {
        App::new("Nice window!", 320, 240).start(&mut BatcherGame {
            batcher: None,
            texture: graphics::Texture::empty(),
        }).unwrap();
    }
}