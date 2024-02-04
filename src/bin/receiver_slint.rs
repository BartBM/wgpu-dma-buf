use std::ffi::CString;
use dma_buf::glutin_example::gl;
use dma_buf::slint_renderer::SlintRenderer;
slint::include_modules!();

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();


    let mut slint_renderer: Option<SlintRenderer> = None;

    let app = App::new().unwrap();
    let app_weak = app.as_weak();

    if let Err(error) = app.window().set_rendering_notifier(move |state, graphics_api| {
        match state {
            slint::RenderingState::RenderingSetup => {
                let context = match graphics_api {
                    slint::GraphicsAPI::NativeOpenGL { get_proc_address } => {
                        let gl = gl::Gl::load_with(|symbol| {
                            let symbol = CString::new(symbol).unwrap();
                            get_proc_address(symbol.as_c_str()).cast()
                        });
                        gl
                    },
                    _ => return,
                };

                slint_renderer = Some(SlintRenderer::new(context));
            }
            slint::RenderingState::BeforeRendering => {
                if let (Some(slint_renderer), Some(app)) = (slint_renderer.as_mut(), app_weak.upgrade()) {
                    let texture = slint_renderer.render();
                    app.set_texture(slint::Image::from(texture));
                    app.window().request_redraw();
                }
            }
            slint::RenderingState::AfterRendering => {}
            slint::RenderingState::RenderingTeardown => {
                //drop(underlay.take());
            }
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