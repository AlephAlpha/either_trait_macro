use either::Either;
use either_trait_macro::either_trait;

#[either_trait]
trait Example<T> {
    fn foo(&mut self, t: T);

    fn bar<F>(&self, f: F, t: &T) -> T
    where
        F: FnOnce(&T) -> T;
}

struct A;

struct B<T> {
    t: Option<T>,
}

impl<T> Example<T> for A {
    fn foo(&mut self, _t: T) {}

    fn bar<F>(&self, f: F, t: &T) -> T
    where
        F: FnOnce(&T) -> T,
    {
        f(t)
    }
}

impl<T> Example<T> for B<T> {
    fn foo(&mut self, t: T) {
        self.t = Some(t);
    }

    fn bar<F>(&self, f: F, t: &T) -> T
    where
        F: FnOnce(&T) -> T,
    {
        f(self.t.as_ref().unwrap_or(t))
    }
}

#[test]
fn test_left() {
    let mut either: Either<A, B<u32>> = Either::Left(A);
    either.foo(2);
    assert_eq!(either.bar(|x| x + 2, &1), 3);
}

#[test]
fn test_right() {
    let mut either: Either<A, B<u32>> = Either::Right(B { t: None });
    either.foo(2);
    assert_eq!(either.bar(|x| x + 2, &1), 4);
}
