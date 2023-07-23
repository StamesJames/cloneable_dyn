//! # Disclaimer
//! I'm quite new to rust and even more so to publishing crates so this crate should be used with caution and feedback is welcome :-)
//!
//! # cloneable_dyn
//! With this crate you can make your traits cloneablr, your structs/enums cloneable as given cloneable traits and derive clone on structs/enums that contain dyn objects of your traits.
//!
//! Make a trait cloneable by attaching the attribute #[dyn_cloneable]. This will generate a new supertrait that contains a function `fn clone_dyn(&self) -> Box<dyn #trait_ident>`
//! Here an Example:
//! ```
//! #[dyn_cloneable]
//! trait TestTrait {}
//!
//! // will generate
//! // pub trait __TestTrait__DynCloneAutoDerive__ {
//! //    fn clone_dyn(&self) -> Box<dyn TestTrait>;
//! // }
//! // and make it a supertrait of TestTrait
//! ```
//!

use proc_macro::TokenStream;
use syn::{self, parse::Parser, punctuated::Punctuated, token::Comma, Ident, Item};
mod clone_dyn_derive;
mod dyn_cloneable;
use dyn_cloneable::make_clonable;
mod utils;
use quote::{format_ident, quote};

#[proc_macro_derive(CloneDyn)]
pub fn clone_dyn_derive(input: TokenStream) -> TokenStream {
    let ast = clone_dyn_derive::parse(input.into());
    clone_dyn_derive::impl_clone_dyn(&ast).into()
}

#[proc_macro_attribute]
pub fn dyn_cloneable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = dyn_cloneable::parse(item.into());
    make_clonable(item).into()
}

#[proc_macro_attribute]
pub fn dyn_clonable_for_traits(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Comma>::parse_terminated;
    let args = parser.parse(attr).expect("wasn't able to parse attrs");
    let item: Item = syn::parse(item).expect("wasn't able to parse Item");
    let item_ident: &Ident = match &item {
        Item::Enum(i) => &i.ident,
        Item::Struct(i) => &i.ident,
        _ => panic!("item must be Enum or struct"),
    };
    let mut impls = quote!();
    for arg in args {
        let trait_ident = format_ident!("{}DynCloneAutoDerive", arg);
        impls = quote!(
            impl #trait_ident for #item_ident {
                fn clone_dyn(&self) -> Box<dyn #arg> {
                    Box::new(self.clone())
                }
            }

            #impls
        )
    }
    quote!(#item #impls).into()
}
