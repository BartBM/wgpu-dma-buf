use std::ffi::{c_char, CString};
use glow::NativeTexture;
use wgpu::{hal, Texture};
use wgpu_hal::Adapter;

pub type EGLint = i32;
pub type EGLuint64KHR = u64;
pub type EGLenum = ::std::os::raw::c_uint;
pub type EGLBoolean = ::std::os::raw::c_uint;
pub type EGLDisplay = *mut ::std::os::raw::c_void;
pub type EGLConfig = *mut ::std::os::raw::c_void;
pub type EGLSurface = *mut ::std::os::raw::c_void;
pub type EGLContext = *mut ::std::os::raw::c_void;
pub type EGLClientBuffer = *mut ::std::os::raw::c_void;
pub type EGLImageKHR = *mut ::std::os::raw::c_void;
pub const EGL_GL_TEXTURE_2D_KHR: u32 = 12465;

// extern "C" {
//     pub fn eglGetProcAddress(procname: *const c_char) -> extern "C" fn();
// }
//
// pub fn get_proc_address(procname: &str) -> extern "C" fn() {
//     unsafe {
//         let string = CString::new(procname).unwrap();
//
//         eglGetProcAddress(string.as_ptr())
//     }
// }
//
// pub type PFNEGLGETPROCADDRESSPROC = Option<
//     unsafe extern "C" fn(
//         procname: *const ::std::os::raw::c_char,
//     ) -> *mut ::std::os::raw::c_void,
// >;

pub type PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC = Option<
    unsafe extern "C" fn(
        dpy: EGLDisplay,
        image: EGLImageKHR,
        fourcc: *mut i32,
        num_planes: *mut i32,
        modifiers: *mut u64,
    ) -> EGLBoolean,
>;

pub type PFNEGLCREATEIMAGEKHRPROC = Option<
    unsafe extern "C" fn(
        dpy: EGLDisplay,
        ctx: EGLContext,
        target: EGLenum,
        buffer: EGLClientBuffer,
        attrib_list: *const EGLint,
    ) -> EGLImageKHR,
>;


pub fn texture_to_dma_buf(adapter: wgpu::Adapter, texture: &Texture) {
    unsafe {
        let mut native_texture: Option<NativeTexture> = None;
        texture.as_hal::<hal::api::Gles, _>(|hal_texture| {
            if let Some(hal_texture) = hal_texture {
                if let wgpu_hal::gles::TextureInner::Texture{raw, target } = hal_texture.inner {
                    native_texture = Some(raw);
                }
            }
        });

        adapter.as_hal::<hal::api::Gles, _, _>(|hal_adapter| {
            if let Some(adapter) = hal_adapter {
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
                    native_texture.unwrap().0.get() as EGLClientBuffer,
                    std::ptr::null()
                );
                assert!(!egl_image_khr.is_null(), "eglCreateImageKHR failed");

                let fn_egl_export_dmabufimage_query_mesa = egl_instance.get_proc_address("eglExportDMABUFImageQueryMESA");
                let egl_export_dmabufimage_query_mesa = fn_egl_export_dmabufimage_query_mesa.expect("eglExportDMABUFImageQueryMESA not present!");
                let egl_function = std::mem::transmute_copy::<_, PFNEGLEXPORTDMABUFIMAGEQUERYMESAPROC>(&egl_export_dmabufimage_query_mesa);
                let (mut fourcc, mut num_planes) = (0, 0);
                let status: u32 = (egl_function.unwrap())(
                    raw_display,
                    egl_image_khr,
                    &mut fourcc,
                    &mut num_planes,
                    std::ptr::null_mut()
                );
                assert_ne!(status, 0, "egl_export_dmabufimage_query_mesa failed"); // FAILS
                assert_eq!(num_planes, 1, "num_planes={num_planes}");
            }
        });
    }
}