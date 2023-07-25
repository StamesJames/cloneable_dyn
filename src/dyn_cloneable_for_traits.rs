use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::Parser, punctuated::Punctuated, token::Comma, Ident, Item};

use crate::utils::make_trait_ident_for_trait;

pub fn parse(input: TokenStream) -> Item {
    syn::parse2(input).expect("Macro has to be attached to Trait")
}

pub fn parse_args(input: TokenStream) -> Punctuated<Ident, Comma> {
    let parser = Punctuated::<Ident, Comma>::parse_terminated;
    parser.parse2(input).expect("wasn't able to parse attrs")
}

pub fn impl_trait_clone_traits(args: Punctuated<Ident, Comma>, item: Item) -> TokenStream {
    let item_ident: &Ident = match &item {
        Item::Enum(i) => &i.ident,
        Item::Struct(i) => &i.ident,
        _ => panic!("item must be Enum or struct"),
    };
    let mut impls = quote!();
    for arg in args {
        let trait_ident = make_trait_ident_for_trait(&arg);
        impls = quote!(
            impl #trait_ident for #item_ident {
                fn clone_dyn(&self) -> Box<dyn #arg> {
                    Box::new(self.clone())
                }
            }

            #impls
        )
    }
    quote!(#item #impls)
}
