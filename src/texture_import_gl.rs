use std::os::unix::net::UnixDatagram;
use bincode::deserialize;
use sendfd::RecvWithFd;
use crate::egl_functions::*;
use crate::glutin_example::gl;
use crate::glutin_example::gl::Gles2;
use crate::glutin_example::gl::types::GLuint;
use crate::texture_export_wgpu::{CLIENT_FILENAME, TextureStorageMetadata};
use khronos_egl::{API as egl};
use crate::wgpu_data::TEXTURE_DIMS;

pub fn dma_buf_to_texture(gl: &Gles2) -> GLuint {
    let socket = UnixDatagram::bind(CLIENT_FILENAME).expect("client file not available");
    let mut buf = vec![0; 20];
    let mut recv_fds = [0];
    let (_s, _j) = socket.recv_with_fd(buf.as_mut_slice(), &mut recv_fds).expect("recv_with_fd failed");
    let texture_storage_metadata: TextureStorageMetadata = deserialize(&buf).unwrap();

    let image_attribs = [
        EGL_LINUX_DRM_FOURCC_EXT, texture_storage_metadata.fourcc as u32,
        EGL_WIDTH, TEXTURE_DIMS.0,
        EGL_HEIGHT, TEXTURE_DIMS.1,
        EGL_DMA_BUF_PLANE0_FD_EXT, recv_fds[0] as u32,
        EGL_DMA_BUF_PLANE0_OFFSET_EXT, texture_storage_metadata.offset as u32,
        EGL_DMA_BUF_PLANE0_PITCH_EXT, texture_storage_metadata.stride as u32,
        EGL_DMA_BUF_PLANE0_MODIFIER_LO_EXT, texture_storage_metadata.modifiers as u32,
        EGL_DMA_BUF_PLANE0_MODIFIER_HI_EXT, (texture_storage_metadata.modifiers >> 32) as u32,
        EGL_NONE,
    ];

    unsafe {
        let display = egl.get_current_display().expect("display").as_ptr();
        let fn_egl_create_image_khr = egl.get_proc_address("eglCreateImageKHR");
        let egl_create_image_khr = fn_egl_create_image_khr.expect("eglCreateImageKHR not present!");
        let egl_function = std::mem::transmute_copy::<_, PFNEGLCREATEIMAGEKHRPROC>(&egl_create_image_khr);
        let egl_image_khr: EGLImageKHR = (egl_function.unwrap())(
            display,
            std::ptr::null_mut(),
            EGL_LINUX_DMA_BUF_EXT,
            std::ptr::null_mut(),
            image_attribs.as_ptr() as _,
        );
        assert!(!egl_image_khr.is_null(), "eglCreateImageKHR failed");

        let mut gl_texture = std::mem::MaybeUninit::uninit();
        gl.GenTextures(1, gl_texture.as_mut_ptr());
        assert_eq!(gl.GetError(), 0);
        gl.BindTexture(gl::TEXTURE_2D, gl_texture.assume_init());
        assert_eq!(gl.GetError(), 0);

        let fn_gl_egl_image_target_texture_2does = egl.get_proc_address("glEGLImageTargetTexture2DOES");
        let gl_egl_image_target_texture_2does = fn_gl_egl_image_target_texture_2does.expect("glEGLImageTargetTexture2DOES not present!");
        let egl_function = std::mem::transmute_copy::<_, PFNGLEGLIMAGETARGETTEXTURE2DOESPROC>(&gl_egl_image_target_texture_2does);
        (egl_function.unwrap())(gl::TEXTURE_2D, egl_image_khr);
        assert!(egl.get_error().is_none());
        assert_eq!(gl.GetError(), 0, "glEGLImageTargetTexture2DOES failed");

        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        assert_eq!(gl.GetError(), 0);

        gl.BindTexture(gl::TEXTURE_2D, 0);
        assert_eq!(gl.GetError(), 0);

        return gl_texture.assume_init();
    }
}