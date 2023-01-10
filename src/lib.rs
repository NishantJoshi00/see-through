use std::ops;

#[cfg(feature = "macros")]
pub use see_derive;

///
/// This trait let's us visit a specific field inside a struct. similar to lens in Haskell
///
pub trait See<N> {
    type Inner;
    fn get(&self) -> &Self::Inner;
    fn set(&mut self) -> &mut Self::Inner;
}

pub trait Look<I>:
    See<I> + ops::Index<I, Output = Self::Inner> + ops::IndexMut<I, Output = Self::Inner>
{
}

impl<T, I, D> Look<I> for T where
    T: See<I, Inner = D> + ops::Index<I, Output = D> + ops::IndexMut<I, Output = D>
{
}
