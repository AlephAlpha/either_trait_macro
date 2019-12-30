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
//! /// An example trait.
//! pub trait Example {
//!     /// Foo.
//!     fn foo(&self, x: i32) -> i32;
//! }
//!
//! struct A;
//!
//! struct B(i32);
//!
//! impl Example for A {
//!     fn foo(&self, x: i32) -> i32 {
//!         x
//!     }
//! }
//!
//! impl Example for B {
//!     fn foo(&self, x: i32) -> i32 {
//!         self.0 + x
//!     }
//! }
//!
//! let mut either: Either<A, B> = Either::Left(A);
//! assert_eq!(either.foo(2), 2);
//!
//! let mut either: Either<A, B> = Either::Right(B(2));
//! assert_eq!(either.foo(2), 4);
//! ```
//!
//! # Limits
//!
//! This macro only supports non-generic traits without any
//! associated constant or associated type.
//! The first parameter of a trait method must be `self`,
//! `&self` or `&mut self`.
//! The types of other parameters and the return type
//! must not contain `Self`.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemTrait, TraitItem, TraitItemMethod};

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
        panic!()
    }
}

#[proc_macro_attribute]
pub fn either_trait(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemTrait);

    let name = &input.ident;
    let items = &input.items;

    let impl_methods = items.iter().map(|item| match item {
        TraitItem::Method(method) => either_method(method),
        _ => panic!(),
    });

    let expand = quote! {
        #input

        impl<L, R> #name for Either<L, R>
        where
            L: #name,
            R: #name,
        {
            #(#impl_methods)*
        }
    };

    TokenStream::from(expand)
}
