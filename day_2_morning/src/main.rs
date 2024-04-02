use std::env;

struct Foo {
    x: (u32, u32),
    y: u32,
}
enum AResult {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> AResult {
    if n % 2 == 0 {
        AResult::Ok(n / 2)
    } else {
        AResult::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn pattern_struct(foo: Foo){
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

fn sleep_for(secs: f32) {
    let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
        dur
    } else {
        std::time::Duration::from_millis(500)
    };
    std::thread::sleep(dur);
    println!("slept for {:?}", dur);
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    /*  Awkward version:    
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("got None"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Err(String::from("got empty string"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("not a hex digit"))
    } 
    */

    /*
     * readable version
     */
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    return Ok(digit);
}

#[rustfmt::skip]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("The first argument is: {}", args[1]);
    } else {
        println!("No arguments provided.");
    }
    let input = match args.get(1) {
        Some(arg) => arg.clone(),
        None => String::from("x"),
    };

    match input.chars().next().unwrap() {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"), // default 
    }

    let foo = Foo { x: (1, 2), y: 3 };
    pattern_struct(foo);
    let foo = Foo { x: (3, 3), y: 2 };
    pattern_struct(foo);
    let foo = Foo { x: (6, 6), y: 7 };
    pattern_struct(foo);

    let n = 100;
    match divide_in_two(n) {
        AResult::Ok(half) => println!("{n} divided in two is {half}"),
        AResult::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
    let n = 77;
    match divide_in_two(n) {
        AResult::Ok(half) => println!("{n} divided in two is {half}"),
        AResult::Err(msg) => println!("sorry, an error happened: {msg}"),
    }

    
    sleep_for(-10.0);
    sleep_for(0.8);

    let s = String::from("invalid");
    let r = hex_or_die_trying(Some(s));

    match r {
        Ok(v) => println!("decimal: {}", v),
        Err(e) => println!("err {}", e),
    };
    let s = String::from("A");
    let r = hex_or_die_trying(Some(s));
    match r {
        Ok(v) => println!("decimal: {}", v),
        Err(e) => println!("err {}", e),
    };
}
