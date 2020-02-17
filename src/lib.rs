#![allow(unused_macros)]

/// Supports struct definition syntax with additional default parameters.
///
/// Each field may be assigned a default value, which will be used in an automatically provided
/// `Default` implementation. Fields with no default assigned use their own `Default::default()`.
///
/// Example:
/// ```
/// use struct_with_default::struct_with_default;
///
/// struct_with_default! {
///     pub struct User {
///         id: u64,
///         name: String = "Bob".to_string(),
///     }
/// }
///
/// //...
///
/// let user = User::default();
/// assert_eq!(user.name, "Bob");
/// ```
#[macro_export]
macro_rules! struct_with_default {
    (
        $( #[$meta:meta] )*
        $visibility:vis struct $type:ident {
            $($field:ident: $fieldtype:ty $(= $default:expr)? $(,)?)*
        }
    ) => {
        $(#[$meta])*
        $visibility struct $type {
            $($field: $fieldtype),
            *
        }
        impl Default for $type {
            fn default() -> Self {
                $type {$(
                    $field: struct_with_default!($($default)?),
                )*}
            }
        }
    };
    // for internal use only:
    ($default:expr) => { $default };
    () => { ::std::default::Default::default() };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        struct_with_default! {
            struct User {
                name: String = "Bob".to_string(),
                id: u64
            }
        }

        let user = User::default();
        assert_eq!(user.name, "Bob");
        assert_eq!(user.id, u64::default());
    }
}
