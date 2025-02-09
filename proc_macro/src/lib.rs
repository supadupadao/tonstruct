use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ToCell)]
pub fn derive_to_cell(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, data, .. } = input;

    let data = if let syn::Data::Struct(data) = data {
        data
    } else {
        unimplemented!()
    };

    let fields = data.fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            builder = ToCell::store(&self.#name, builder).map_err(|err| anyhow::Error::msg(err.to_string()))?;
        }
    });

    let trait_impl = quote! {
        impl ToCell for #ident {
            fn store<'a>(
                &self,
                mut builder: &'a mut tonlib_core::cell::CellBuilder,
            ) -> anyhow::Result<&'a mut tonlib_core::cell::CellBuilder>{
                #(#fields)*
                Ok(builder)
            }
        }
    };

    trait_impl.into()
}

#[proc_macro_derive(FromCell)]
pub fn derive_from_cell(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, data, .. } = input;

    let data = if let syn::Data::Struct(data) = data {
        data
    } else {
        unimplemented!()
    };

    let fields = data.fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: FromCell::load(parser)?,
        }
    });

    let trait_impl = quote! {
        impl FromCell for #ident {
            fn load(parser: &mut tonlib_core::cell::CellParser) -> anyhow::Result<Self> {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };

    trait_impl.into()
}
