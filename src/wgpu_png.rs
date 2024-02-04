use std::io::Write;
use wgpu::{CommandEncoder, Device, Queue, Texture};
use crate::wgpu_data;

/**
 * Export GPU texture to PNG image for testing
 * source: gfx-rs/wgpu example
 **/
pub async fn export_texture_image(_path: Option<String>, device: Device, queue: Queue, texture: &Texture, mut command_encoder: CommandEncoder) {
    // The texture now contains our rendered image
    let mut texture_data = Vec::<u8>::with_capacity((wgpu_data::TEXTURE_DIMS.0 * wgpu_data::TEXTURE_DIMS.1 * 4) as usize);
    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: texture_data.capacity() as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    command_encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                // This needs to be a multiple of 256. Normally we would need to pad
                // it but we here know it will work out anyways.
                bytes_per_row: Some(wgpu_data::TEXTURE_DIMS.0 * 4),
                rows_per_image: Some(wgpu_data::TEXTURE_DIMS.1 ),
            },
        },
        wgpu::Extent3d {
            width: wgpu_data::TEXTURE_DIMS.0,
            height: wgpu_data::TEXTURE_DIMS.1,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(command_encoder.finish()));
    log::info!("Commands submitted.");

    //-----------------------------------------------

    // Time to get our image.
    let buffer_slice = output_staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
    // device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    device.poll(wgpu::Maintain::Wait);
    receiver.recv_async().await.unwrap().unwrap();
    log::info!("Output buffer mapped.");
    {
        let view = buffer_slice.get_mapped_range();
        texture_data.extend_from_slice(&view[..]);
    }
    log::info!("Image data copied to local.");
    output_staging_buffer.unmap();

    export_image_png(texture_data.to_vec(), _path.unwrap());
    log::info!("Done.");
}

fn export_image_png(image_data: Vec<u8>, path: String) {
    let mut png_data = Vec::<u8>::with_capacity(image_data.len());
    let mut encoder = png::Encoder::new(
        std::io::Cursor::new(&mut png_data),
        wgpu_data::TEXTURE_DIMS.0,
        wgpu_data::TEXTURE_DIMS.1,
    );
    encoder.set_color(png::ColorType::Rgba);
    let mut png_writer = encoder.write_header().unwrap();
    png_writer.write_image_data(&image_data[..]).unwrap();
    png_writer.finish().unwrap();
    log::info!("PNG file encoded in memory.");

    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(&png_data[..]).unwrap();
    log::info!("PNG file written to disc as \"{}\".", path);
}