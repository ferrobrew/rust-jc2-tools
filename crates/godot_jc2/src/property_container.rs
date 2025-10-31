use godot::prelude::*;
use jc2_file_formats::property_container::{PropertyContainer, PropertyEntry, PropertyValue};
use jc2_hashing::HashString;

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct JcPropertyContainer {
    base: Base<RefCounted>,
    value: Option<PropertyContainer>,
}

impl JcPropertyContainer {
    pub fn new(value: PropertyContainer) -> Gd<JcPropertyContainer> {
        let mut result = JcPropertyContainer::new_gd();
        result.bind_mut().value = Some(value);
        result
    }
}

#[godot_api]
impl JcPropertyContainer {
    #[func]
    pub fn get(&mut self, key: GString) -> Variant {
        let Some(container) = &self.value else {
            return Variant::nil();
        };

        let Some(value) = container.get(hash(key)).cloned() else {
            return Variant::nil();
        };

        match value {
            PropertyEntry::Container(container) => {
                Variant::from(JcPropertyContainer::new(container))
            }
            PropertyEntry::Value(value) => match value {
                PropertyValue::Empty => Variant::nil(),
                PropertyValue::I32(value) => Variant::from(value),
                PropertyValue::F32(value) => Variant::from(value),
                PropertyValue::String(value) => Variant::from(value),
                PropertyValue::Vec2(value) => Variant::from(Vector2::from_array(value.into())),
                PropertyValue::Vec3(value) => Variant::from(Vector3::from_array(value.into())),
                PropertyValue::Vec4(value) => Variant::from(Vector4::from_array(value.into())),
                PropertyValue::Mat3x3(value) => Variant::from(Basis::from_cols(
                    Vector3::new(value[0], value[1], value[2]),
                    Vector3::new(value[3], value[4], value[5]),
                    Vector3::new(value[6], value[7], value[8]),
                )),
                PropertyValue::Mat3x4(value) => Variant::from(Transform3D::from_cols(
                    Vector3::new(value[0], value[1], value[2]),
                    Vector3::new(value[3], value[4], value[5]),
                    Vector3::new(value[6], value[7], value[8]),
                    Vector3::new(value[9], value[10], value[11]),
                )),
                PropertyValue::VecI32(value) => Variant::from(PackedInt32Array::from(value)),
                PropertyValue::VecF32(value) => Variant::from(PackedFloat32Array::from(value)),
            },
        }
    }

    #[func]
    pub fn add(&mut self, key: GString, value: Variant) {
        fn maybe<T: FromGodot, F: FnOnce(T) -> Option<PropertyEntry>>(
            value: Variant,
            f: F,
        ) -> Option<PropertyEntry> {
            value.try_to::<T>().ok().and_then(f)
        }

        fn some<T: FromGodot, F: FnOnce(T) -> PropertyEntry>(
            value: Variant,
            f: F,
        ) -> Option<PropertyEntry> {
            Some(f(value.to::<T>()))
        }

        let Some(entry) = (match value.get_type() {
            VariantType::OBJECT => maybe(value, |value: Gd<JcPropertyContainer>| {
                value
                    .bind()
                    .value
                    .as_ref()
                    .map(|container| PropertyEntry::Container(container.clone()))
            }),
            VariantType::BOOL => some(value, |value: bool| (value as i32).into()),
            VariantType::INT => some(value, |value: i32| value.into()),
            VariantType::FLOAT => some(value, |value: f32| value.into()),
            VariantType::STRING => some(value, |value: GString| value.to_string().into()),
            VariantType::VECTOR2 => some(value, |value: Vector2| value.to_array().into()),
            VariantType::VECTOR2I => some(value, |value: Vector2i| {
                [value.x as f32, value.y as f32].into()
            }),
            VariantType::VECTOR3 => some(value, |value: Vector3| value.to_array().into()),
            VariantType::VECTOR3I => some(value, |value: Vector3i| {
                [value.x as f32, value.y as f32, value.z as f32].into()
            }),
            VariantType::VECTOR4 => some(value, |value: Vector4| value.to_array().into()),
            VariantType::VECTOR4I => some(value, |value: Vector4i| {
                [
                    value.x as f32,
                    value.y as f32,
                    value.z as f32,
                    value.w as f32,
                ]
                .into()
            }),
            VariantType::QUATERNION => some(value, |value: Quaternion| {
                [value.x, value.y, value.z, value.w].into()
            }),
            VariantType::BASIS => some(value, |value: Basis| {
                [
                    value.rows[0].x,
                    value.rows[0].y,
                    value.rows[0].z,
                    value.rows[1].x,
                    value.rows[1].y,
                    value.rows[1].z,
                    value.rows[2].x,
                    value.rows[2].y,
                    value.rows[2].z,
                ]
                .into()
            }),
            VariantType::TRANSFORM3D => some(value, |value: Transform3D| {
                [
                    value.basis.rows[0].x,
                    value.basis.rows[0].y,
                    value.basis.rows[0].z,
                    value.basis.rows[1].x,
                    value.basis.rows[1].y,
                    value.basis.rows[1].z,
                    value.basis.rows[2].x,
                    value.basis.rows[2].y,
                    value.basis.rows[2].z,
                    value.origin.x,
                    value.origin.y,
                    value.origin.z,
                ]
                .into()
            }),
            VariantType::PACKED_INT32_ARRAY => {
                some(value, |value: PackedInt32Array| value.to_vec().into())
            }
            VariantType::PACKED_FLOAT32_ARRAY => {
                some(value, |value: PackedFloat32Array| value.to_vec().into())
            }
            _ => None,
        }) else {
            return;
        };

        let hash = hash(key);

        match &mut self.value {
            Some(container) => container.insert(hash, entry),
            None => self.value = Some([(hash, entry)].into()),
        }
    }

    #[func]
    pub fn clear(&mut self) {
        self.value = None
    }

    #[func]
    pub fn containers(&mut self) -> Array<Gd<JcPropertyContainer>> {
        let Some(container) = &self.value else {
            return Array::default();
        };

        Array::from_iter(
            container
                .containers()
                .cloned()
                .map(|container| JcPropertyContainer::new(container)),
        )
    }
}

fn hash(key: GString) -> HashString {
    HashString::from_bytes(key.to_utf8_buffer().as_slice())
}
