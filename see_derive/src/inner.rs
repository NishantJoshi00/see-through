use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::sync;
use syn::{spanned::Spanned, DeriveInput};

static FIELDS: sync::Mutex<Vec<String>> = sync::Mutex::new(Vec::new());

pub(crate) fn see_derive(input: DeriveInput, look: bool) -> Result<TokenStream, syn::Error> {
    let look = if look {
        quote! { all() }
    } else {
        quote! { not(all()) }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => named
            .into_iter()
            .filter_map(|field| field.ident.map(|idn| (idn, field.ty)))
            .map(|(ident, ty)| (field_consumer(&ident), ty)),
        _ => Err(syn::Error::new(
            input.clone().span(),
            "The datatype should be a struct with named fields",
        ))?,
    };
    let (field, types) = split_iter(fields);
    let (old_f, new_f) = split_iter(field);
    Ok(quote! {
        #(
            impl #impl_generics See<crate::see_t::#new_f> for #name #ty_generics #where_clause {
                type Inner = #types;
                fn get(&self) -> &Self::Inner {
                    &self.#old_f
                }
                fn set(&mut self) -> &mut Self::Inner {
                    &mut self.#old_f
                }
            }


            #[cfg(#look)]
            impl #impl_generics std::ops::Index<crate::see_t::#new_f> for #name #ty_generics #where_clause {
                type Output = #types;

                fn index(&self, index: crate::see_t::#new_f) -> &Self::Output {
                    <Self as See<crate::see_t::#new_f>>::get(self)
                }
            }

            #[cfg(#look)]
            impl #impl_generics std::ops::IndexMut<crate::see_t::#new_f> for #name #ty_generics #where_clause {
                fn index_mut(&mut self, index: crate::see_t::#new_f) -> &mut Self::Output {
                    <Self as See<crate::see_t::#new_f>>::set(self)
                }
            }
        )*
    })
}

pub(crate) fn load_fields(input: &DeriveInput) -> TokenStream {
    create_struct_stream(input.span())
}
pub(crate) fn auto_load() -> TokenStream {
    let struct_stream = create_struct_stream(Span::call_site());
    quote! {
        pub(crate) mod see_t {
            #struct_stream
        }
    }
}

fn field_consumer(idn: &Ident) -> (Ident, Ident) {
    let mut store = FIELDS.lock().unwrap();
    if !store.contains(&idn.to_string()) {
        store.push(idn.to_string());
    }
    (idn.clone(), field_to_ident(&idn.to_string(), idn.span()))
}

fn field_to_ident(name: &str, span: Span) -> Ident {
    Ident::new(&name.to_uppercase(), span)
}

fn create_struct_stream(span: Span) -> TokenStream {
    let structs = FIELDS.lock().unwrap();
    let structs = structs.iter().map(|name| field_to_ident(name, span));
    quote! {
        #(pub struct #structs;)*
    }
}

fn split_iter<T, U>(
    input_iter: impl Iterator<Item = (T, U)>,
) -> (impl Iterator<Item = T>, impl Iterator<Item = U>) {
    let (mut s1, mut s2) = (Vec::new(), Vec::new());
    input_iter.for_each(|(t, u)| {
        s1.push(t);
        s2.push(u);
    });

    (s1.into_iter(), s2.into_iter())
}
