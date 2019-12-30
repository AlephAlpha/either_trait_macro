Implements your own traits for [`Either`](https://crates.io/crates/either). If your trait is implemented for both type `A` and `B`, then it is automatically implemented for `Either<A, B>`.

# Usage
When defining a trait, add the attribute `#[either_trait]`.

# Example
```rust
use either::Either;
use either_trait_macro::either_trait;

#[either_trait]
/// Apply a function `n` times.
trait Apply {
    fn times<T, F>(&self, t: T, f: F) -> T
    where
        F: Fn(T) -> T;
}

struct Once;

impl Apply for Once {
    fn times<T, F>(&self, t: T, f: F) -> T
    where
        F: Fn(T) -> T,
    {
        f(t)
    }
}

impl Apply for u32 {
    fn times<T, F>(&self, t: T, f: F) -> T
    where
        F: Fn(T) -> T,
    {
        let mut t = t;
        for _ in 0..*self {
            t = f(t);
        }
        t
    }
}

let either: Either<Once, u32> = Either::Left(Once);
assert_eq!(either.times(1, |x| x + 2), 3);
```

# Limitations

This macro only supports traits without any associated constant or associated type. Generic type parameters of the trait must not be `L` or `R`. The first parameter of a trait method must be `self`, `&self` or `&mut self`. The types of other parameters and the return type must not contain `Self`.
