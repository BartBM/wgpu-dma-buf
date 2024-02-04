use std::time::Duration;
use wgpu::util::DeviceExt;
use crate::wgpu_data;
use crate::wgpu_data::ViewProj;

pub struct WgpuContext {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    view_proj_buffer: wgpu::Buffer,
    view_proj_bind_group: wgpu::BindGroup
}

impl WgpuContext {
    pub async fn create() -> WgpuContext {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
        });

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: wgpu_data::TEXTURE_DIMS.0,
                height: wgpu_data::TEXTURE_DIMS.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex_buffer"),
            contents: bytemuck::cast_slice(wgpu_data::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let view_proj_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("view_proj_buffer"),
            contents: bytemuck::cast_slice(&[ViewProj::view_proj_rotation(0.0)]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let view_proj_bind_group_layout = device.create_bind_group_layout(&ViewProj::desc());

        let view_proj_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &view_proj_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: view_proj_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render_pipeline_layout"),
            bind_group_layouts: &[&view_proj_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(render_pipeline_layout).as_ref(),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu_data::Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::TextureFormat::Rgba8Unorm.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        log::info!("Wgpu context set up");

        Self {
            adapter,
            device,
            queue,
            texture,
            texture_view,
            render_pipeline,
            vertex_buffer,
            view_proj_buffer,
            view_proj_bind_group
        }
    }

    pub fn render_to_texture(&self) {
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.view_proj_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(command_encoder.finish()));

        // Test code to export texture to png
        //use crate::wgpu_png::export_texture_image;
        //export_texture_image(Some("please_don't_git_push_me.png".to_string()), device, queue, &texture, command_encoder).await;
    }

    pub fn animate(&self) {
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        let mut rotation = 0.0;
        loop {
            self.queue.write_buffer(&self.view_proj_buffer, 0, bytemuck::cast_slice(&[ViewProj::view_proj_rotation(rotation)]));

            let mut command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.set_bind_group(0, &self.view_proj_bind_group, &[]);
                render_pass.draw(0..3, 0..1);
            }
            self.queue.submit(Some(command_encoder.finish()));
            rotation = rotation + 10.0;
            std::thread::sleep(Duration::from_secs(1));
        }
    }
}