use gl;
use std::{self, ffi::CStr, ffi::CString};

/// Estrutura que armazena e manipula valores relacionados aos Shaders.
/// 
pub struct Shader {
    pub vert_id: gl::types::GLuint,
    pub frag_id: gl::types::GLuint,
    pub prog_id: gl::types::GLuint,

    pub is_empty: bool,
}

impl Shader {
    /// Cria um shader vazio.
    /// 
    pub fn empty() -> Self {
        Self {
            vert_id: 0,
            frag_id: 0,
            prog_id: 0,

            is_empty: true,
        }
    }

    /// Cria um novo shader usando os códigos fornecidos pela função.
    /// 
    pub fn new(vert_code: &CStr, frag_code: &CStr) -> Result<Shader, String> {
        let vert_id = Shader::from_vertex_source(vert_code)?;
        let frag_id = Shader::from_fragment_source(frag_code)?;

        let prog_id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(prog_id, vert_id);
            gl::AttachShader(prog_id, frag_id);

            gl::LinkProgram(prog_id);
        }

        // Retorna o estado de compilação do Shader.
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(prog_id, gl::LINK_STATUS, &mut success);
        }

        // Gerenciamento de erros.
        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(prog_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            // Escreve a mensagem de erro.
            unsafe {
                gl::GetProgramInfoLog(
                    prog_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        unsafe {
            gl::DetachShader(prog_id, vert_id);
            gl::DetachShader(prog_id, frag_id);
        }

        Ok(Shader { 
            vert_id,
            frag_id,
            prog_id,

            is_empty: false,
        })
    }

    /// Usa este shader.
    /// 
    pub fn use_program(&mut self) {
        unsafe {
            gl::UseProgram(self.prog_id);
        }
    }

    /// Cria um novo Vertex Shader.
    /// 
    pub fn from_vertex_source(source: &CStr) -> Result<gl::types::GLuint, String> {
        shader_from_source(source, gl::VERTEX_SHADER)
    }

    /// Cria um novo Fragment Shader.
    /// 
    pub fn from_fragment_source(source: &CStr) -> Result<gl::types::GLuint, String> {
        shader_from_source(source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    /// Deleta o Shader quando removido do programa.
    /// 
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.vert_id);
            gl::DeleteShader(self.frag_id);
            gl::DeleteProgram(self.prog_id);
        }
    }
}

/// Cria umm novo Vertex ou Fragment Shader.
/// 
pub fn shader_from_source(
    source: &CStr, 
    kind: gl::types::GLenum
) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    // Define a fonte do shader e o compila.
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    // Retorna o estado de compilação do shader.
    let mut sucess: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut sucess);
    }

    // Retorna um erro.
    if sucess == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // Converte o buffer á uma CString.
        let error = create_whitespace_cstring_with_len(len as usize);

        // Escreve o Log de erro do shader.
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }

        // Finalmente, retorna a mensagem de erro.
        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

/// Uma função que cria um string vazia com o tamanho fornecido.
/// 
fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Aloca um buffer do tamanho correto.
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

    // Enche a string.
    buffer.extend([b' '].iter().cycle().take(len as usize));

    // Converte o Buffer em uma CString.
    unsafe { CString::from_vec_unchecked(buffer) }
}