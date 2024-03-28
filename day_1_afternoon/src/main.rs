fn main() {
    // inits array to be full of 22's
    let mut a: [i8; 10] = [22; 10];
    let mut b: [i8; 5] = [1, 2, 3, 4, 5];
    a[5] = 0;
    b[1] = 10;
    //debug print
    println!("a: {a:?}");
    //pretty print format
    println!("b: {b:#?}");

    // tuple
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
    println!("t: {:?}", t);

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    // you can use for ... in with arrays
    for prime in primes {
        // goes from 2 to (prime -1), if it is divisable it is not a prime number
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    print_tuple((12, 33));
}

fn print_tuple(tuple: (i32, i32)) {
    // pattern matchin destruct
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}
