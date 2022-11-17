use gl;

/// Estrutura usada para mandar informações ao Shader.
/// 
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32__Vec3 {
    pub num1: f32,
    pub num2: f32,
    pub num3: f32,
}

impl f32__Vec3 {
    /// Atribui os ponteiros.
    /// 
    pub unsafe fn attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32, f32)> for f32__Vec3 {
    /// Cria um f32__Vec3 a partir de uma tupla.
    /// 
    fn from(other: (f32, f32, f32)) -> Self {
        Self {
            num1: other.0,
            num2: other.1,
            num3: other.2,
        }
    }
}

/// Estrutura usada para mandar informações ao Shader.
/// 
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32__Vec4 {
    pub num1: f32,
    pub num2: f32,
    pub num3: f32,
    pub num4: f32,
}

impl f32__Vec4 {
    /// Atribui os ponteiros.
    /// 
    pub unsafe fn attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::FLOAT,
            gl::FALSE,
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}

impl From<(f32, f32, f32, f32)> for f32__Vec4 {
    /// Cria um f32__Vec4 a partir de uma tupla.
    /// 
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Self {
            num1: other.0,
            num2: other.1,
            num3: other.2,
            num4: other.3,
        }
    }
}

/// Estrutura que representa uma vértice do OpenGL;
/// 
#[derive(Clone)]
pub struct Vertex {
    pub position: f32__Vec3,
    pub texcoord: f32__Vec3,
    pub color:    f32__Vec4,
    pub mode:     f32__Vec3,
}

impl Vertex {
    /// Cria uma Vértice para a renderização de formas.
    /// 
    pub fn as_shape(position: (f32, f32), color: (f32, f32, f32, f32)) -> Self {
        Self {
            position: (position.0, position.1, 0.0).into(),
            texcoord: (0.0, 0.0, 0.0).into(),
            color:    color.into(),
            mode:     (0.0, 0.0, 1.0).into(),
        }
    }

    /// Cria uma Vértice para a renderização de texturas.
    /// 
    pub fn as_texture(position: (f32, f32), texcoord: (f32, f32), color: (f32, f32, f32, f32)) -> Self {
        Self {
            position: (position.0, position.1, 0.0).into(),
            texcoord: (texcoord.0, texcoord.1, 0.0).into(),
            color:    color.into(),
            mode:     (1.0, 0.0, 0.0).into(),
        }
    }

    /// Atribui os ponteiros.
    /// 
    pub fn attrib_pointers() {
        let stride = std::mem::size_of::<Self>();

        // Position.
        let location = 0;
        let offset   = 0;

        unsafe {
            f32__Vec3::attrib_pointer(stride, location, offset);
        }

        // Texcoord.
        let location = 1;
        let offset   = offset + std::mem::size_of::<f32__Vec3>();

        unsafe {
            f32__Vec3::attrib_pointer(stride, location, offset);
        }

        // Color.
        let location = 2;
        let offset   = offset + std::mem::size_of::<f32__Vec3>();

        unsafe {
            f32__Vec4::attrib_pointer(stride, location, offset);
        }

        // Type.
        let location = 3;
        let offset   = offset + std::mem::size_of::<f32__Vec4>();

        unsafe {
            f32__Vec3::attrib_pointer(stride, location, offset);
        }
    }
}