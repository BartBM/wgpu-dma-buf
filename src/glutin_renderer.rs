use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::ptr;
use glutin::display::GlDisplay;
use crate::glutin_example::gl;

pub struct Renderer {
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    gl: gl::Gl,
    texture: gl::types::GLuint
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                log::info!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                log::info!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                log::info!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl.CreateProgram();

            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut ebo = std::mem::zeroed();
            gl.GenBuffers(1, &mut ebo);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl.BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (INDICES.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                INDICES.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLsizei, ptr::null());
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, (5 * std::mem::size_of::<f32>()) as gl::types::GLsizei, (3 * std::mem::size_of::<f32>()) as *const () as *const _);

            let (texture_storage_meta_data, fd) = crate::texture_import_gl::fd_read();
            let texture = crate::texture_import_gl::dma_buf_to_texture(&gl, texture_storage_meta_data, fd);

            Self { program, vao, vbo, gl, texture }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.texture);
            self.gl.DrawElements(gl::TRIANGLES, 6,  gl::UNSIGNED_INT, ptr::null());
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub unsafe fn create_shader(
    gl: &gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
    gl.CompileShader(shader);
    shader
}

pub fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

#[rustfmt::skip]
pub static VERTEX_DATA: [f32; 20] = [
    0.5, 0.5, 0.0, 1.0, 0.0,   // top right
    0.5, -0.5, 0.0, 1.0, 1.0,  // bottom right
    -0.5, -0.5, 0.0, 0.0, 1.0, // bottom left
    -0.5, 0.5, 0.0, 0.0, 0.0   // top left
];

pub static INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

pub const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoords;
out vec2 TexCoords;

void main() {
    TexCoords = aTexCoords;
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
\0";

pub const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 330 core

out vec4 FragColor;
in vec2 TexCoords;
uniform sampler2D Texture1;

void main() {
   FragColor = texture(Texture1, TexCoords);
}
\0";
