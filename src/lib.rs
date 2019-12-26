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
//! either_trait! {
//!     /// An example trait.
//!     pub trait Example {
//!         /// Foo.
//!         fn foo(&self, x: i32) -> i32;
//!     }
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

#[macro_export]
#[doc(hidden)]
macro_rules! _either {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            Either::Left($pattern) => $result,
            Either::Right($pattern) => $result,
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _either_method {
    (fn $name:ident(self $(, $par:ident : $par_ty: ty)*) $(-> $ret: ty)?) => {
        fn $name(self $(, $par: $par_ty)*) $(-> $ret)? {
            $crate::_either!(self, inner => inner.$name($($par),*))
        }
    };
    (fn $name:ident(&self $(, $par:ident : $par_ty: ty)*) $(-> $ret: ty)?) => {
        fn $name(&self $(, $par: $par_ty)*) $(-> $ret)? {
            $crate::_either!(self, inner => inner.$name($($par),*))
        }
    };
    (fn $name:ident(&mut self $(, $par:ident : $par_ty: ty)*) $(-> $ret: ty)?) => {
        fn $name(&mut self $(, $par: $par_ty)*) $(-> $ret)? {
            $crate::_either!(self, inner => inner.$name($($par),*))
        }
    };
}

/// A macro for defining a trait and implementing it for
/// [`Either`](https://crates.io/crates/either).
#[macro_export]
macro_rules! either_trait {
    {
        $(#[$outer:meta])*
        pub trait $trait:ident {
            $(
                $(#[$inner:ident $($inner_args:tt)*])*
                fn $name:ident($($args:tt)*) $(-> $ret: ty)?
            );*;
        }
    } => {
        $(#[$outer])*
        pub trait $trait {
            $(
                $(#[$inner $($inner_args)*])*
                fn $name($($args)*) $(-> $ret)?
            );*;
        }

        impl<L, R> $trait for Either<L, R>
        where
            L: $trait,
            R: $trait,
        {
            $($crate::_either_method!{fn $name($($args)*) $(-> $ret)?})*
        }
    };
    {
        $(#[$outer:meta])*
        trait $trait:ident {
            $(
                $(#[$inner:ident $($inner_args:tt)*])*
                fn $name:ident($($args:tt)*) $(-> $ret: ty)?
            );*;
        }
    } => {
        $(#[$outer])*
        trait $trait {
            $(
                $(#[$inner $($inner_args)*])*
                fn $name($($args)*) $(-> $ret)?
            );*;
        }

        impl<L, R> $trait for Either<L, R>
        where
            L: $trait,
            R: $trait,
        {
            $($crate::_either_method!{fn $name($($args)*) $(-> $ret)?})*
        }
    };
}
