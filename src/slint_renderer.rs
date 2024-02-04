use std::num::NonZeroU32;
use crate::glutin_example::gl;
use crate::glutin_example::gl::Gles2;
use crate::glutin_example::gl::types::GLuint;
use crate::glutin_renderer::get_gl_string;
use crate::texture_import_gl::dma_buf_to_texture;
use crate::wgpu_data::TEXTURE_DIMS;

pub struct SlintRenderer {
    pub gl: Gles2,
    texture_id: GLuint
}

impl SlintRenderer {
    pub fn new(gl: Gles2) -> Self {
        if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
            log::info!("Running on {}", renderer.to_string_lossy());
        }
        if let Some(version) = get_gl_string(&gl, gl::VERSION) {
            log::info!("OpenGL Version {}", version.to_string_lossy());
        }

        if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
            log::info!("Shaders version on {}", shaders_version.to_string_lossy());
        }
        let texture_id = dma_buf_to_texture(&gl);
        Self { gl, texture_id }
    }

    pub fn render(&self) -> slint::Image {
        let result_texture = unsafe {
            slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(NonZeroU32::new(self.texture_id).unwrap(), TEXTURE_DIMS.into()).build()
        };
        result_texture
    }
}
