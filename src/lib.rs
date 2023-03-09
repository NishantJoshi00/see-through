#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR" ), "/", "README.md"))]

use std::ops;

#[cfg(feature = "macros")]
pub use see_derive;

///
/// This trait let's us visit a specific field inside a struct. similar to lens in Haskell
///
/// e.g.
/// Consider a scenerio where, you are passing a generic to a function and incidentally you also want
/// to get a value inside it. Let's assume that there are multiple structs with the same field
/// `foo` you might wish to for example view it. But, it would be impossible to do so,
///
pub trait See<N> {
    /// This is the type of the value that we want to [`See`][See]
    type Inner;
    /// By using `&self` get the internal value [`Inner`][See::Inner]
    fn get(&self) -> &Self::Inner;
    /// If we have access to `&mut self` try and mutate [`&mut Inner`][See::Inner]
    fn set(&mut self) -> &mut Self::Inner;
}

///
/// A trait to provide an additional `ops::Index` functionality over `See`
///
/// This trait, is an extension to that See trait, which also allows easier indexing with the help
/// of `ops::Index`. This allows you to use simple formats like `value[see_t::x]` instead of using
/// functions to get or set these values. This not only helps with ease of use, but also allows you
/// to implement multiple Look clause on the generic, unlocking more lookups inside the generic.
///
pub trait Look<I>:
    See<I> + ops::Index<I, Output = Self::Inner> + ops::IndexMut<I, Output = Self::Inner>
{
}

/// This implementation is automatically derived when all the conditions are met.
#[automatically_derived]
impl<T, I, D> Look<I> for T where
    T: See<I, Inner = D> + ops::Index<I, Output = D> + ops::IndexMut<I, Output = D>
{
}
