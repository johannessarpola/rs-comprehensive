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

    // mutable reference
    {
        let a = 'A';
        let b = 'B';
        let mut r: &char = &a;
        println!("r: {}", *r);
        r = &b;
        // *r = 'x'; // will err as even though the pointer is mutable the pointed value is not
        println!("r: {}", *r);
    }
    ();

    // exclusive references (mutates value stored in the pointed location)
    {
        let mut point = (1, 2);
        // exclusive reference
        let x_coord = &mut point.0;
        // pointed value can then be changed
        *x_coord = 20;
        println!("point: {point:?}");
    }
    ();

    // slices
    {
        #[allow(unused_mut)]
        let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
        println!("a: {a:?}");

        let s: &[i32] = &a[2..4];
        
        // a[3] = 99; // you cannot assing a value here as it is already borrowed into slice for memory safety

        println!("s: {s:?}");
    }
    ();

    // strings
    {
        let s1: &str = "World";
        println!("s1: {s1}");
    
        let mut s2: String = String::from("Hello ");
        println!("s2: {s2}");
        s2.push_str(s1);
        println!("s2: {s2}");
    
        let s3: &str = &s2[6..];
        println!("s3: {s3}");
    }();

}

fn print_tuple(tuple: (i32, i32)) {
    // pattern matchin destruct
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}
