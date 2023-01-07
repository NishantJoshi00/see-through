//!
//! Derive For the traits `See` and an extension trait `Load`
//!

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
mod inner;

///
/// Extension of the trait `See`, providing auto implementation for `See`,
/// while also combining the loading visitors into a single place.
///
/// ## Example
/// ```rust
/// #[derive(see_derive::See)]
/// struct Point {
///     x: i32,
///     y: i32
/// }
/// ```
/// This above example implements `See<crate::see_t::__x>` and `See<crate::see_t::__y>` for the point struct
///
#[proc_macro_derive(See)]
pub fn see_derive(input: TokenStream) -> TokenStream {
    match inner::see_derive(parse_macro_input!(input as DeriveInput)) {
        Ok(res) => res,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

///
/// The derive macro includes the visitors in your codebase. Which
/// are created during the use of `#[derive(See)]`
///
/// ## Example
/// This derive must be used at the end of the origin location of your current crate, i.e. `lib.rs` or `main.rs`
/// or a way that will allow use to access the data as `crate::see_t`
/// ```rust
/// // In lib.rs
///
/// // .. your code goes here
///
///
/// pub(crate) mod see_t {
///     #[derive(see_derive::Load)]
///     struct SeeT;
/// }
/// ``
///
///
#[proc_macro_derive(Load)]
pub fn load_fields(input: TokenStream) -> TokenStream {
    inner::load_fields(parse_macro_input!(input as DeriveInput)).into()
}
