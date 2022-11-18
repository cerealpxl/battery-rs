extern crate sdl2;
extern crate gl;
extern crate glam;
extern crate image;

pub mod graphics;
pub use graphics::{ Shader, Batcher, Texture, Quad };

pub mod system;
pub use system::{ App, Configuration, KeyCode, MouseButton };

#[cfg(test)]
mod tests {
    use super::*;

    struct Game;
    impl Configuration for Game {
        fn startup(&mut self, _app: &mut App)  {}
        fn shutdown(&mut self, _app: &mut App)  {}
        fn update(&mut self, _app: &mut App) {}
        fn render(&mut self, _app: &mut App) {}
    }

    #[test]
    pub fn window_test() {
        App::new("Nice window!", 320, 240, 60.0).start(&mut Game {}).unwrap();
    }

    struct BatcherGame {
        batcher: Option<graphics::Batcher>,
        texture: graphics::Texture,
        rect_position: (f32, f32)
    }

    impl Configuration for BatcherGame {
        fn startup(&mut self, _app: &mut App)  {
            self.batcher = Some(graphics::Batcher::new());
            self.texture = graphics::Texture::new().from_path("ferris.png").unwrap();
        }

        fn update(&mut self, app: &mut App) {
            if app.input.key_down(KeyCode::Left) {
                self.rect_position.0 -= 16.0;
            }
            if app.input.key_down(KeyCode::Right) {
                self.rect_position.0 += 16.0;
            }

            let fps = app.timer.get_fps();
            app.set_title(fps.to_string().as_ref());
        }

        fn render(&mut self, app: &mut App) {
            let batcher = self.batcher.as_mut().expect("Osh?");

            batcher.origin(app);
            batcher.rectangle(16.0, 8.0, 16.0, 16.0);
            batcher.texture(
                &self.texture, 
                self.rect_position,
                Some(Quad::from_texture((0.0, 0.0), (128.0, 128.0), &self.texture)),
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

        fn shutdown(&mut self, _app: &mut App) {}
    }

    #[test]
    pub fn batcher_test() {
        App::new("Nice window!", 320, 240, 60.0).start(&mut BatcherGame {
            batcher: None,
            texture: graphics::Texture::empty(),
            rect_position: (0.0, 0.0)
        }).unwrap();
    }
}