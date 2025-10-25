use crate::math::{Vec2, Vec3, Vec4};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GenericVertex {
    pub position: Vec3<f32>,
    pub normal: Vec3<f32>,
    pub tangent: Vec3<f32>,
    pub binormal: Vec3<f32>,

    pub morph_position: Vec3<f32>,
    pub morph_normal: Vec3<f32>,
    pub morph_tangent: Vec3<f32>,
    pub morph_binormal: Vec3<f32>,

    pub uv0: Vec2<f32>,
    pub uv1: Vec2<f32>,
    pub uv2: Vec2<f32>,
    pub uv3: Vec2<f32>,

    pub size: f32,

    pub bone_weights: [f32; 8],
    pub bone_indices: [u32; 8],

    pub diffuse_color: Vec4<f32>,
    pub specular_color: Vec4<f32>,
}
