use std::sync::{Arc, Mutex};

static BANNER: &str = "I'm a global string";
static mut MUT_BANNER: &str = "Mutable global string";

// these are inlined
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn main() {
    println!("Hello, world!");

    //named structs
    {
        struct Person {
            name: String,
            age: u8,
        }

        fn describe(person: &Person) {
            println!("{} is {} years old", person.name, person.age);
        }
        let mut peter = Person {
            name: String::from("Peter"),
            age: 27,
        };
        describe(&peter);

        peter.age = 28;
        describe(&peter);

        let name = String::from("Avery");
        let age = 39;
        let avery = Person { name, age };
        describe(&avery);

        let jackie = Person {
            name: String::from("Jackie"),
            ..avery
        };
        describe(&jackie);
        println!("{}", BANNER);
    }
    ();

    //tuple struct
    {
        struct Point(i32, i32);
        let p = Point(17, 23);
        println!("({}, {})", p.0, p.1);

        //usually used as wrapper on units
        struct PoundsOfForce(f64);
        struct Newtons(f64);

        println!("Newtons: {}", Newtons(100.).0);
        println!("PoundsOfForce: {}", PoundsOfForce(10.).0);
    }
    ();

    //enums
    #[allow(dead_code)]
    {
        #[derive(Debug)]
        enum Direction {
            Left,
            Right,
        }

        #[derive(Debug)]
        enum PlayerMove {
            Pass,                        // Simple variant
            Run(Direction),              // Tuple variant
            Teleport { x: u32, y: u32 }, // Struct variant
        }

        let m: PlayerMove = PlayerMove::Run(Direction::Left);
        let m2: PlayerMove = PlayerMove::Teleport { x: 10, y: 20 };
        let m3: PlayerMove = PlayerMove::Pass;
        println!("On 1 turn: {:?}", m);
        println!("On 2 turn: {:?}", m2);
        println!("On 3 turn: {:?}", m3);
    }
    ();

    // change static global string
    {
        // use unsafe
        unsafe {
            println!("{}", MUT_BANNER);
            MUT_BANNER = "Changed global string";
            println!("{}", MUT_BANNER);
        }

        // with mutex
        // synchronization mutex
        let global_var_mutex = Arc::new(Mutex::new(42));

        // clone to share the mutex
        let global_var_mutex_clone = global_var_mutex.clone();

        std::thread::spawn(move || {
            let mut value = global_var_mutex_clone.lock().unwrap();
            println!("value in thread: {}", value);
            *value = 100;
        })
        .join()
        .expect("Failed to join thread");

        println!(
            "value updated in thread: {}",
            global_var_mutex.lock().unwrap()
        )
    }
    ();

    {
        let text = "Hello";
        let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
        for (idx, &b) in text.as_bytes().iter().enumerate() {
            digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
        }
        println!("digest: {:?}", digest);
    }
    ();

    // type aliases
    #[allow(dead_code)]
    {
        enum CarryableConcreteItem {
            Left,
            Right,
        }
        // siilar to typedef in C
        type Item = CarryableConcreteItem;
    }
    ();
}
