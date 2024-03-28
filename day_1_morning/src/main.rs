fn main() {
    println!("Hello, world!");

    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");
    
    println!("result: {}", interproduct(120, 100, 248));
    
    // panics because of overflow
    // println!("result: {}", interproduct16(120, 100, 248));

    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // type is determined to be u32 from earlier call so this will panic
    //takes_i8(x);
    let n = 20;
    println!("fib({n}) = {}", fib(n));

    let x = 10;
    // if expressions can return value
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);

    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");

    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    let mut i = 0;
    loop {
        i += 1;
        if i > 3 {
            break;
        }
        if i % 2 == 0 {
            println!("skipping: {}", i);
            continue;
        }
        println!("loop: {}", i);
    }

    // label loop
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    println!("searching for {}", target_value);
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                println!("found: {}", target_value);
                break 'outer;
            }
        }
    }
    println!("elements searched: {elements_searched}");


    //blocks
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");

    //shadowing
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");

    println!("gcd: {}", gcd(143, 52));

}

// implicit return
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        // The next number is found by adding up the two numbers before it
        return fib(n - 1) + fib(n - 2);
    }
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

// panics because of overflow
fn interproduct16(a: i16, b: i16, c: i16) -> i16 {
    return a * b + b * c + c * a;
}

