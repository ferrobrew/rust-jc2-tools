use godot::prelude::*;

pub const fn hash_chars(data: &[char]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    let mut i = 0;

    while i < data.len() {
        hash ^= data[i] as u64;
        hash = hash.wrapping_mul(0x00000100000001B3);
        i += 1;
    }

    hash
}

pub const fn hash_str(data: &str) -> u64 {
    let bytes = data.as_bytes();
    let mut hash = 0xcbf29ce484222325_u64;
    let mut i = 0;

    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(0x00000100000001B3);
        i += 1;
    }

    hash
}

pub trait PropertyStruct {
    fn append_property_list(
        &self,
        properties: &mut Vec<crate::godot::register::info::PropertyInfo>,
    );
    fn get_property(&self, property: &[char]) -> Option<Variant>;
    fn set_property(&mut self, property: &[char], value: &Variant) -> bool;
}

#[macro_export]
macro_rules! property_struct {
    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $base_vis:vis base: $base:ty,
            $(
                $(#[$($field_meta:tt)*])*
                $field_vis:vis $field:ident: $ty:ty,
            )*
        }
    ) => {
        $(#[$struct_meta])*
        $vis struct $name { $base_vis base: $base, $($field_vis $field: $ty,)* }

        $crate::property_struct! {
            @impl $name, true,
            $($(#[$($field_meta)*])* $field_vis $field: $ty,)*
        }
    };

    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$($field_meta:tt)*])*
                $field_vis:vis $field:ident: $ty:ty,
            )*
        }
    ) => {
        $(#[$struct_meta])*
        $vis struct $name { $($field_vis $field: $ty,)* }

        $crate::property_struct! {
            @impl $name, false,
            $($(#[$($field_meta)*])* $field_vis $field: $ty,)*
        }
    };

    (
        @impl $name:ident, $has_base:tt,
        $(
            $(#[group($group_name:literal$(, $group_prefix:literal)?)])?
            $(#[subgroup($subgroup_name:literal$(, $subgroup_prefix:literal)?)])?
            $(#[usage($($usage_flag:ident)|*$(|)?)])?
            $(#[hint($hint_kind:ident$(, $hint_string:literal)?)])?
            $field_vis:vis $field:ident: $ty:ty,
        )*
    ) => {
        impl $crate::properties::PropertyStruct for $name {
            #[inline(always)]
            fn append_property_list(&self, properties: &mut Vec<$crate::godot::register::info::PropertyInfo>) {
                $crate::property_struct!(@base_op $has_base, self, append_property_list(properties), ());

                #[allow(unused_imports)]
                use $crate::godot::register::info::{PropertyInfo, PropertyUsageFlags, PropertyHint, PropertyHintInfo};

                $(
                    {
                        $(properties.push(PropertyInfo::new_group($group_name, $crate::property_struct!(@optional $($group_prefix)? "")));)?
                        $(properties.push(PropertyInfo::new_subgroup($subgroup_name, $crate::property_struct!(@optional $($subgroup_prefix)? "")));)?

                        let mut info = PropertyInfo::new_export::<$ty>(stringify!($field));
                        $(info.usage = $(PropertyUsageFlags::$usage_flag |)+ PropertyUsageFlags::NONE;)?
                        $(info = info.with_hint_info(PropertyHintInfo {
                            hint: PropertyHint::$hint_kind,
                            hint_string: $crate::property_struct!(@optional $($hint_string)? "").into(),
                        });)?

                        properties.push(info);
                    }
                )*
            }

            #[inline(always)]
            fn get_property(&self, property: &[char]) -> Option<$crate::godot::builtin::Variant> {
                $(
                    #[allow(non_upper_case_globals)]
                    const $field: u64 = $crate::properties::hash_str(stringify!($field));
                )*

                match $crate::properties::hash_chars(property) {
                    $(
                        #[allow(non_upper_case_globals)]
                        $field => Some($crate::godot::builtin::Variant::from(self.$field.clone())),
                    )*
                    _ => $crate::property_struct!(@base_op $has_base, self, get_property(property), None),
                }
            }

            #[inline(always)]
            fn set_property(&mut self, property: &[char], value: &$crate::godot::builtin::Variant) -> bool {
                $(
                    #[allow(non_upper_case_globals)]
                    const $field: u64 = $crate::properties::hash_str(stringify!($field));
                )*

                match $crate::properties::hash_chars(property) {
                    $(
                        #[allow(non_upper_case_globals)]
                        $field => {
                            if let Ok(converted) = value.try_to() { self.$field = converted; true } else { false }
                        },
                    )*
                    _ => $crate::property_struct!(@base_op $has_base, self, set_property(property, value), false),
                }
            }
        }
    };

    (@optional $prefix:literal $fallback:literal) => {
        $prefix
    };
    (@optional $fallback:literal) => {
        $fallback
    };

    (@base_op true, $self:ident, $method:ident($($args:ident),*), $fallback:expr) => {{
        use godot_utils::properties::PropertyStruct;
        $self.base.$method($($args),*)
    }};
    (@base_op false, $self:ident, $method:ident($($args:ident),*), $fallback:expr) => {
        $fallback
    };
}

#[macro_export]
macro_rules! property_bitflags {
    (
        $(#[$meta:meta])*
        pub struct $name:ident: $ty:ty {
            $(const $flag:ident = $value:expr;)*
        }
    ) => {
        $crate::property_bitflags! {
            @impl [$(#[$meta])*] [pub] $name: $ty {
                $(const $flag = $value;)*
            }
        }
    };

    (
        $(#[$meta:meta])*
        struct $name:ident: $ty:ty {
            $(const $flag:ident = $value:expr;)*
        }
    ) => {
        $crate::property_bitflags! {
            @impl [$(#[$meta])*] [private] $name: $ty {
                $(const $flag = $value;)*
            }
        }
    };

    (@decl [$($meta:tt)*] [pub] $name:ident: $ty:ty) => {
        $($meta)*
        #[derive(Var, Export)]
        #[godot(transparent)]
        pub struct $name($ty);
    };

    (@decl [$($meta:tt)*] [private] $name:ident: $ty:ty) => {
        $($meta)*
        #[derive(Var, Export)]
        #[godot(transparent)]
        struct $name($ty);
    };

    (
        @impl [$($meta:tt)*] [$vis:ident] $name:ident: $ty:ty {
            $(const $flag:ident = $value:expr;)*
        }
    ) => {
        $crate::property_bitflags! {
            @decl [$($meta)*] [$vis] $name: $ty
        }

        impl $crate::godot::meta::GodotConvert for $name {
            type Via = $ty;

            fn godot_shape() -> $crate::godot::meta::shape::GodotShape {
                const ENUMERATORS: &[$crate::godot::meta::shape::EnumeratorShape] = &[
                    $(
                        $crate::godot::meta::shape::EnumeratorShape::new_int(stringify!($flag), $value as i64),
                    )*
                ];

                $crate::godot::meta::shape::GodotShape::Enum {
                    variant_type: $crate::godot::meta::element_variant_type::<Self>(),
                    enumerators: std::borrow::Cow::Borrowed(ENUMERATORS),
                    godot_name: None,
                    is_bitfield: true,
                }
            }
        }

        impl $crate::godot::meta::ToGodot for $name {
            type Pass = <$ty as $crate::godot::meta::ToGodot>::Pass;

            fn to_godot(&self) -> $crate::godot::meta::ToArg<'_, Self::Via, Self::Pass> {
                $crate::godot::meta::ToGodot::to_godot(&self.0)
            }
        }

        impl $crate::godot::meta::FromGodot for $name {
            fn try_from_godot(
                via: $ty,
            ) -> ::std::result::Result<Self, $crate::godot::meta::error::ConvertError> {
                Ok(Self { 0: via })
            }
        }

        impl $crate::godot::meta::Element for $name where Self: 'static {}

        $crate::bitflags::bitflags! {
            impl $name: $ty {
                $(const $flag = $value;)*
            }
        }
    };
}
