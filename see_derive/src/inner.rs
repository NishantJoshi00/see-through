use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;
use syn::{DeriveInput, spanned::Spanned};
use std::sync;

static FIELDS: sync::Mutex<Vec<String>> = sync::Mutex::new(Vec::new());



pub(crate) fn see_derive(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { named, .. }), .. }) => {
            named.into_iter()
                .filter_map(|field| {
                    field.ident.map(|idn| (idn, field.ty))
                })
                .map(|(ident, ty)| {
                    (field_consumer(ident), ty)
                })
        }
        _ => Err(syn::Error::new(input.clone().span(), "The datatype should be a struct with named fields"))?
    };
    let (field, types) = split_iter(fields);
    let (old_f, new_f) = split_iter(field);

    Ok(quote! {
        #(
            impl See<crate::see_t::#new_f> for #name {
                type Inner = #types;
                fn get(&self) -> &Self::Inner {
                    &self.#old_f
                }
                fn set(&mut self) -> &mut Self::Inner {
                    &mut self.#old_f
                }
            }

            impl std::ops::Index<crate::see_t::#new_f> for #name {
                type Output = #types;

                fn index(&self, index: crate::see_t::#new_f) -> &Self::Output {
                    <Self as See<crate::see_t::#new_f>>::get(self)
                }
            }

            impl std::ops::IndexMut<crate::see_t::#new_f> for #name {
                fn index_mut(&mut self, index: crate::see_t::#new_f) -> &mut Self::Output {
                    <Self as See<crate::see_t::#new_f>>::set(self)
                }
            }
        )*
    })
}

pub(crate) fn load_fields(input: DeriveInput) -> TokenStream {
    create_struct_stream(input.span())
}

fn field_consumer(idn: Ident) -> (Ident, Ident) {
    let mut store = FIELDS.lock().unwrap();
    match store.contains(&idn.to_string()) {
        true => (),
        false => {
            store.push(idn.to_string()); 
        }
    }
    (idn.clone(), field_to_ident(&idn.to_string(), idn.span()))
}

fn field_to_ident<'a>(name: &'a str, span: Span) -> Ident {
    Ident::new(&format!("{}", name.to_uppercase()), span)
}

fn create_struct_stream(span: Span) -> TokenStream {
    let structs = FIELDS.lock().unwrap();
    let structs = structs
        .iter().map(|name| {
            field_to_ident(name, span)
        });
    quote! {
        #(pub struct #structs;)*
    }
}

fn split_iter<T, U>(input_iter: impl Iterator<Item = (T, U)>) -> (impl Iterator<Item = T>, impl Iterator<Item = U>) {
    let (mut s1, mut s2) = (Vec::new(), Vec::new());
    input_iter.for_each(|(t, u)| {
        s1.push(t);
        s2.push(u);
    });


    (s1.into_iter(), s2.into_iter())
}