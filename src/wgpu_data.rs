use cgmath::SquareMatrix;

pub const TEXTURE_DIMS: (u32, u32) = (256, 256);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub(crate) const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.5, 0.5, 0.5],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.5, 0.5, 0.0],
    }
];

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewProj {
    data: [[f32; 4]; 4],
}

impl ViewProj {
    pub fn desc() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("view_proj_bind_group_layout"),
        }
    }

    pub fn view_proj_rotation(r: f32) -> ViewProj {
        let mut mat = cgmath::Matrix4::identity();
        mat.x.x =  f32::cos(r);
        mat.x.y =  f32::sin(r);
        mat.y.x = -f32::sin(r);
        mat.y.y =  f32::cos(r);

        ViewProj {
            data: mat.into()
        }
    }
}
