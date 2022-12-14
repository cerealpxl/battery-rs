use crate::App;

use super::{ open_gl, Shader, Vertex, Texture, Quad, Canvas };
use glam::{ Mat4, Vec2, vec2, vec3 };

/// Default Vertex Shader code.
///
const DEFAULT_VERT_CODE: &str = r#"
    #version 330

    uniform mat4 u_matrix;

    layout (location = 0) in vec3 a_position;
    layout (location = 1) in vec3 a_texcoord;
    layout (location = 2) in vec4 a_color;
    layout (location = 3) in vec3 a_type;

    out vec3 o_texcoord;
    out vec4 o_color;
    out vec3 o_type;

    void main(void)
    {
        // Assign the GL Position.
        gl_Position = u_matrix * vec4(a_position.xy, 0, 1);
        
        // Send the texcoord and color data to the fragment.
        o_texcoord = a_texcoord;
        o_color    = a_color;
        o_type     = a_type;
    }
"#;

/// Default Fragment Shader code.
///
const DEFAULT_FRAG_CODE: &str = r#"
    #version 330

    uniform sampler2D u_texture;

    out vec4 a_color;
    in vec3 o_texcoord;
    in vec4 o_color;
    in vec3 o_type;

    void main(void)
    {
        vec4 color = texture(u_texture, o_texcoord.xy);
        a_color =
            o_type.x * color * o_color + 
            o_type.y * color.a * o_color + 
            o_type.z * o_color;
    }
"#;

/// Whether the Batch will draw a Shape or a Texture.
/// 
#[derive(PartialEq)]
#[derive(Clone)]
pub enum BatchModes {
    Shape,
    Texture,
}

/// Group of Vertices.
/// 
pub struct Batch {
    pub mode:    BatchModes,
    pub texture: Option<u32>,

    pub indices_count: u32,
    pub indices_start: u32,

    pub elements: u32,
}

/// A 2D Batcher used to draw shapes, images and textures.
///
pub struct Batcher {
    vertices: Vec<Vertex>,
    batches:  Vec<Batch>,

    vertex_array_object:  u32,
    vertex_buffer_object: u32,

    shader: Shader,
    matrix: Mat4,
    color:  (f32, f32, f32, f32),
}

impl Batcher {
    /// Creates a new Batcher.
    ///
    pub fn new() -> Self {
        open_gl::enable(open_gl::EnableCap::Blend).unwrap();
        open_gl::disable(open_gl::EnableCap::DepthTest).unwrap();

        open_gl::blend_func(
            open_gl::BlendFactor::SrcAlpha, 
            open_gl::BlendFactor::OneMinusSrcAlpha
        ).unwrap();
        
        // Creates the default shader.
        let shader = Shader::new(
            &std::ffi::CString::new(DEFAULT_VERT_CODE).unwrap(), 
            &std::ffi::CString::new(DEFAULT_FRAG_CODE).unwrap()
        ).unwrap();

        // Gen vertex buffers.
        let vertex_array_object  = open_gl::gen_vertex_array().unwrap();
        let vertex_buffer_object = open_gl::gen_buffer().unwrap();

        Self {
            vertices: Vec::new(),
            batches:  Vec::new(),

            vertex_array_object,
            vertex_buffer_object,

            shader,
            matrix: Mat4::IDENTITY,
            color:  (1.0, 1.0, 1.0, 1.0),
        }
    }

    /// Prepare the Batcher for rendering.
    ///
    pub fn origin(&mut self, app: &App) {
        self.viewport(
            app.get_width()  as f32,
            app.get_height() as f32,
        );

        self.vertices.clear();
        self.batches.clear();
    }

    /// Presents the drawn contents of the Batcher.
    ///
    pub fn present(&mut self) {
        self.shader.use_program();

        if self.vertices.len() > 0 {
            let size    = self.vertices.len() * std::mem::size_of::<Vertex>();
            let pointer = self.vertices.as_ptr();

            // Setting up the Vertex Buffer.
            open_gl::bind_buffer(open_gl::BufferTarget::Array, self.vertex_buffer_object).unwrap();
            open_gl::buffer_data(
                open_gl::BufferTarget::Array, 
                size    as        gl::types::GLsizeiptr,
                pointer as *const gl::types::GLvoid, 
                open_gl::BufferUsage::DynamicDraw
            ).unwrap();
            
            // Setting up Vertex Attribute pointers.
            open_gl::bind_buffer(open_gl::BufferTarget::Array, 0).unwrap();
            open_gl::bind_vertex_array(self.vertex_array_object).unwrap();
            open_gl::bind_buffer(open_gl::BufferTarget::Array, self.vertex_buffer_object).unwrap();

            Vertex::attrib_pointers();

            open_gl::bind_buffer(open_gl::BufferTarget::Array, 0).unwrap();
            open_gl::bind_vertex_array(0).unwrap();
        }

        // Loop all batches and draw vertices.
        let texture_location = open_gl::get_uniform_location(self.shader.prog_id, "u_texture").unwrap();
        let matrix_location  = open_gl::get_uniform_location(self.shader.prog_id, "u_matrix").unwrap();

        if self.batches.len() > 0 {
            open_gl::bind_vertex_array(self.vertex_array_object).unwrap();
            open_gl::uniform_matrix_4f(matrix_location, false, &self.matrix.to_cols_array()[0]).unwrap();
            
            for batch in self.batches.iter() {
                // Assign the current texture.
                match batch.texture {
                    Some(texture) => {
                        open_gl::active_texture(0).unwrap();
                        open_gl::bind_texture(open_gl::TextureTarget::Texture2D, texture).unwrap();
                        
                        open_gl::uniform_1i(texture_location, 0).unwrap();
                    },
                    None => {
                        open_gl::bind_texture(open_gl::TextureTarget::Texture2D, 0).unwrap();
                    }
                }
                
                // Render primitives.
                open_gl::draw_arrays(
                    open_gl::PrimitiveType::Triangles, 
                    batch.indices_start as i32,
                    batch.indices_count as usize
                ).unwrap();
            }
        }

        self.vertices.clear();
        self.batches.clear();
    }

