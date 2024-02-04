use std::num::NonZeroU32;
use dma_buf::texture_export_wgpu;
use dma_buf::wgpu_context::WgpuContext;
use dma_buf::wgpu_data::TEXTURE_DIMS;
slint::include_modules!();

/// This example creates a WGPU OpenGL texture
/// and directly uses it in Slint which does not work
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    // Render to texture
    let wgpu_context = pollster::block_on(WgpuContext::create());
    wgpu_context.render_to_texture();
    let texture = texture_export_wgpu::export_to_opengl_texture(&wgpu_context.texture).expect("texture not created");

    let app = App::new().unwrap();
    let app_weak = app.as_weak();

    if let Err(error) = app.window().set_rendering_notifier(move |state, _graphics_api| {
        match state {
            slint::RenderingState::BeforeRendering => {
                if let Some(app) = app_weak.upgrade() {
                    let slint_image = unsafe {
                        slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(NonZeroU32::new(texture.0.into()).unwrap(), TEXTURE_DIMS.into()).build()
                    };
                    app.set_texture(slint::Image::from(slint_image));
                    app.window().request_redraw();
                }
            }
            slint::RenderingState::AfterRendering => {}
            _ => {}
        }
    }) {
        match error {
            slint::SetRenderingNotifierError::Unsupported => eprintln!("This example requires the use of the GL backend. Please run with the environment variable SLINT_BACKEND=GL set."),
            _ => unreachable!()
        }
        std::process::exit(1);
    }

    app.run().unwrap();
}