use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemTrait, TraitBound};

use crate::utils::make_trait_ident_for_trait;

pub fn parse(input: TokenStream) -> ItemTrait {
    syn::parse2(input).expect("Macro has to be attached to Trait")
}

pub fn make_clonable(mut trait_item: syn::ItemTrait) -> TokenStream {
    let trait_ident = &trait_item.ident;
    let clone_trait_ident = make_trait_ident_for_trait(trait_ident);
    let stream: TokenStream = quote!(#clone_trait_ident);
    let new_bound: TraitBound = syn::parse2(stream).unwrap();
    trait_item
        .supertraits
        .push(syn::TypeParamBound::Trait(new_bound));

    quote! {
        #trait_item
        pub trait #clone_trait_ident {
            fn clone_dyn(&self) -> Box<dyn #trait_ident>;
        }
    }
}