    /// Recreates the ortho matrix used for rendering and resize the OpenGL Viewport.
    ///
    pub fn viewport(&mut self, width: f32, height: f32) {
        self.matrix = Mat4::orthographic_rh_gl(
            0.0, 
            width,
            height,
            0.0,
            0.0,
            1.0,
        );

        open_gl::viewport(0, 0, width as i32, height as i32).unwrap();
    }

    /// Sets the Matrix translation.
    ///
    pub fn translate(&mut self, x: f32, y: f32) {
        self.matrix *= Mat4::from_translation(vec3(x, y, 0.0));
    }

    /// Sets the Matrix scale.
    ///
    pub fn scale(&mut self, x: f32, y: f32) {
        self.matrix *= Mat4::from_scale(vec3(x, y, 1.0));
    }

    /// Sets the active draw color.
    /// 
    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = (r, g, b, a);  
    }

    /// Returns the active draw color.
    ///
    pub fn get_color(&mut self) -> (f32, f32, f32, f32) {
        self.color.clone()
    }

    /// Sets the current render target.
    ///
    pub fn set_canvas(&mut self, canvas: &Canvas) {
        self.present();
        self.viewport(canvas.get_width() as f32, canvas.get_height() as f32);

        // Flips vertically the ortho matrix.
        self.scale(1.0, -1.0);
        self.translate(0.0, -(canvas.get_height() as f32));

        open_gl::bind_framebuffer(open_gl::FramebufferTarget::Framebuffer, canvas.handle).unwrap();
    }

    /// Resets the current render target.
    ///
    pub fn reset_canvas(&mut self, app: &App) {
        self.present();
        self.viewport(app.get_width() as f32, app.get_height() as f32);

        open_gl::bind_framebuffer(open_gl::FramebufferTarget::Framebuffer, 0).unwrap();
    }

    /// Returns a valid Batch structure.
    /// 
    fn get_batch(&mut self, mode: BatchModes, texture: Option<u32>) -> &mut Batch {
        if self.batches.len() <= 0 || self.batches[self.batches.len() - 1].mode != mode {
            // If there is no compatible batch, creates a new.
            let mut indices_start = 0;

            if self.vertices.len() > 0 {
                indices_start = self.vertices.len() as u32;
            }

            self.batches.push(Batch {
                mode,
                texture,
                elements:      0,
                indices_count: 0,
                indices_start,
            });
        }

        // Returns the last batch added.
        let index = self.batches.len() - 1;

        &mut self.batches[index]
    }

    /// Push a Tri.
    /// 
    #[inline]
    pub fn push_tri(&mut self, texture: Option<u32>, mode: BatchModes, v0: Vertex, v1: Vertex, v2: Vertex) {
        let mut batch = self.get_batch(mode, texture);

        // Updates the vertex count.
        batch.elements      += 1;
        batch.indices_count += 3;

        // Push vertices.
        self.vertices.push(v0);
        self.vertices.push(v1);
        self.vertices.push(v2);     
    }

    /// Push a Quad.
    /// 
    #[inline]
    pub fn push_quad(&mut self, texture: Option<u32>, mode: BatchModes, v0: Vertex, v1: Vertex, v2: Vertex, v3: Vertex) {
        let mut batch = self.get_batch(mode, texture);

        // Updates the vertex count.
        batch.elements      += 2;
        batch.indices_count += 6;

        // Push vertices.
        self.vertices.push(v0.clone());
        self.vertices.push(v1);
        self.vertices.push(v3.clone());        
        self.vertices.push(v0);
        self.vertices.push(v2);
        self.vertices.push(v3);
    }

    /// Draws a triangle.
    /// 
    pub fn triangle(&mut self, pos0: (f32, f32), pos1: (f32, f32), pos2: (f32, f32)) {
        self.push_tri(
            None,
            BatchModes::Shape,
            Vertex::as_shape(pos0, self.color),
            Vertex::as_shape(pos1, self.color),
            Vertex::as_shape(pos2, self.color)
        )
    }

    /// Draws a rectangle.
    /// 
    pub fn rectangle(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.push_quad(
            None, 
            BatchModes::Shape,
            Vertex::as_shape((x, y), self.color),
            Vertex::as_shape((x + width, y), self.color),
            Vertex::as_shape((x, y + height), self.color),
            Vertex::as_shape((x + width, y + height), self.color)
        );
    }
    
    /// Draws a hollow rectangle.
    ///
    pub fn hollow_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, thickness: f32) {
        if thickness > 0.0 {
            let thick_x = thickness.min(width  / 2.0);
            let thick_y = thickness.min(height / 2.0);

            self.rectangle(x, y, width, thick_y);
            self.rectangle(x, y + height - thick_y, width, thick_y);
            self.rectangle(x, y + thick_y, thick_x, height - thick_y * 2.0);
            self.rectangle(x + width - thick_x, y + thick_y, thick_x, height - thick_y * 2.0);
        }
    }

    /// Draws a circle.
    ///
    pub fn circle(&mut self, x: f32, y: f32, radius: f32) {
        let mut last = Vec2::from_angle(0.0) * radius;
        let tau      = std::f32::consts::TAU;
        let step     = 20.0;

        for i in 1..(step as i32 + 1) {
            let next = Vec2::from_angle((i as f32) / step * tau) * radius;

            self.push_tri(
                None, 
                BatchModes::Shape,
                Vertex::as_shape((x + last.x, y + last.y), self.color),
                Vertex::as_shape((x + next.x, y + next.y), self.color),
                Vertex::as_shape((x, y), self.color)
            );

            last = next;
        }
    }

    /// Draws a texture.
    ///
    pub fn texture(
        &mut self, 
        texture:   &Texture,
        x:         f32,
        y:         f32,
        quad:      Option<Quad>,
        angle:     Option<f32>,
        scale:     Option<(f32, f32)>,
        origin:    Option<(f32, f32)>
    ) {
        let mut width  = texture.get_width()  as f32;
        let mut height = texture.get_height() as f32;

        let mut uv0 = (0.0, 0.0);
        let mut uv1 = (1.0, 0.0);
        let mut uv2 = (0.0, 1.0);
        let mut uv3 = (1.0, 1.0);

        match quad {
            Some(mut quad) => {
                width  = quad.width;
                height = quad.height;

                let uvs = quad.get_vertex_texcoords();
                uv0 = uvs[0].clone();
                uv1 = uvs[1].clone();
                uv2 = uvs[2].clone();
                uv3 = uvs[3].clone();
            },
            None => {}
        }

        let mut pos0 = vec2(0.0,   0.0);
        let mut pos1 = vec2(width, 0.0);
        let mut pos2 = vec2(0.0,   height);
        let mut pos3 = vec2(width, height);
        
        // Origin offset.
        match origin {
            Some(origin) => {
                let origin = vec2(origin.0, origin.1);
                pos0 -= origin;
                pos1 -= origin;
                pos2 -= origin;
                pos3 -= origin;
            },
            None => {}
        }

        // Scale.
        match scale {
            Some(scale) => {
                let scale = vec2(scale.0, scale.1);
                pos0 *= scale;
                pos1 *= scale;
                pos2 *= scale;
                pos3 *= scale;
            },
            None => {}
        }

        // Rotation.
        match angle {
            Some(angle) => {
                if angle != 0.0 {
                    let angle = vec2(angle.sin(), angle.cos());
                    pos0 = pos0.rotate(angle);
                    pos1 = pos1.rotate(angle);
                    pos2 = pos2.rotate(angle);
                    pos3 = pos3.rotate(angle);
                }
            },
            None => {}
        }

        // Translates to the Position.
        let position = vec2(x, y);
        pos0 += position;
        pos1 += position;
        pos2 += position;
        pos3 += position;

        self.push_quad(
            Some(texture.handle), 
            BatchModes::Texture,
            Vertex::as_texture(pos0.into(), uv0, self.color),
            Vertex::as_texture(pos1.into(), uv1, self.color),
            Vertex::as_texture(pos2.into(), uv2, self.color),
            Vertex::as_texture(pos3.into(), uv3, self.color)
        );
    }
    
    /// Draws a canvas.
    ///
    pub fn canvas(&mut self, 
        canvas:    &Canvas, 
        x:         f32,
        y:         f32,
        quad:      Option<Quad>,
        angle:     Option<f32>,
        scale:     Option<(f32, f32)>,
        origin:    Option<(f32, f32)>
    ) {
        self.texture(&canvas.texture, x, y, quad, angle, scale, origin);
    }
}