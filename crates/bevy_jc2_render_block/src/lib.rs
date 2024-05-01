use std::path::PathBuf;

use bevy::{
    asset::{embedded_asset, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        renderer::RenderDevice,
        texture::{CompressedImageFormats, ImageLoaderSettings, ImageSampler},
    },
    utils::HashSet,
};
use thiserror::Error;

use jc2_file_formats::render_block_model as rbm;

use self::materials::{general::RenderBlockGeneralMaterial, RenderBlockMaterial};

pub mod materials;

#[derive(Error, Debug)]
pub enum RenderBlockModelError {
    #[error("unsupported render block")]
    UnsupportedRenderBlock { block: rbm::RenderBlock },
    #[error("unsupported primitive type")]
    UnsupportedPrimitive { primitive: rbm::PrimitiveType },
    #[error("invalid rbm file: {0}")]
    Binrw(#[from] binrw::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Asset, Debug, Clone, TypePath)]
pub struct RenderBlockPrimitive {
    pub mesh: Handle<Mesh>,
    pub material: RenderBlockMaterial,
}

#[derive(Asset, Debug, Clone, TypePath)]
pub struct RenderBlockMesh {
    pub primitives: Vec<RenderBlockPrimitive>,
}

#[derive(Bundle, Default)]
pub struct RenderBlockBundle {
    pub mesh: Handle<RenderBlockMesh>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Default)]
struct RenderBlockLoader {
    #[allow(dead_code)]
    pub supported_compressed_formats: CompressedImageFormats,
}

#[inline]
fn get_primitive_topology(
    primitive: rbm::PrimitiveType,
) -> Result<PrimitiveTopology, RenderBlockModelError> {
    use rbm::PrimitiveType::*;
    match primitive {
        TriangleList | IndexedTriangleList => Ok(PrimitiveTopology::TriangleList),
        TriangleStrip | IndexedTriangleStrip => Ok(PrimitiveTopology::TriangleStrip),
        PointSprite | IndexedPointSprite => Ok(PrimitiveTopology::PointList),
        LineList => Ok(PrimitiveTopology::LineList),
        TriangleFan | IndexedTriangleFan => {
            Err(RenderBlockModelError::UnsupportedPrimitive { primitive })
        }
    }
}

impl AssetLoader for RenderBlockLoader {
    type Asset = RenderBlockMesh;
    type Settings = ();
    type Error = RenderBlockModelError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            let model = rbm::RenderBlockModel::read(&mut binrw::io::Cursor::new(&bytes))?;

            let mut primitives = Vec::with_capacity(model.blocks.len());

            for block in model.blocks.iter() {
                match block {
                    rbm::RenderBlock::General(general) => {
                        let mut mesh = Mesh::new(
                            get_primitive_topology(general.material.primitive_type)?,
                            RenderAssetUsages::default(),
                        );

                        macro_rules! vec_attr {
                            ($mesh:ident, $attribute:expr, $vec:ty, $block:expr, $field:ident) => {
                                $mesh.insert_attribute(
                                    $attribute,
                                    $block
                                        .vertices
                                        .iter()
                                        .map(|vertex| <$vec>::from_array(vertex.$field.into()))
                                        .collect::<Vec<$vec>>(),
                                )
                            };
                        }

                        mesh.insert_indices(Indices::U16(general.indices.to_vec()));

                        vec_attr!(mesh, Mesh::ATTRIBUTE_POSITION, Vec3, general, position);
                        vec_attr!(mesh, Mesh::ATTRIBUTE_UV_0, Vec2, general, uv0);
                        vec_attr!(mesh, Mesh::ATTRIBUTE_UV_1, Vec2, general, uv1);
                        vec_attr!(mesh, Mesh::ATTRIBUTE_NORMAL, Vec3, general, normal);
                        vec_attr!(mesh, Mesh::ATTRIBUTE_TANGENT, Vec4, general, tangent);
                        vec_attr!(mesh, Mesh::ATTRIBUTE_COLOR, Vec4, general, color);

                        fn load_image(
                            load_context: &mut LoadContext,
                            path: impl Into<PathBuf>,
                            is_srgb: bool,
                        ) -> Handle<Image> {
                            load_context.load_with_settings(
                                path.into(),
                                move |settings: &mut ImageLoaderSettings| {
                                    settings.is_srgb = is_srgb;
                                    settings.sampler = ImageSampler::Descriptor(
                                        SamplerDescriptor {
                                            address_mode_u: AddressMode::Repeat,
                                            address_mode_v: AddressMode::Repeat,
                                            address_mode_w: AddressMode::Repeat,
                                            mag_filter: FilterMode::Linear,
                                            min_filter: FilterMode::Linear,
                                            mipmap_filter: FilterMode::Linear,
                                            anisotropy_clamp: 16,
                                            lod_min_clamp: 0.0,
                                            lod_max_clamp: 0.0,
                                            ..default()
                                        }
                                        .into(),
                                    );
                                },
                            )
                        }

                        let parent = if let Some(parent) = load_context.path().parent() {
                            parent.to_path_buf()
                        } else {
                            load_context.path().into()
                        };
                        let textures = &general.material.textures;

                        let mut material = RenderBlockGeneralMaterial::from(&general.attributes);
                        material.diffuse_texture = Some(load_image(
                            load_context,
                            parent.join(textures[0].as_ref()),
                            true,
                        ));
                        material.normal_texture = Some(load_image(
                            load_context,
                            parent.join(textures[1].as_ref()),
                            false,
                        ));
                        material.properties_texture = Some(load_image(
                            load_context,
                            parent.join(textures[2].as_ref()),
                            false,
                        ));

                        let mesh = load_context.add_labeled_asset("Mesh".into(), mesh);
                        let material = load_context
                            .add_labeled_asset("Material".into(), material)
                            .into();

                        primitives.push(RenderBlockPrimitive { mesh, material });
                    }
                    _ => {
                        return Err(RenderBlockModelError::UnsupportedRenderBlock {
                            block: block.clone(),
                        })
                    }
                }
            }

