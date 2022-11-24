#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(GenerateGetterTraitsForStructFieldsFromTufaCommon)]
pub fn derive_generate_getter_traits_for_struct_fields_from_tufa_common(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::TufaCommon)
}

#[proc_macro_derive(GenerateGetterTraitsForStructFieldsFromCrate)]
pub fn derive_generate_getter_traits_for_struct_fields_from_crate(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::Crate)
}

fn generate(
    input: proc_macro::TokenStream,
    path: proc_macro_helpers::path::Path,
) -> proc_macro::TokenStream {
    use convert_case::Casing;
    let ast: syn::DeriveInput =
        syn::parse(input).expect("GenerateGetterTraitsForStructFields syn::parse(input) failed");
    let ident = &ast.ident;
    let generated_traits_implementations = match ast.data {
        syn::Data::Struct(datastruct) => datastruct.fields.into_iter().map(|field| {
            let (field_ident, pascal_case_field_ident) = match field.ident {
                None => panic!("field.ident is None"),
                Some(field_ident) => (
                    field_ident.clone(),
                    syn::Ident::new(
                        &format!("{field_ident}").to_case(convert_case::Case::Pascal),
                        ident.span(),
                    ),
                ),
            };
            let type_ident = field.ty;
            let path_trait_ident =
                format!("{path}::config_mods::traits::fields::Get{pascal_case_field_ident}")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path_trait_ident parse failed");
            let function_name_ident = format!("get_{field_ident}")
                .parse::<proc_macro2::TokenStream>()
                .expect("function_name_ident parse failed");
            quote::quote! {
                impl #path_trait_ident for #ident {
                    fn #function_name_ident (&self) -> &#type_ident {
                        &self.#field_ident
                    }
                }
            }
        }),
        _ => panic!("GenerateGetterTraitsForStructFields only works on Struct"),
    };
    let gen = quote::quote! {
        #(#generated_traits_implementations)*
    };
    gen.into()
}
