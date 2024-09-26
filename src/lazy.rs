/// Implements `get` for a field using copy
macro_rules! impl_get_copy {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<get_ $field>](&self) -> $type {
                self.$field
            }
        }
    };
}
pub(crate) use impl_get_copy;

// /// Implements `get` for a field using reference
// macro_rules! impl_get_ref {
//     ($field:ident, $type:ty) => {
//         paste::paste! {
//             #[allow(unused)]
//             pub fn [<get_ $field>](&self) -> &$type {
//                 &self.$field
//             }
//         }
//     };
// }
// pub(crate) use impl_get_ref;

// /// Implements `get` for a field using clone
// macro_rules! impl_get_clone {
//     ($field:ident, $type:ty) => {
//         paste::paste! {
//             #[allow(unused)]
//             pub fn [<get_ $field>](&self) -> $type {
//                 self.$field.clone()
//             }
//         }
//     };
// }
// pub(crate) use impl_get_clone;

// /// Implements `set` for a field
// macro_rules! impl_set {
//     ($field:ident, $type:ty) => {
//         paste::paste! {
//             #[allow(unused)]
//             pub fn [<set_ $field>](&mut self, val: $type) {
//                 self.$field = val;
//             }
//         }
//     };
// }
// pub(crate) use impl_set;

/// Implements `with` for a field
macro_rules! impl_with {
    ($field:ident, $type:ty) => {
        paste::paste! {
            #[allow(unused)]
            pub fn [<with_ $field>](mut self, val: $type) -> Self {
                self.$field = val;
                self
            }
        }
    };
}
pub(crate) use impl_with;
