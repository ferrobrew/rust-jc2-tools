use godot::{
    classes::{
        ArrayMesh, Material,
        mesh::{ArrayFormat, ArrayType as MeshArrayType, PrimitiveType},
    },
    obj::IndexEnum,
    prelude::*,
};

pub struct MeshBuilder {
    mesh: Gd<ArrayMesh>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            mesh: ArrayMesh::new_gd(),
        }
    }

    pub fn surface(self, f: impl FnOnce(MeshSurfaceBuilder) -> MeshSurfaceBuilder) -> Self {
        Self {
            mesh: f(MeshSurfaceBuilder::new(self.mesh)).build(),
            ..self
        }
    }

    pub fn build(self) -> Gd<ArrayMesh> {
        self.mesh
    }
}

pub struct MeshSurfaceBuilder {
    mesh: Gd<ArrayMesh>,
    primitive_type: PrimitiveType,
    material: Option<Gd<Material>>,
    arrays: [Variant; MeshArrayType::ENUMERATOR_COUNT],
    blend_shapes: Vec<VariantArray>,
}

impl MeshSurfaceBuilder {
    pub fn new(mesh: Gd<ArrayMesh>) -> Self {
        Self {
            mesh,
            primitive_type: PrimitiveType::TRIANGLES,
            material: None,
            arrays: std::array::from_fn(|_| Variant::nil()),
            blend_shapes: vec![],
        }
    }

    #[inline]
    fn set(mut self, array: MeshArrayType, values: Variant) -> Self {
        self.arrays[array.ord() as usize] = values;
        self
    }

    #[inline]
    fn get(&self, array: MeshArrayType) -> &Variant {
        &self.arrays[array.ord() as usize]
    }

    pub fn primitive_type(mut self, primitive_type: PrimitiveType) -> Self {
        self.primitive_type = primitive_type;
        self
    }

    pub fn material<T: GodotClass + Inherits<Material>>(mut self, material: Gd<T>) -> Self {
        self.material = Some(material.upcast());
        self
    }

    pub fn vertices<T: MeshArray + MeshVertexArray>(self, values: T) -> Self {
        self.set(MeshArrayType::VERTEX, values.to_array())
    }

    pub fn normals<T: MeshArray + MeshNormalArray>(self, values: T) -> Self {
        self.set(MeshArrayType::NORMAL, values.to_array())
    }

    pub fn tangents<T: MeshArray + MeshTangentArray>(self, values: T) -> Self {
        self.set(MeshArrayType::TANGENT, values.to_array())
    }

    pub fn colors<T: MeshArray + MeshColorArray>(self, values: T) -> Self {
        self.set(MeshArrayType::COLOR, values.to_array())
    }

    pub fn uv1<T: MeshArray + MeshTexCoordArray>(self, values: T) -> Self {
        self.set(MeshArrayType::TEX_UV, values.to_array())
    }

    pub fn uv2<T: MeshArray + MeshTexCoordArray>(self, values: T) -> Self {
        self.set(MeshArrayType::TEX_UV2, values.to_array())
    }

    pub fn custom0<T: MeshArray + MeshCustomArray>(self, values: T) -> Self {
        self.set(MeshArrayType::CUSTOM0, values.to_array())
    }

    pub fn custom1<T: MeshArray + MeshCustomArray>(self, values: T) -> Self {
        self.set(MeshArrayType::CUSTOM1, values.to_array())
    }

    pub fn custom2<T: MeshArray + MeshCustomArray>(self, values: T) -> Self {
        self.set(MeshArrayType::CUSTOM2, values.to_array())
    }

    pub fn custom3<T: MeshArray + MeshCustomArray>(self, values: T) -> Self {
        self.set(MeshArrayType::CUSTOM3, values.to_array())
    }

    pub fn bones<T: MeshArray + MeshCustomArray>(self, values: T) -> Self {
        self.set(MeshArrayType::BONES, values.to_array())
    }

    pub fn weights<T: MeshArray + MeshWeightArray>(self, values: T) -> Self {
        self.set(MeshArrayType::WEIGHTS, values.to_array())
    }

    pub fn indices<T: MeshArray + MeshIndexArray>(self, values: T) -> Self {
        self.set(MeshArrayType::INDEX, values.to_array())
    }

    pub fn blend_shape(
        self,
        f: impl FnOnce(MeshBlendShapeBuilder) -> MeshBlendShapeBuilder,
    ) -> Self {
        Self {
            blend_shapes: f(MeshBlendShapeBuilder::new(self.blend_shapes)).build(),
            ..self
        }
    }

