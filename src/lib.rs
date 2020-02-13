#![allow(unused_macros)]

macro_rules! _default_impl {
    ($default:expr) => { $default };
    () => { ::std::default::Default::default() };
}

/// Supports struct definition syntax with additional default parameters.
///
/// Each field may be assigned a default value, which will be used in an automatically provided
/// `Default` implementation. Fields with no default assigned use their own `Default::default()`.
///
/// Example:
/// ```
/// struct_with_default! {
/// User {
///     id: u64,
///     name: String = "Bob",
/// }
///
/// //...
///
/// let user = User::default()
/// assert_eq!(user.name, "Bob");
/// ```
macro_rules! struct_with_default {
    (
        $(
            #[$meta:meta]
        )*
        $type:ident {
            $($field:ident: $fieldtype:ty $(= $default:expr)? $(,)?)*
        }
    ) => {
        $(#[$meta])*
        struct $type {
            $($field: $fieldtype),
            *
        }
        impl Default for $type {
            fn default() -> Self {
                $type {$(
                    $field: _default_impl!($($default)?),
                )*}
            }
        }
    };
}
