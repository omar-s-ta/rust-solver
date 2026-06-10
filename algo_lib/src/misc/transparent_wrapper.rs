/// Defines a newtype that transparently forwards to its single wrapped field
/// via [`Deref`] and [`DerefMut`].
///
/// This is useful for creating a distinct type (for example, to attach
/// inherent methods or trait implementations) while still being able to call
/// all of the inner type's methods directly on the wrapper.
///
/// # Syntax
///
/// ```text
/// transparent_wrapper!(Name = InnerType);
/// transparent_wrapper!(Name<T, U> = InnerType);
/// transparent_wrapper!(Name = InnerType, derive Clone, Debug);
/// ```
///
/// - `Name` — the wrapper type to generate. It is declared as
///   `pub struct Name(InnerType);` with a single private field.
/// - `<T, U>` — optional generic parameters carried over to both the struct
///   definition and the generated `Deref`/`DerefMut` impls.
/// - `InnerType` — the wrapped type, exposed as `Deref::Target`.
/// - `derive ...` — an optional comma-separated list of traits to `#[derive]`
///   on the generated struct.
///
/// # Requirements
///
/// [`Deref`] and [`DerefMut`] (from `std::ops`) must be in scope at the macro
/// call site.
///
/// # Examples
///
/// ```ignore
/// use std::ops::{Deref, DerefMut};
///
/// transparent_wrapper!(Wrapper<T> = Vec<T>, derive Clone, Debug);
///
/// let mut w = Wrapper(vec![1, 2, 3]);
/// w.push(4);            // Vec method via DerefMut
/// assert_eq!(w.len(), 4); // Vec method via Deref
/// ```
#[macro_export]
macro_rules! transparent_wrapper {
    ($name: ident $(<$($par: ident$(,)?)+>)? = $t: ty $(, derive $($d: ty$(,)?)+)?) => {
        $(#[derive($($d,)+)])?
        pub struct $name$(<$($par,)+>)?($t);

        impl$(<$($par,)+>)? Deref for $name$(<$($par,)+>)? {
            type Target = $t;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl$(<$($par,)+>)? DerefMut for $name$(<$($par,)+>)? {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
