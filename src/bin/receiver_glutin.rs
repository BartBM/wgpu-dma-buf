use std::error::Error;
use winit::event_loop::EventLoopBuilder;
use dma_buf::glutin_example;


fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    glutin_example::main(EventLoopBuilder::new().build().unwrap())
}
