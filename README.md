Implements your own traits for [`Either`](https://crates.io/crates/either). If your trait is implemented for both type `A` and `B`, then it is automatically implemented for `Either<A, B>`.

# Usage
When defining a trait, add the attribute `#[either_trait]`.

# Example
```rust
use either::Either;
use either_trait_macro::either_trait;

#[either_trait]
/// An example trait.
pub trait Example {
    /// Foo.
    fn foo(&self, x: i32) -> i32;
}

struct A;

struct B(i32);

impl Example for A {
    fn foo(&self, x: i32) -> i32 {
        x
    }
}

impl Example for B {
    fn foo(&self, x: i32) -> i32 {
        self.0 + x
    }
}

let mut either: Either<A, B> = Either::Left(A);
assert_eq!(either.foo(2), 2);

let mut either: Either<A, B> = Either::Right(B(2));
assert_eq!(either.foo(2), 4);
```

# Limits

This macro only supports non-generic traits without any associated constant or associated type. The first parameter of a trait method must be `self`, `&self` or `&mut self`. The types of other parameters and the return type must not contain `Self`.
