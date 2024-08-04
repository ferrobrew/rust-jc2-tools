use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct WireframeNormalsPlugin;

impl Plugin for WireframeNormalsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WireframeNormals>()
            .register_type::<NoWireframeNormals>()
            .register_type::<WireframeNormalsConfig>()
            .register_type::<WireframeNormalColors>()
            .init_resource::<WireframeNormalsConfig>()
            .add_systems(Update, draw_wireframe_normals);
    }
}

#[derive(Component, Debug, Clone, Default, Reflect, Eq, PartialEq)]
#[reflect(Component, Default)]
pub struct WireframeNormals;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct WireframeNormalColors {
    pub normal_color: Srgba,
    pub tangent_color: Srgba,
    pub bitangent_color: Srgba,
}

impl Default for WireframeNormalColors {
    fn default() -> Self {
        Self {
            normal_color: bevy::color::palettes::basic::RED,
            tangent_color: bevy::color::palettes::basic::GREEN,
            bitangent_color: bevy::color::palettes::basic::BLUE,
        }
    }
}

#[derive(Component, Debug, Clone, Default, Reflect, Eq, PartialEq)]
#[reflect(Component, Default)]
pub struct NoWireframeNormals;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct WireframeNormalsConfig {
    pub global: bool,
    pub length: f32,
    pub default_normal_color: Srgba,
    pub default_tangent_color: Srgba,
    pub default_bitangent_color: Srgba,
}

impl Default for WireframeNormalsConfig {
    fn default() -> Self {
        Self {
            global: false,
            length: 0.02,
            default_normal_color: bevy::color::palettes::basic::RED,
            default_tangent_color: bevy::color::palettes::basic::GREEN,
            default_bitangent_color: bevy::color::palettes::basic::BLUE,
        }
    }
}

type TransformableMesh<'a> = (
    &'a Handle<Mesh>,
    &'a GlobalTransform,
    Option<&'a WireframeNormalColors>,
);

fn draw_wireframe_normals(
    mut gizmos: Gizmos,
    config: Res<WireframeNormalsConfig>,
    meshes: Res<Assets<Mesh>>,
    meshes_with_wireframe: Query<TransformableMesh, With<WireframeNormals>>,
    meshes_without_no_wireframe: Query<TransformableMesh, Without<NoWireframeNormals>>,
) {
    let draw = |(mesh, transform, colors): TransformableMesh| {
        if let Some(mesh) = meshes.get(mesh) {
            let positions = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .and_then(|x| x.as_float3());
            let normals = mesh
                .attribute(Mesh::ATTRIBUTE_NORMAL)
                .and_then(|x| x.as_float3());
            let tangents = mesh
                .attribute(Mesh::ATTRIBUTE_TANGENT)
                .and_then(|x| match x {
                    bevy::render::mesh::VertexAttributeValues::Float32x4(values) => Some(values),
                    _ => None,
                });

            if let (Some(positions), Some(normals), Some(tangents)) = (positions, normals, tangents)
            {
                let (normal_color, tangent_color, binormal_color) = match colors {
                    Some(config) => (
                        config.normal_color,
                        config.tangent_color,
                        config.bitangent_color,
                    ),
                    None => (
                        config.default_normal_color,
                        config.default_tangent_color,
                        config.default_bitangent_color,
                    ),
                };

                for i in 0..mesh.count_vertices() {
                    let (position, normal, tangent): (Vec3, Vec3, Vec4) =
                        (positions[i].into(), normals[i].into(), tangents[i].into());

                    let flipped = tangent.w;
                    let matrix = transform.compute_matrix();
                    let position = matrix.transform_point(position);

                    let normal = matrix.transform_vector3(normal);
                    gizmos.arrow(
                        position,
                        position + (normal.normalize() * config.length),
                        normal_color,
                    );

                    let tangent = matrix.transform_vector3(tangent.xyz());
                    gizmos.arrow(
                        position,
                        position + (tangent.normalize() * config.length),
                        tangent_color,
                    );

                    let binormal = matrix.transform_vector3(normal.cross(tangent) * flipped);
                    gizmos.arrow(
                        position,
                        position + (binormal.normalize() * config.length),
                        binormal_color,
                    );
                }
            }
        }
    };

    if config.global {
        meshes_without_no_wireframe.iter().for_each(draw);
    } else {
        meshes_with_wireframe.iter().for_each(draw);
    }
}
