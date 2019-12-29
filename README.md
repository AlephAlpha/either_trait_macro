Implements your own traits for [`Either`](https://crates.io/crates/either).

# Usage
When defining a trait, wrap it with the macro `either_trait`.

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

Now the macro only supports non-generic traits with only trait methods; i.e., no trait constants, no trait functions, no associated types, etc. The trait methods must also be non-generic, and their parameters and return types must not contain `Self`. Furthermore, the methods must not use patterns as parameters (e.g., `fn(&mut self, (a, b): (i32, i32));`).