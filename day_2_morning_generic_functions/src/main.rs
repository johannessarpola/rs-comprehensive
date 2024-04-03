/// Pick `even` or `odd` depending on the value of `n`.
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn coords(&self) -> (&T, &U) {
        (&self.x, &self.y)
    }

    // fn set_x(&mut self, x: T)
}

// custom methods for certain combination of types
impl Point<f64, f64> {
    fn does_float(&self) -> bool {
        return true;
    }
}

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

impl From<f64> for Foo {
    fn from(from: f64) -> Foo {
        Foo(format!("Converted from f64: {from}"))
    }
}

// trait bounds with :Clone
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// where clause makes it a bit more readable
fn another_duplicate<T>(a: T) -> (T, T)
where
    // these can be combined with +
    T: Clone + ToString,
{
    (a.clone(), a.clone())
}

#[derive(Clone, Debug)]
struct Clonable(String);

#[allow(dead_code)]
struct NotClonable(i8);

trait ToString {
    fn formatted(&self) -> String;
}

impl ToString for Clonable {
    fn formatted(&self) -> String {
        format!("{self:?}")
    }
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

// has derive(debug) bound
fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a tuple: {:?}", pick(28, ("dog", 1), ("cat", 2)));

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("doesFloat {}", float.does_float());

    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
    println!("coords {:?}", float.coords());

    let from_int = Foo::from(123);
    let from_float = Foo::from(123.123);
    let from_bool = Foo::from(true);
    println!("{from_int:?}, {from_bool:?}, {from_float:?}");

    let foo = String::from("foo");
    let pair = duplicate(foo);
    let pair2 = another_duplicate(Clonable(String::from("abc")));
    // does not satisfy trait bound
    // let pair3 = duplicate(NotClonable(1));
    println!(
        "{:?}, {},{}",
        pair,
        pair2.0.formatted(),
        pair2.1.formatted()
    );

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("debuggable: {debuggable:?}");
}
