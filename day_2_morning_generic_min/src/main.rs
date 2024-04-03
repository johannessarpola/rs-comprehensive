use std::cmp::Ordering;

fn min<T: Ord>(l: T, r: T) -> T {
    // https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
    // https://doc.rust-lang.org/stable/std/cmp/enum.Ordering.html
    match l.cmp(&r) {
        Ordering::Less | Ordering::Equal => l,
        Ordering::Greater => r,
    }
}

fn main() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);

    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');

    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}