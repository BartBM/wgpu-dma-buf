struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) color: vec3f,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>
};

@group(0) @binding(0)
var<uniform> view_proj: mat4x4<f32>;

@vertex
//fn vs_main(@builtin(vertex_index) in_vertex_index: u32,  vert: Vertex) -> @builtin(position) vec4<f32> {
fn vs_main(@builtin(vertex_index) in_vertex_index: u32,  vert: VertexInput) -> VertexOutput {
    var output : VertexOutput;
    output.clip_position = view_proj * vec4<f32>(vert.pos[0], vert.pos[1], vert.pos[2], 1.0);
    output.color = vec4<f32>(vert.color, 1.0) ;
    return output;
}

@fragment
fn fs_main(fragData: VertexOutput) -> @location(0) vec4<f32> {
    return fragData.color;
}
//fn fs_main() -> @location(0) vec4<f32> {
//    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
//}
