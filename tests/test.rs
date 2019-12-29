use either::Either;
use either_trait_macro::either_trait;

#[either_trait]
/// An example trait.
trait Example {
    /// Foo.
    fn foo(&self, x: i32) -> i32;

    /// Bar.
    fn bar(&mut self, z: (i32, i32));
}

struct A;

struct B(i32);

impl Example for A {
    fn foo(&self, x: i32) -> i32 {
        x
    }

    fn bar(&mut self, (x, y): (i32, i32)) {
        println!("{}, {}", x, y);
    }
}

impl Example for B {
    fn foo(&self, x: i32) -> i32 {
        self.0 + x
    }

    fn bar(&mut self, (x, y): (i32, i32)) {
        self.0 += x + y;
        println!("{}, {}", x, y);
    }
}

#[test]
fn test1() {
    let mut either: Either<A, B> = Either::Left(A);
    assert_eq!(either.foo(2), 2);
    either.bar((3, 4));
    assert_eq!(either.foo(0), 0);
}

#[test]
fn test2() {
    let mut either: Either<A, B> = Either::Right(B(2));
    assert_eq!(either.foo(2), 4);
    either.bar((3, 4));
    assert_eq!(either.foo(0), 9);
}
