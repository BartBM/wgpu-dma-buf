use crate::glutin_example::gl::types::GLenum;

/// EGL functions
/// source: Makepad (EddyB)

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
pub const EGL_NONE: u32 = 12344;
pub const EGL_HEIGHT: u32 = 12374;
pub const EGL_WIDTH: u32 = 12375;
pub const EGL_LINUX_DMA_BUF_EXT: u32 = 12912;
pub const EGL_LINUX_DRM_FOURCC_EXT: u32 = 12913;
pub const EGL_DMA_BUF_PLANE0_FD_EXT: u32 = 12914;
pub const EGL_DMA_BUF_PLANE0_OFFSET_EXT: u32 = 12915;
pub const EGL_DMA_BUF_PLANE0_PITCH_EXT: u32 = 12916;
pub const EGL_DMA_BUF_PLANE0_MODIFIER_LO_EXT: u32 = 13379;
pub const EGL_DMA_BUF_PLANE0_MODIFIER_HI_EXT: u32 = 13380;

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

pub type PFNEGLEXPORTDMABUFIMAGEMESAPROC = Option<
    unsafe extern "C" fn(
        dpy: EGLDisplay,
        image: EGLImageKHR,
        fds: *mut i32,
        strides: *mut EGLint,
        offsets: *mut EGLint,
    ) -> EGLBoolean,
>;

pub type PFNEGLDESTROYIMAGEKHRPROC = Option<
    unsafe extern "C" fn(
        dpy: EGLDisplay,
        image: EGLImageKHR,
    ) -> EGLBoolean,
>;

pub type PFNGLEGLIMAGETARGETTEXTURE2DOESPROC = Option<
    unsafe extern "C" fn(
        GLenum,
        EGLImageKHR,
    ),
>;