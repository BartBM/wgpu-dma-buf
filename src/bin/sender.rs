use dma_buf::texture_export_wgpu;
use dma_buf::wgpu_context::WgpuContext;

pub fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_timestamp_nanos()
        .init();

    // Render to texture
    let wgpu_context = pollster::block_on(WgpuContext::create());
    wgpu_context.render_to_texture();
    // Export the texture to DMA-BUF
    let texture = texture_export_wgpu::export_to_opengl_texture(&wgpu_context.texture);
    let (texture_storage_meta_data, dma_buf_fd) = texture_export_wgpu::export_to_dma_buf(&wgpu_context.adapter, texture.unwrap());
    texture_export_wgpu::fd_write(texture_storage_meta_data, dma_buf_fd);
    // Animate the texture by changing view_proj buffer
    wgpu_context.animate();
}