#[macro_export]
macro_rules! hash_string_group {
    ($vis:vis $name:ident {
        $($variant:ident = $value:expr),+
        $(,)?
    }) => {
        $crate::paste! {
            $vis trait $name {
                $(const $variant: $crate::HashString = $crate::HashString::from_str($value);)+
            }

            impl $name for $crate::HashString {}
        }
    };
}

#[macro_export]
macro_rules! hash_string_group_enum {
    ($vis:vis $name:ident {
        $($variant:ident = $value:expr),+
        $(,)?
    }) => {
        $crate::paste! {
            $crate::hash_string_group!($vis [<$name Hashes>] {
                $($variant = $value,)+
            });

            $vis enum $name {
                $([<$variant:camel>],)+
            }

            impl $name {
                #[inline]
                $vis const fn to_str(&self) -> &str {
                    match *self {
                        $($name::[<$variant:camel>] => $value,)+
                    }
                }

                #[inline]
                $vis const fn to_hash(&self) -> $crate::HashString {
                    match *self {
                        $($name::[<$variant:camel>] => $crate::HashString::from_str($value),)+
                    }
                }

                #[inline]
                $vis const fn from_hash(hash: $crate::HashString) -> Option<$name> {
                    match hash {
                        $($crate::HashString::$variant => Some($name::[<$variant:camel>]),)+
                        _ => None,
                    }
                }
            }

            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    self.to_str()
                }
            }

            impl From<$name> for String {
                fn from(value: $name) -> Self {
                    value.to_str().into()
                }
            }

            impl TryFrom<$crate::HashString> for $name {
                type Error = &'static str;

                fn try_from(value: $crate::HashString) -> Result<Self, Self::Error> {
                    if let Some(result) = Self::from_hash(value) {
                        Ok(result.into())
                    } else {
                        Err(concat!("failed convert from ", stringify!($crate::HashString), " to ",stringify!($name)))
                    }
                }
            }
        }
    }
}
