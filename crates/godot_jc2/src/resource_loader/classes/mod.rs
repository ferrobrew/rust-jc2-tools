use std::collections::HashMap;

use godot::{
    classes::class_macros::private::class_macros::private::class_macros::meta::ClassId,
    global::Error,
    obj::{
        Bounds, NewAlloc,
        bounds::{DeclUser, MemManual},
    },
    prelude::*,
};

use jc2_file_formats::property_container::{PropertyContainer, PropertyEntry, PropertyValue};
use jc2_hashing::HashString;

use crate::resource_loader::JcResourceError;

use super::{JcResourceResult, JcResourceThread};

pub mod animation;
pub mod components;
pub mod game_objects;

#[macro_export]
macro_rules! jc_class_registry {
    ($($ident:ident),*$(,)?) => {
        pub fn register(registry: &mut $crate::resource_loader::classes::JcClassRegistry) {
            #[allow(unused_imports)]
            use $crate::resource_loader::classes::JcClass;

            $($ident::register(registry);)*
        }
    }
}

jc_class_registry!(animation, components, game_objects);

#[derive(Default, Debug)]
pub struct JcClassRegistry(JcNodeClasses, JcClasses);

impl JcClassRegistry {
    pub fn new() -> Self {
        let mut registry = JcClassRegistry::default();
        register(&mut registry);
        registry
    }

    pub fn import(&self, class: &str) -> Option<JcClassImportImpl> {
        self.1.get(&class.into()).map(|x| x.0)
    }

    pub fn export(&self, class: ClassId) -> Option<JcClassExportImpl> {
        self.0.get(&class).and_then(|x| self.1.get(x).map(|x| x.1))
    }
}

pub type JcNodeClasses = HashMap<ClassId, GString>;
pub type JcClasses = HashMap<GString, JcClassImpl>;
pub type JcClassImpl = (JcClassImportImpl, JcClassExportImpl);
pub type JcClassImportImpl =
    fn(&PropertyContainer, &mut JcResourceThread) -> JcResourceResult<Gd<Node3D>>;
pub type JcClassExportImpl =
    fn(Gd<Node3D>, &mut PropertyContainer, &mut JcResourceThread) -> JcResourceResult<()>;

