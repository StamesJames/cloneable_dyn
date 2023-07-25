//! # Disclaimer
//! I'm quite new to rust and even more so to publishing crates so this crate should be used with caution and feedback is welcome :-)
//!
//! # cloneable_dyn
//! With this crate you can make your traits cloneablr, your structs/enums cloneable as given cloneable traits and derive clone on structs/enums that contain dyn objects of your traits.
//!
//! Make a trait cloneable by attaching the attribute #[dyn_cloneable]. This will generate a new supertrait that contains a function `fn clone_dyn(&self) -> Box<dyn #trait_ident>`
//! Here an Example:
//! ```
//! use cloneable_dyn::dyn_cloneable;
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
use syn::Item;
mod clone_dyn_derive;
mod dyn_cloneable;
use dyn_cloneable::make_clonable;
mod dyn_cloneable_for_traits;
mod utils;

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
pub fn dyn_cloneable_for_traits(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = dyn_cloneable_for_traits::parse_args(attr.into());
    let item: Item = dyn_cloneable_for_traits::parse(item.into());

    dyn_cloneable_for_traits::impl_trait_clone_traits(args, item).into()
}