    pub fn build(mut self) -> Gd<ArrayMesh> {
        let blend_shape_count = self.blend_shapes.len();
        let mesh_blend_shape_count = self.mesh.get_blend_shape_count() as usize;
        for idx in mesh_blend_shape_count..blend_shape_count {
            self.mesh.add_blend_shape(&format!("blend_shape_{idx}"));
        }

        let mut flags = ArrayFormat::default();

        if !self.get(MeshArrayType::VERTEX).is_nil() {
            flags |= ArrayFormat::VERTEX;
        }

        if !self.get(MeshArrayType::NORMAL).is_nil() {
            flags |= ArrayFormat::NORMAL;
        }

        if !self.get(MeshArrayType::TANGENT).is_nil() {
            flags |= ArrayFormat::TANGENT;
        }

        if !self.get(MeshArrayType::COLOR).is_nil() {
            flags |= ArrayFormat::COLOR;
        }

        if !self.get(MeshArrayType::TEX_UV).is_nil() {
            flags |= ArrayFormat::TEX_UV;
        }

        if !self.get(MeshArrayType::TEX_UV2).is_nil() {
            flags |= ArrayFormat::TEX_UV2;
        }

        if !self.get(MeshArrayType::BONES).is_nil() {
            flags |= ArrayFormat::BONES;
        }

        if !self.get(MeshArrayType::WEIGHTS).is_nil() {
            flags |= ArrayFormat::WEIGHTS;
        }

        if !self.get(MeshArrayType::INDEX).is_nil() {
            flags |= ArrayFormat::INDEX;
        }

        self.mesh
            .add_surface_from_arrays_ex(self.primitive_type, &self.arrays.to_godot())
            .blend_shapes(&self.blend_shapes.to_godot())
            .flags(flags)
            .done();

        if let Some(material) = self.material {
            let surface_index = self.mesh.get_surface_count() - 1;
            self.mesh.surface_set_material(surface_index, &material);
        }

        self.mesh
    }
}

pub struct MeshBlendShapeBuilder {
    blend_shapes: Vec<VariantArray>,
    arrays: [Variant; MeshArrayType::ENUMERATOR_COUNT],
}

impl MeshBlendShapeBuilder {
    pub fn new(blend_shapes: Vec<VariantArray>) -> Self {
        Self {
            blend_shapes,
            arrays: std::array::from_fn(|_| Variant::nil()),
        }
    }

    #[inline]
    fn set(mut self, array: MeshArrayType, values: Variant) -> Self {
        self.arrays[array.ord() as usize] = values;
        self
    }

    pub fn vertices<T: MeshArray + MeshVertexArray>(self, values: T) -> Self {
        self.set(MeshArrayType::VERTEX, values.to_array())
    }

    pub fn normals<T: MeshArray + MeshNormalArray>(self, values: T) -> Self {
        self.set(MeshArrayType::NORMAL, values.to_array())
    }

    pub fn tangents<T: MeshArray + MeshTangentArray>(self, values: T) -> Self {
        self.set(MeshArrayType::TANGENT, values.to_array())
    }

    pub fn build(self) -> Vec<VariantArray> {
        let mut blend_shapes = self.blend_shapes;
        blend_shapes.push(self.arrays.to_godot());
        blend_shapes
    }
}

pub trait MeshArray {
    fn to_array(self) -> Variant;
}

impl MeshArray for PackedColorArray {
    fn to_array(self) -> Variant {
        Variant::from(self)
    }
}

impl MeshArray for PackedFloat32Array {
    fn to_array(self) -> Variant {
        Variant::from(self)
    }
}

impl MeshArray for PackedFloat64Array {
    fn to_array(self) -> Variant {
        Variant::from(self)
    }
}

impl MeshArray for PackedInt32Array {
    fn to_array(self) -> Variant {
        Variant::from(self)
    }
}

impl MeshArray for PackedVector2Array {
    fn to_array(self) -> Variant {
        Variant::from(self)
    }
}

impl MeshArray for PackedVector3Array {
    fn to_array(self) -> Variant {
        Variant::from(self)
    }
}

pub trait MeshVertexArray {}

impl MeshVertexArray for PackedVector2Array {}
impl MeshVertexArray for PackedVector3Array {}

pub trait MeshNormalArray {}

impl MeshNormalArray for PackedVector3Array {}

pub trait MeshTangentArray {}

impl MeshTangentArray for PackedFloat32Array {}
impl MeshTangentArray for PackedFloat64Array {}

pub trait MeshColorArray {}

impl MeshColorArray for PackedColorArray {}

pub trait MeshTexCoordArray {}

impl MeshTexCoordArray for PackedVector2Array {}
impl MeshTexCoordArray for PackedVector3Array {}

pub trait MeshCustomArray {}

impl MeshCustomArray for PackedByteArray {}
impl MeshCustomArray for PackedFloat32Array {}

pub trait MeshBoneArray {}

impl MeshBoneArray for PackedFloat32Array {}
impl MeshBoneArray for PackedInt32Array {}

pub trait MeshWeightArray {}

impl MeshWeightArray for PackedFloat32Array {}
impl MeshWeightArray for PackedFloat64Array {}

pub trait MeshIndexArray {}

impl MeshIndexArray for PackedInt32Array {}
