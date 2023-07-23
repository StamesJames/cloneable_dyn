use proc_macro2::Ident;
use quote::format_ident;

pub fn make_trait_ident_for_trait(ident: &Ident) -> Ident {
    format_ident!("__{ident}__DynCloneAutoDerive__")
}
