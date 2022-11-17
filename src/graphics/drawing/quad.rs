use crate::graphics::Texture;

/// Used for Texture rendering, selecting a part of a Texture to draw.
/// Useful for Sprite Atlases.
///
/// See :. ['https://github.com/love2d/love/blob/main/src/modules/graphics/Quad.cpp']
///
pub struct Quad {
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,

    vertex_positions: [(f32, f32); 4],
    vertex_texcoords: [(f32, f32); 4],
}

impl Quad {
    /// Creates a new Quad.
    ///
    pub fn new(position: (f32, f32), size: (f32, f32), source: (f32, f32)) -> Self {
        Self {
            x:      position.0,
            y:      position.1,
            width:  size.0,
            height: size.1,

            vertex_positions: [
                (0.0,    0.0   ),
                (size.0, 0.0   ),
                (0.0,    size.1),
                (size.0, size.1)
            ],

            vertex_texcoords: [
                ( position.0           / source.0,  position.1           / source.1),
                ((position.0 + size.0) / source.0,  position.1           / source.1),
                ( position.0           / source.0, (position.1 + size.1) / source.1),
                ((position.0 + size.0) / source.0, (position.1 + size.1) / source.1),
            ],
        }
    }

    /// Creates a new Quad based in a Texture as Source.
    ///
    pub fn from_texture(position: (f32, f32), size: (f32, f32), texture: &Texture) -> Self {
        Self::new(
            position, 
            size, 
            (texture.get_width() as f32, texture.get_height() as f32)
        )
    }

    /// Returns the Quad Vertex positions.
    ///
    pub fn get_vertex_positions(&mut self) -> &[(f32, f32); 4] {
        &self.vertex_positions
    }

    /// Returns the Quad Vertex texcoords.
    ///
    pub fn get_vertex_texcoords(&mut self) -> &[(f32, f32); 4] {
        &self.vertex_texcoords
    }
}