            Ok(RenderBlockMesh { primitives })
        })
    }
}

#[derive(Default)]
pub struct RenderBlockPlugin;

impl Plugin for RenderBlockPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "assets/shaders/general_material.wgsl");

        app.init_asset::<RenderBlockPrimitive>()
            .init_asset::<RenderBlockMesh>()
            .add_plugins(MaterialPlugin::<RenderBlockGeneralMaterial>::default())
            .add_systems(
                PreUpdate,
                (
                    load_mesh,
                    reload_mesh.after(load_mesh),
                    general_material_changed.after(reload_mesh),
                    general_material_transform_changed.after(general_material_changed),
                ),
            )
            .preregister_asset_loader::<RenderBlockLoader>(&["rbm"]);
    }

    fn finish(&self, app: &mut App) {
        let supported_compressed_formats = match app.world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),
            None => CompressedImageFormats::NONE,
        };
        app.register_asset_loader(RenderBlockLoader {
            supported_compressed_formats,
        });
    }
}

fn apply_mesh(commands: &mut Commands, entity: Entity, mesh: &RenderBlockMesh) {
    let children: Vec<Entity> = mesh
        .primitives
        .iter()
        .map(|primitive| {
            commands
                .spawn(MaterialMeshBundle {
                    mesh: primitive.mesh.clone(),
                    material: match primitive.material.clone() {
                        RenderBlockMaterial::General(material) => material,
                    },
                    ..default()
                })
                .id()
        })
        .collect();
    commands.entity(entity).push_children(&children);
}

fn load_mesh(
    mut commands: Commands,
    assets: Res<Assets<RenderBlockMesh>>,
    query: Query<(Entity, &Handle<RenderBlockMesh>), Changed<Handle<RenderBlockMesh>>>,
) {
    for (entity, handle) in &query {
        commands.entity(entity).despawn_descendants();
        if let Some(mesh) = assets.get(handle) {
            apply_mesh(&mut commands, entity, mesh);
        }
    }
}

fn reload_mesh(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<RenderBlockMesh>>,
    assets: Res<Assets<RenderBlockMesh>>,
    query: Query<(Entity, &Handle<RenderBlockMesh>)>,
) {
    let mut loaded = HashSet::with_capacity(events.len());
    for event in events.read() {
        match event {
            AssetEvent::Added { id }
            | AssetEvent::Modified { id }
            | AssetEvent::LoadedWithDependencies { id } => {
                loaded.insert_unique_unchecked(*id);
            }
            _ => {}
        }
    }
    for (entity, handle) in query.iter() {
        if loaded.contains(&handle.id()) {
            commands.entity(entity).despawn_descendants();
            if let Some(mesh) = assets.get(handle) {
                apply_mesh(&mut commands, entity, mesh);
            }
        }
    }
}

fn general_material_changed(
    materials: Res<Assets<RenderBlockGeneralMaterial>>,
    mut materials_changed: Query<
        (&Handle<RenderBlockGeneralMaterial>, &mut Transform),
        Changed<Handle<RenderBlockGeneralMaterial>>,
    >,
) {
    for (handle, mut transform) in &mut materials_changed {
        if let Some(material) = materials.get(handle) {
            transform.scale = Vec3::splat(material.scale);
        }
    }
}

fn general_material_transform_changed(
    mut materials: ResMut<Assets<RenderBlockGeneralMaterial>>,
    mut transforms_changed: Query<
        (&Handle<RenderBlockGeneralMaterial>, &Transform),
        Changed<Transform>,
    >,
) {
    for (handle, transform) in &mut transforms_changed {
        if let Some(material) = materials.get_mut(handle) {
            material.scale = transform.scale.max_element();
        }
    }
}
