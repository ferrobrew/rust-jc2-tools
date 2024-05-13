#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_world, mesh_position_local_to_clip, mesh_normal_local_to_world, mesh_tangent_local_to_world}
#import bevy_pbr::pbr_types::{PbrInput, pbr_input_new}
#import bevy_pbr::pbr_functions::{prepare_world_normal, apply_normal_mapping, calculate_view, apply_pbr_lighting}
#import bevy_core_pipeline::tonemapping::tone_mapping

struct Material {
    specular_power: f32,
    uv0_scale: vec2<f32>,
    uv1_scale: vec2<f32>,
    channel_texture_mask: vec4<f32>,
    channel_ambient_occlusion_mask: vec4<f32>,
};

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv0: vec2<f32>,
    @location(2) uv1: vec2<f32>,
    @location(3) normal: vec3<f32>,
    @location(4) tangent: vec4<f32>,
    @location(7) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv0: vec2<f32>,
    @location(1) uv1: vec2<f32>,
    @location(2) world_normal: vec3<f32>,
    @location(3) world_tangent: vec4<f32>,
    @location(4) world_position: vec4<f32>,
    @location(7) instance_index: u32,
    @location(8) color: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: Material;

@group(2) @binding(1) var diffuse_texture: texture_2d<f32>;
@group(2) @binding(2) var diffuse_sampler: sampler;

@group(2) @binding(3) var normal_texture: texture_2d<f32>;
@group(2) @binding(4) var normal_sampler: sampler;

@group(2) @binding(5) var properties_texture: texture_2d<f32>;
@group(2) @binding(6) var properties_sampler: sampler;

@group(2) @binding(7) var dirt_color_texture: texture_2d<f32>;
@group(2) @binding(8) var dirt_color_sampler: sampler;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let model = get_model_matrix(vertex.instance_index);
    let position = vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(model, position);
    out.world_position = mesh_position_local_to_world(model, position);
    out.uv0 = vertex.uv0 * material.uv0_scale;
    out.uv1 = vertex.uv1 * material.uv1_scale;    
#ifdef VERTEX_NORMALS
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
#endif
#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(
        model,
        vertex.tangent,
        vertex.instance_index
    );
#endif
    out.color = vertex.color;
    out.instance_index = vertex.instance_index;
    return out;
}

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    input: VertexOutput
) -> @location(0) vec4<f32> {
    var diffuse_color = textureSample(diffuse_texture, diffuse_sampler, input.uv0);
#ifdef USE_CHANNEL_TEXTURES
    diffuse_color = vec4<f32>(vec3<f32>(dot(diffuse_color, material.channel_texture_mask)), 1.0);
#endif
    diffuse_color *= vec4<f32>(input.color.rgb, 1.0);

#ifdef ALPHA_TEST
    if diffuse_color.a < 0.5 {
        discard;
    }
#endif

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

    var N: vec3<f32> = input.world_normal;
#ifdef VERTEX_TANGENTS
#ifdef VERTEX_UVS
    var T: vec3<f32> = input.world_tangent.xyz;
    var B: vec3<f32> = cross(N, T) * input.world_tangent.w;
    var Nt: vec3<f32> = normalize(textureSampleBias(normal_texture, normal_sampler, input.uv0, view.mip_bias).rgb * 2.0 - 1.0);
#ifdef DOUBLE_SIDED
    if !is_front {
        Nt = -Nt;
    }
#endif
    N = Nt.x * T + Nt.y * B + Nt.z * N;
#endif
#endif

#ifdef USE_SNOW
    let SnowFactor: f32 = 1.0;
    let SnowColor: vec4<f32> = vec4(1.0, 1.0, 1.0, 1.0);
    let snow_mask: f32 = dirt_color.r;
    let snow_factor: f32 = saturate(5.0 * (N.y - (0.7 + (1 - snow_mask))) - SnowFactor);
    diffuse_color = mix(diffuse_color, SnowColor, snow_factor);
#endif

    var pbr_input: PbrInput = pbr_input_new();
    pbr_input.material.base_color = diffuse_color;
    pbr_input.material.metallic = saturate(specular_intensity * material.specular_power * reflection);
    pbr_input.material.reflectance = saturate(reflection);
    pbr_input.material.emissive = vec4<f32>(emissive);
    pbr_input.frag_coord = input.clip_position;
    pbr_input.world_position = input.world_position;
    pbr_input.world_normal = prepare_world_normal(
        input.world_normal,
#ifdef DOUBLE_SIDED
        true,
#else
        false,
#endif
        is_front,
    );
    pbr_input.is_orthographic = view.projection[3].w == 1.0;
    pbr_input.N = N;
    pbr_input.V = calculate_view(input.world_position, pbr_input.is_orthographic);

    return tone_mapping(apply_pbr_lighting(pbr_input), view.color_grading);
}