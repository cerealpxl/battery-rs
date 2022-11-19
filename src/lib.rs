extern crate sdl2;
extern crate gl;
extern crate glam;
extern crate image;

pub mod graphics;
pub use graphics::{ Shader, Batcher, Texture, Quad, Canvas };

pub mod system;
pub use system::{ App, Configuration, KeyCode, MouseButton };

#[cfg(test)]
mod tests {
    use super::*;

    struct HelloWorld {
        angle:   f32,
        batcher: Option<Batcher>,
    }

    impl Configuration for HelloWorld {
        /// Creates the Batcher at the Startup.
        /// This is called after the OpenGL Setup,
        ///
        fn startup(&mut self, _app: &mut App) {
            println!("Hello there.");

            self.batcher = Some(Batcher::new());
        }

        /// Called when the game closes.
        /// 
        fn shutdown(&mut self, _app: &mut App) {
            println!("Bye-bye");
        }

        /// Called every frame of the Application.
        ///
        fn update(&mut self, _app: &mut App) {
            self.angle += 0.1;
        }

        /// Draws a nice rectangle.
        ///
        fn render(&mut self, app: &mut App) {
            let batcher = self.batcher.as_mut().expect("Where is my nice batcher?");

            // Creates a new canvas.
            let canvas  = Canvas::new(16, 16);

            // Resets the current drawing context.
            batcher.origin(app);

            // Sets the current canvas and draws a rectangle.
            batcher.set_canvas(&canvas);
            batcher.hollow_rectangle(0.0, 0.0, 16.0, 16.0, 2.0);

            // Sends the drawing data to the Canvas.
            batcher.reset_canvas(app);

            // Draws the Canvas using the center as it origin point.
            batcher.canvas(
                &canvas,
                (app.get_width()  as f32) / 2.0 - 8.0, 
                (app.get_height() as f32) / 2.0 - 8.0,
                None,
                Some(self.angle),
                None,
                Some((8.0, 8.0)),
            );
                
            // Finally, draw the screen.
            batcher.present();
        }
    }

    #[test]
    pub fn hello_world() {
        App::new("Nice window!", 320, 240, 60.0).start(&mut HelloWorld {
            angle:   0.0,
            batcher: None,
        }).unwrap();
    }
}