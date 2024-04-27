#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_world, mesh_position_local_to_clip, mesh_normal_local_to_world}

@group(2) @binding(1) var diffuse_texture: texture_2d<f32>;
@group(2) @binding(2) var diffuse_sampler: sampler;

@group(2) @binding(3) var normal_texture: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;

@group(2) @binding(5) var properties_texture: texture_2d<f32>;
@group(2) @binding(6) var properties_sampler: sampler;

@group(2) @binding(7) var dirt_color_texture: texture_2d<f32>;
@group(2) @binding(8) var dirt_color_sampler: sampler;


struct Material {
    scale: f32,
    specular_power: f32,
    uv0_scale: vec2<f32>,
    uv1_scale: vec2<f32>,
    channel_texture_mask: vec4<f32>,
    channel_ambient_occlusion_mask: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: Material;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) uv1: vec2<f32>,
    @location(3) normal: vec3<f32>,
    @location(4) tangent: vec3<f32>,
    @location(5) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) uv1: vec2<f32>,
    @location(3) world_normal: vec3<f32>,
    @location(4) world_tangent: vec3<f32>,
    @location(5) color: vec4<f32>,
    @location(6) @interpolate(flat) instance_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let model = get_model_matrix(vertex.instance_index);
    let position = vec4<f32>(vertex.position * material.scale, 1.0);

    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(model, position);
    out.world_position = mesh_position_local_to_world(model, position);
    out.uv0 = vertex.uv0 * material.uv0_scale;
    out.uv1 = vertex.uv1 * material.uv1_scale;
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.world_tangent = mesh_normal_local_to_world(vertex.tangent, vertex.instance_index);
    out.color = vertex.color;
    out.instance_index = vertex.instance_index;
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var diffuse_color = textureSample(diffuse_texture, diffuse_sampler, input.uv0);
#ifdef USE_CHANNEL_TEXTURES
    diffuse_color = vec4<f32>(dot(diffuse_color, material.channel_texture_mask).xxx, 1.0);
#endif
    diffuse_color *= vec4<f32>(input.color.rgb, 1.0);

    let dirt_color = textureSample(dirt_color_texture, dirt_color_sampler, input.uv1);
    diffuse_color *= mix(
        vec4<f32>(1.0, 1.0, 1.0, 1.0),
        vec4<f32>(dirt_color.rgb, 1.0),
        material.channel_ambient_occlusion_mask.r
    );

    let properties = textureSample(properties_texture, properties_sampler, input.uv0);
    let reflection = properties.r;
    let specular_intensity = properties.g;
    let emissive = properties.b;

    return input.color * diffuse_color;
}