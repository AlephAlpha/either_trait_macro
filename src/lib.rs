//! Implements your own traits for [`Either`](https://crates.io/crates/either).
//! If your trait is implemented for both type `A` and `B`,
//! then it is automatically implemented for `Either<A, B>`.
//!
//! # Usage
//! When defining a trait, wrap it with the macro `either_trait`.
//!
//! # Example
//! ```rust
//! use either::Either;
//! use either_trait_macro::either_trait;
//!
//! #[either_trait]
//! /// Apply a function `n` times.
//! trait Apply {
//!     fn times<T, F>(&self, t: T, f: F) -> T
//!     where
//!         F: Fn(T) -> T;
//! }
//!
//! struct Once;
//!
//! impl Apply for Once {
//!     fn times<T, F>(&self, t: T, f: F) -> T
//!     where
//!         F: Fn(T) -> T,
//!     {
//!         f(t)
//!     }
//! }
//!
//! impl Apply for u32 {
//!     fn times<T, F>(&self, t: T, f: F) -> T
//!     where
//!         F: Fn(T) -> T,
//!     {
//!         let mut t = t;
//!         for _ in 0..*self {
//!             t = f(t);
//!         }
//!         t
//!     }
//! }
//!
//! let either: Either<Once, u32> = Either::Left(Once);
//! assert_eq!(either.times(1, |x| x + 2), 3);
//! ```
//!
//! # Limitations
//!
//! This macro only supports traits without any associated
//! constant or associated type.
//! Generic type parameters of the trait must not be `L` or `R`.
//! The first parameter of a trait method must be `self`,
//! `&self` or `&mut self`.
//! The types of other parameters and the return type
//! must not contain `Self`.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, FnArg, Generics, Ident, ItemTrait, TraitItem, TraitItemMethod,
};

fn either_method(method: &TraitItemMethod) -> proc_macro2::TokenStream {
    let sig = &method.sig;
    let name = &sig.ident;
    if let FnArg::Receiver(_) = sig.inputs[0] {
        let args_left = sig.inputs.iter().skip(1).map(|arg| {
            if let FnArg::Typed(arg) = arg {
                &arg.pat
            } else {
                unreachable!()
            }
        });
        let args_right = args_left.clone();
        quote! {
            #sig {
                match self {
                    either::Either::Left(left) => left.#name(#(#args_left),*),
                    either::Either::Right(right) => right.#name(#(#args_right),*),
                }
            }
        }
    } else {
        panic!("The first parameter of a trait method must be `self`, `&self` or `&mut self`.")
    }
}

fn impl_item(name: &Ident, generics: &Generics) -> proc_macro2::TokenStream {
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut extended_generics = generics.clone();

    assert!(
        extended_generics.type_params().all(|param| {
            let name = param.ident.to_string();
            name != "L" && name != "R"
        }),
        "Generic type parameters must not be `L` or `R`."
    );

    extended_generics
        .params
        .push(parse_quote!(L: #name #ty_generics));
    extended_generics
        .params
        .push(parse_quote!(R: #name #ty_generics));

    quote! {
        impl #extended_generics #name #ty_generics for Either<L, R> #where_clause
    }
}

#[proc_macro_attribute]
pub fn either_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemTrait);

    let name = &input.ident;
    let items = &input.items;

    let impl_item = impl_item(&name, &input.generics);

    let impl_methods = items.iter().map(|item| match item {
        TraitItem::Method(method) => either_method(method),
        _ => panic!("The trait must be without associated constants or associated types."),
    });

    let expand = quote! {
        #input

        #impl_item
        {
            #(#impl_methods)*
        }
    };

    TokenStream::from(expand)
}
