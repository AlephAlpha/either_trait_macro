//! Implements your own traits for [`Either`](https://crates.io/crates/either).
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
//! Now the macro only supports non-generic traits with only trait methods;
//! i.e., no trait constants, no trait functions, no associated types, etc.
//! The trait methods must also be non-generic, and their parameters
//! and return types must not contain `Self`. Furthermore, the methods must
//! not use patterns as parameters (e.g., `fn(&mut self, (a, b): (i32, i32));`).

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