pub trait JcClass<const NAMED: bool = true, const TRANSFORMED: bool = true>
where
    Self: GodotClass
        + Inherits<Node3D>
        + Bounds<Declarer = DeclUser>
        + Bounds<Memory = MemManual>
        + NewAlloc,
{
    const NAME: &str;

    fn register(registry: &mut JcClassRegistry) {
        let name: GString = Self::NAME.into();
        registry.0.insert(Self::class_id(), name.clone());

        let functions: JcClassImpl = (Self::import_internal, Self::export_internal);
        registry.1.insert(name, functions);
    }

    #[inline]
    #[doc(hidden)]
    fn import_internal(
        properties: &PropertyContainer,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<Gd<Node3D>> {
        let mut node = Self::new_alloc();
        let mut base = node.clone().upcast();

        if NAMED {
            base.set_name(&StringName::from(
                properties
                    .get_value::<&str>("name")
                    .or_else(|| properties.get_value("_name"))
                    .unwrap_or(Self::NAME),
            ));
        }

        // TODO: game appears to read them like this, and treats them as interchangeable...
        if TRANSFORMED {
            let mut transform = Transform3D::IDENTITY;
            if transform.read(properties, "world") {
                base.set_transform(transform);
            } else if transform.read(properties, "local") {
                base.set_transform(transform);
            }
        }

        let result = node.bind_mut().import(properties, thread);
        if let Err(err) = result {
            node.free();
            return Err(err);
        }

        Ok(base)
    }

    #[inline]
    #[doc(hidden)]
    fn export_internal(
        instance: Gd<Node3D>,
        properties: &mut PropertyContainer,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<()> {
        if NAMED {
            instance.get_name().to_gstring().write(properties, "name");
        }

        if TRANSFORMED {
            instance.get_global_transform().write(properties, "world");
            instance.get_transform().write(properties, "local");
        }

        match instance.try_cast::<Self>() {
            Ok(instance) => instance.bind().export(properties, thread),
            Err(instance) => Err(JcResourceError::Export {
                context: instance.get_path().to_gstring(),
                error: Error::ERR_INVALID_PARAMETER,
            }),
        }
    }

    fn import(
        &mut self,
        properties: &PropertyContainer,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<()>;

    fn export(
        &self,
        properties: &mut PropertyContainer,
        thread: &mut JcResourceThread,
    ) -> JcResourceResult<()>;
}

#[macro_export]
macro_rules! jc_class {
    (
        $(#[JcClass$(<$named:literal, $transformed:literal>)?($class:literal)])?
        $(#[derive($($derive:ty),+)])?
        pub struct $name:ident {
            data: $data:ident,
        }
    ) => {
        #[derive(GodotClass$(, $($derive,)+)*)]
        #[class(tool, init, base=Node3D)]
        pub struct $name {
            base: Base<Node3D>,
            data: $data,
        }

        #[godot_api]
        impl INode3D for $name {
            fn on_get_property_list(&mut self) -> Vec<godot::register::info::PropertyInfo> {
                use godot_utils::properties::PropertyStruct;
                let mut property_list = Vec::<godot::register::info::PropertyInfo>::new();
                self.data.append_property_list(&mut property_list);
                property_list
            }

            fn on_get(&self, property: StringName) -> Option<Variant> {
                use godot_utils::properties::PropertyStruct;
                self.data.get_property(property.chars())
            }

            fn on_set(&mut self, property: StringName, value: Variant) -> bool {
                use godot_utils::properties::PropertyStruct;
                let (was_set, notify) = self.data.set_property(property.chars(), &value);
                if notify {
                    self.base_mut().update_gizmos();
                }
                was_set
            }
        }

        $crate::jc_class!(@impl $name $($class)*);
    };

    (@impl $name:ident) => {};
    (@impl $name:ident $class:literal) => {
        impl crate::resource_loader::classes::JcClass for $name {
            const NAME: &str = $class;

            fn import(
                &mut self,
                properties: &jc2_file_formats::property_container::PropertyContainer,
                _thread: &mut crate::resource_loader::JcResourceThread,
            ) -> crate::resource_loader::JcResourceResult<()> {
                Ok(self.data.read(properties))
            }

            fn export(
                &self,
                properties: &mut jc2_file_formats::property_container::PropertyContainer,
                _thread: &mut crate::resource_loader::JcResourceThread,
            ) -> crate::resource_loader::JcResourceResult<()> {
                Ok(self.data.write(properties))
            }
        }
    };
}

#[macro_export]
macro_rules! jc_property_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $base_vis:vis base: $base:ty,
            $(
                $(#[$($field_meta:tt)*])*
                $field_vis:vis $field:ident$(($field_key:literal))?: $ty:ty $(= $field_default:expr)?,
            )*
        }
    ) => {
        godot_utils::property_struct! {
            $(#[$meta])*
            $vis struct $name {
                $base_vis base: $base,
                $($(#[$($field_meta)*])* $field_vis $field: $ty,)*
            }
        }

        $crate::jc_property_struct! {
            @impl $name, true, [$($field $($field_key)*,)*]
        }

        $crate::jc_property_struct! {
            @impl_default $name, true, [$($field $($field_default)*,)*]
        }
    };

    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$($field_meta:tt)*])*
                $field_vis:vis $field:ident$(($field_key:literal))?: $ty:ty $(= $field_default:expr)?,
            )*
        }
    ) => {
        godot_utils::property_struct! {
            $(#[$meta])*
            $vis struct $name { $($(#[$($field_meta)*])* $field_vis $field: $ty,)* }
        }

        $crate::jc_property_struct! {
            @impl $name, false, [$($field $($field_key)*,)*]
        }

        $crate::jc_property_struct! {
            @impl_default $name, false, [$($field $($field_default)*,)*]
        }
    };

    (@impl_default $ty:ty, true, [$($field:ident $($default:expr)?,)*]) => {
        impl Default for $ty {
            fn default() -> Self {
                Self {
                    base: Default::default(),
                    $($field: $crate::jc_property_struct!(@field_default $($default)?),)*
                }
            }
        }
    };

    (@impl_default $ty:ty, false, [$($field:ident $($default:expr)?,)*]) => {
        impl Default for $ty {
            fn default() -> Self {
                Self {
                    $($field: $crate::jc_property_struct!(@field_default $($default)?),)*
                }
            }
        }
    };

    (@field_default) => {
        Default::default()
    };
    (@field_default $default:expr) => {
        $default
    };

    (@impl $ty:ty, $has_base:tt, [$($field:ident $($key:literal)?,)*]) => {
        impl $ty {
            pub fn read(&mut self, properties: &jc2_file_formats::property_container::PropertyContainer) {
                #[allow(unused_imports)]
                use jc2_hashing::HashString;
                $($crate::jc_property_struct!(@const_decl $field $($key)*);)*
                $crate::jc_property_struct!(@base_op $has_base, read, self, properties);

                #[allow(unused_imports)]
                use jc2_file_formats::property_container::PropertyValueExtension;
                #[allow(unused_imports)]
                use crate::resource_loader::classes::GodotPropertyValueExtension;
                $(self.$field.read(properties, $field);)*
            }

            pub fn write(&self, properties: &mut jc2_file_formats::property_container::PropertyContainer) {
                #[allow(unused_imports)]
                use jc2_hashing::HashString;
                $($crate::jc_property_struct!(@const_decl $field $($key)*);)*
                $crate::jc_property_struct!(@base_op $has_base, write, self, properties);

                #[allow(unused_imports)]
                use jc2_file_formats::property_container::PropertyValueExtension;
                #[allow(unused_imports)]
                use crate::resource_loader::classes::GodotPropertyValueExtension;
                $(self.$field.write(properties, $field);)*
            }
        }
    };

    (@const_decl $field:ident) => {
        #[allow(non_upper_case_globals)]
        const $field: HashString = HashString::from_str(stringify!($field));
    };
    (@const_decl $field:ident $key:literal) => {
        #[allow(non_upper_case_globals)]
        const $field: HashString = HashString::from_str($key);
    };

    (@base_op true, $method:ident, $self:ident, $properties:ident) => {
        $self.base.$method($properties);
    };
    (@base_op false, $method:ident, $self:ident, $properties:ident) => {};
}

trait GodotPropertyValueExtension {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool;
    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>);
}

impl GodotPropertyValueExtension for Transform3D {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        let hash = hash.into();
        match properties.get(hash) {
            Some(PropertyEntry::Value(PropertyValue::Mat4x4(value))) => {
                *self = Transform3D::from_cols(
                    Vector3::new(value[0], value[1], value[2]),
                    Vector3::new(value[4], value[5], value[6]),
                    Vector3::new(value[8], value[9], value[10]),
                    Vector3::new(value[12], value[13], value[14]),
                );
                true
            }
            _ => false,
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        let [a, b, c] = &self.basis.rows;
        let o = &self.origin;

        properties.insert(
            hash,
            [
                a.x, b.x, c.x, 0.0, // Right
                a.y, b.y, c.y, 0.0, // Up
                a.z, b.z, c.z, 0.0, // Forward
                o.x, o.y, o.z, 1.0, // Translation
            ],
        );
    }
}

impl GodotPropertyValueExtension for PackedVector3Array {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        match properties.get(hash) {
            Some(PropertyEntry::Value(PropertyValue::VecF32(value))) => {
                self.clear();
                for i in 0..value.len() / 3 {
                    self.push(Vector3::new(value[i + 0], value[i + 1], value[i + 2]))
                }
                true
            }
            _ => false,
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        let mut value = vec![];
        for vector in self.as_slice() {
            value.push(vector.x);
            value.push(vector.y);
            value.push(vector.z);
        }
        properties.insert(hash, value);
    }
}

impl GodotPropertyValueExtension for Vector3 {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        match properties.get(hash) {
            Some(PropertyEntry::Value(PropertyValue::Vec3(value))) => {
                *self = Vector3::new(value.x, value.y, value.z);
                true
            }
            _ => false,
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        properties.insert(hash, [self.x, self.y, self.z])
    }
}

impl GodotPropertyValueExtension for GString {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        if let Some(property) = properties.get_value::<&str>(hash) {
            *self = GString::from(property);
            true
        } else {
            false
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        properties.insert(hash, self.to_string().as_str());
    }
}
