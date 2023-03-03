## See Derive

A inner crate that provides `#[derive(...)]` for `See` & `Look` trait in the repository.



## Usage


The macros provided are
- `#[derive(See)]` 
- `#[derive(Look)]` this will automatically derive `See` as it's an internal dependency
- `auto_load!()` this macro loads all the unit structs which allow both `See` and `Look` to access fields inside generics. This macro should be called in the root of the repository and after all the `See` and `Look` derivation are done.



## Description

For more details on how to use this, please visit [see-through](https://github.com/NishantJoshi00/see-through)

