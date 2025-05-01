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
            ToCell::store(&self.#name, builder)?;
        }
    });

    let trait_impl = quote! {
        impl ToCell for #ident {
            fn store(
                &self,
                builder: &mut impl tonstruct::TonstructBuilder,
            ) -> tonstruct::Result<()> {
                #(#fields)*
                Ok(())
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
            fn load(parser: &mut impl tonstruct::TonstructParser) -> tonstruct::Result<Self> {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };

    trait_impl.into()
}
