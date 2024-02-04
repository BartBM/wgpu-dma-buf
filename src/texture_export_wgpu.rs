use std::os::fd::RawFd;
use std::os::unix::net::UnixDatagram;
use std::path::Path;
use std::fs;
use glow::NativeTexture;
use sendfd::SendWithFd;
use wgpu::{hal, Texture};
use serde::{Serialize, Deserialize};
use crate::egl_functions::*;

#[repr(C, packed)]
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub struct TextureStorageMetadata {
    pub fourcc: EGLint,
    pub offset: EGLint,
    pub modifiers: EGLuint64KHR,
    pub stride: EGLint,
}

pub fn export_to_opengl_texture(texture: &Texture) -> Option<NativeTexture> {
    let mut native_texture: Option<NativeTexture> = None;
    unsafe {
        texture.as_hal::<hal::api::Gles, _>(|hal_texture| {
            if let Some(hal_texture) = hal_texture {
                if let wgpu_hal::gles::TextureInner::Texture { raw, target: _target } = hal_texture.inner {
                    native_texture = Some(raw);
                }
            }
        });
    }
    return native_texture;
}


/// Export texture to DMA buf
/// source: Makepad (EddyB)

pub fn export_to_dma_buf(adapter: &wgpu::Adapter, native_texture: NativeTexture) -> (TextureStorageMetadata, RawFd) {
    unsafe {
        return adapter.as_hal::<hal::api::Gles, _, _>(|hal_adapter| {
            match hal_adapter {
                Some(adapter) => {
                    let raw_display = adapter.adapter_context().raw_display().unwrap().as_ptr();
                    let raw_context = adapter.adapter_context().raw_context();
                    let egl_instance = adapter.adapter_context().egl_instance().unwrap();
                    // let egl_1_5 = adapter.adapter_context().egl_instance().unwrap().upcast::<khronos_egl::EGL1_5>().expect("EGL 1.5 expected");

                    let fn_egl_create_image_khr = egl_instance.get_proc_address("eglCreateImageKHR");
                    let egl_create_image_khr = fn_egl_create_image_khr.expect("eglCreateImageKHR not present!");
                    let egl_function = std::mem::transmute_copy::<_, PFNEGLCREATEIMAGEKHRPROC>(&egl_create_image_khr);
                    let egl_image_khr: EGLImageKHR = (egl_function.unwrap())(
                        raw_display,
                        raw_context,
                        EGL_GL_TEXTURE_2D_KHR,
                        native_texture.0.get() as EGLClientBuffer,
                        std::ptr::null()
                    );
                    assert!(!egl_image_khr.is_null(), "eglCreateImageKHR failed");

                    let fn_egl_export_dmabufimage_query_mesa = egl_instance.get_proc_address("eglExportDMABUFImageQueryMESA");
                    let egl_export_dmabufimage_query_mesa = fn_egl_export_dmabufimage_query_mesa.expect("eglExportDMABUFImageQueryMESA not present!");
                    let egl_function = std::mem::transmute_copy::<_, PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC>(&egl_export_dmabufimage_query_mesa);
                    let (mut fourcc, mut num_planes, mut modifiers) = (0, 0, 0);
                    let status: u32 = (egl_function.unwrap())(
                        raw_display,
                        egl_image_khr,
                        &mut fourcc,
                        &mut num_planes,
                        &mut modifiers
                    );
                    assert_ne!(status, 0, "egl_export_dmabufimage_query_mesa failed");
                    assert_eq!(num_planes, 1, "num_planes={num_planes}");

                    let fn_egl_export_dmabufimage_mesa = egl_instance.get_proc_address("eglExportDMABUFImageMESA");
                    let egl_export_dmabufimage_mesa = fn_egl_export_dmabufimage_mesa.expect("eglExportDMABUFImageMESA not present!");
                    let egl_function = std::mem::transmute_copy::<_, PFNEGLEXPORTDMABUFIMAGEMESAPROC>(&egl_export_dmabufimage_mesa);
                    let (mut dma_buf_fd, mut offset, mut stride) = (0, 0, 0);
                    let status: u32 = (egl_function.unwrap())(
                        raw_display,
                        egl_image_khr,
                        &mut dma_buf_fd,
                        &mut stride,
                        &mut offset,
                    );
                    assert_ne!(status, 0, "egl_export_dmabufimage_mesa failed");

                    let fn_egl_destroy_image_khr = egl_instance.get_proc_address("eglDestroyImageKHR");
                    let egl_destroy_image_khr = fn_egl_destroy_image_khr.expect("eglDestroyImageKHR not present!");
                    let egl_function = std::mem::transmute_copy::<_, PFNEGLDESTROYIMAGEKHRPROC>(&egl_destroy_image_khr);
                    let status: u32 = (egl_function.unwrap())(
                        raw_display,
                        egl_image_khr
                    );
                    assert_ne!(status, 0, "egl_destroy_image_khr failed");

                    let texture_storage_metadata = TextureStorageMetadata { fourcc, modifiers, stride, offset };
                    log::info!("Sent texture to dam_buf_fd");

                    return (texture_storage_metadata, dma_buf_fd);
                },
                None => panic!("Export to DMA buf failed")
            }
        });

    }
}

pub const SERVER_FILENAME: &str = "/tmp/test_server";
pub const CLIENT_FILENAME: &str = "/tmp/test_client";

pub fn fd_write(texture_storage_metadata: TextureStorageMetadata, dma_buf_fd: RawFd) {
    let texture_storage_data = bincode::serialize(&texture_storage_metadata).expect("texture_storage_metadata to bytes failed");
    if Path::new(&SERVER_FILENAME).exists() {
        fs::remove_file(SERVER_FILENAME).expect("server file remove failed");
    }
    if Path::new(&CLIENT_FILENAME).exists() {
        fs::remove_file(CLIENT_FILENAME).expect("client file remove failed");
    }
    let socket = UnixDatagram::bind(SERVER_FILENAME).expect("server");
    log::info!("Waiting for client to connect...");
    loop {
        match socket.connect(CLIENT_FILENAME) {
            Ok(()) => { break; },
            Err(_e) => {}
        }
    }
    socket.send_with_fd(&*texture_storage_data, &[dma_buf_fd]).expect("send_with_fd");
}

