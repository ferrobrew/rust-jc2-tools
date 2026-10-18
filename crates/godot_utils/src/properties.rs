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
    fn append_property_list(&self, properties: &mut Vec<::godot::register::info::PropertyInfo>);
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
                $(#[group($group_name:literal, $group_prefix:literal)])?
                $(#[subgroup($subgroup_name:literal, $subgroup_prefix:literal)])?
                $(#[usage($($usage_flag:ident)|*$(|)?)])?
                $(#[hint($hint_kind:ident, $hint_string:literal)])?
                $field_vis:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        $vis struct $name {
            $base_vis base: $base,
            $($field_vis $field: $ty,)*
        }

        impl $crate::properties::PropertyStruct for $name {
            #[inline(always)]
            fn append_property_list(&self, properties: &mut Vec<::godot::register::info::PropertyInfo>) {
                self.base.append_property_list(properties);

                $(
                    {
                        $(
                            properties.push(::godot::register::info::PropertyInfo::new_group($group_name, $group_prefix));
                        )?
                        $(
                            properties.push(::godot::register::info::PropertyInfo::new_subgroup($subgroup_name, $subgroup_prefix));
                        )?
                        let mut info = ::godot::register::info::PropertyInfo::new_export::<$ty>(stringify!($field));
                        $(
                            info.usage = $(::godot::register::info::PropertyUsageFlags::$usage_flag |)+ ::godot::register::info::PropertyUsageFlags::NONE;
                        )?
                        $(
                            info = info.with_hint_info(::godot::register::info::PropertyHintInfo {
                                hint: ::godot::register::info::PropertyHint::$hint_kind,
                                hint_string: $hint_string.into()
                            });
                        )?
                        properties.push(info);
                    }
                )*
            }

            #[inline(always)]
            fn get_property(&self, property: &[char]) -> Option<::godot::builtin::Variant> {
                $(
                    #[allow(non_upper_case_globals)]
                    const $field: u64 = $crate::properties::hash_str(stringify!($field));
                )*
                match $crate::properties::hash_chars(property) {
                    $(
                        #[allow(non_upper_case_globals)]
                        $field => Some(::godot::builtin::Variant::from(self.$field.clone())),
                    )*
                    _ => self.base.get_property(property),
                }
            }

            #[inline(always)]
            fn set_property(&mut self, property: &[char], value: &::godot::builtin::Variant) -> bool {
                $(
                    #[allow(non_upper_case_globals)]
                    const $field: u64 = $crate::properties::hash_str(stringify!($field));
                )*
                match $crate::properties::hash_chars(property) {
                    $(
                        #[allow(non_upper_case_globals)]
                        $field => {
                            if let Ok(converted) = value.try_to() {
                                self.$field = converted;
                                true
                            } else {
                                false
                            }
                        },
                    )*
                    _ => self.base.set_property(property, value),
                }
            }
        }
    };

    (
        $(#[$struct_meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[group($group_name:literal, $group_prefix:literal)])?
                $(#[subgroup($subgroup_name:literal, $subgroup_prefix:literal)])?
                $(#[usage($($usage_flag:ident)|*$(|)?)])?
                $(#[hint($hint_kind:ident, $hint_string:literal)])?
                $field_vis:vis $field:ident : $ty:ty
            ),* $(,)?
        }
    ) => {
        $(#[$struct_meta])*
        $vis struct $name {
            $($field_vis $field: $ty,)*
        }

        impl $crate::properties::PropertyStruct for $name {
            #[inline(always)]
            fn append_property_list(&self, properties: &mut Vec<::godot::register::info::PropertyInfo>) {
                $(
                    {
                        $(
                            properties.push(::godot::register::info::PropertyInfo::new_group($group_name, $group_prefix));
                        )?
                        $(
                            properties.push(::godot::register::info::PropertyInfo::new_subgroup($subgroup_name, $subgroup_prefix));
                        )?
                        let mut info = ::godot::register::info::PropertyInfo::new_export::<$ty>(stringify!($field));
                        $(
                            info.usage = $(::godot::register::info::PropertyUsageFlags::$usage_flag |)+ ::godot::register::info::PropertyUsageFlags::NONE;
                        )?
                        $(
                            info = info.with_hint_info(::godot::register::info::PropertyHintInfo {
                                hint: ::godot::register::info::PropertyHint::$hint_kind,
                                hint_string: $hint_string.into()
                            });
                        )?
                        properties.push(info);
                    }
                )*
            }

            #[inline(always)]
            fn get_property(&self, property: &[char]) -> Option<::godot::builtin::Variant> {
                $(
                    #[allow(non_upper_case_globals)]
                    const $field: u64 = $crate::properties::hash_str(stringify!($field));
                )*
                match $crate::properties::hash_chars(property) {
                    $(
                        #[allow(non_upper_case_globals)]
                        $field => Some(::godot::builtin::Variant::from(self.$field.clone())),
                    )*
                    _ => None,
                }
            }

            #[inline(always)]
            fn set_property(&mut self, property: &[char], value: &::godot::builtin::Variant) -> bool {
                $(
                    #[allow(non_upper_case_globals)]
                    const $field: u64 = $crate::properties::hash_str(stringify!($field));
                )*
                match $crate::properties::hash_chars(property) {
                    $(
                        #[allow(non_upper_case_globals)]
                        $field => {
                            if let Ok(converted) = value.try_to() {
                                self.$field = converted;
                                true
                            } else {
                                false
                            }
                        },
                    )*
                    _ => false,
                }
            }
        }
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

        impl ::godot::meta::GodotConvert for $name {
            type Via = $ty;

            fn godot_shape() -> ::godot::meta::shape::GodotShape {
                const ENUMERATORS: &[::godot::meta::shape::EnumeratorShape] = &[
                    $(
                        ::godot::meta::shape::EnumeratorShape::new_int(stringify!($flag), $value as i64),
                    )*
                ];

                ::godot::meta::shape::GodotShape::Enum {
                    variant_type: ::godot::meta::element_variant_type::<Self>(),
                    enumerators: std::borrow::Cow::Borrowed(ENUMERATORS),
                    godot_name: None,
                    is_bitfield: true,
                }
            }
        }

        impl ::godot::meta::ToGodot for $name {
            type Pass = <$ty as ::godot::meta::ToGodot>::Pass;

            fn to_godot(&self) -> ::godot::meta::ToArg<'_, Self::Via, Self::Pass> {
                ::godot::meta::ToGodot::to_godot(&self.0)
            }
        }

        impl ::godot::meta::FromGodot for $name {
            fn try_from_godot(
                via: $ty,
            ) -> ::std::result::Result<Self, ::godot::meta::error::ConvertError> {
                Ok(Self { 0: via })
            }
        }

        impl ::godot::meta::Element for $name where Self: 'static {}

        bitflags::bitflags! {
            impl $name: $ty {
                $(const $flag = $value;)*
            }
        }
    };
}
