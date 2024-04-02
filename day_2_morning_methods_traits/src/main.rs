#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

// supertrait
trait Animal {
    fn leg_count(&self) -> u32;
}

// pet pulls in the 'interface' from supertrait 'Animal'
trait Pet: Animal {
    fn talk(&self) -> String;
    fn age(&self) -> i8;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}
#[derive(Debug, Clone, Default)]

struct Dog {
    name: String,
    age: i8,
}

#[derive(Debug, Clone, Default)]
struct Cat {
    name: String,
    age: i8,
}

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }

    fn age(&self) -> i8 {
        self.age * 7 // age in dog years
    }
}

impl Animal for Cat {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("Meow! My name is {}!", self.name)
    }

    fn age(&self) -> i8 {
        self.age * 5 // age in cat years
    }
}


//associated types
#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
    type Output; // Many standard library traits have associated types, including arithmetic operators and Iterator.
    fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
    type Output = MetersSquared; // output -> MetersSquared
    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}


fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish(); // self: takes ownership of the object and moves it away from the caller.
                   // The method becomes the owner of the object. The object will be dropped (deallocated)
                   // when the method returns, unless its ownership is explicitly transmitted.
                   // Complete ownership does not automatically mean mutability.

    //race.add_lap(42); // won't work as the ownership moved in .finish()

    let fido = Dog {
        name: String::from("Fido"),
        age: 5,
    };
    fido.greet();
    println!("age: {}", fido.age());
    println!("debug: {:?}", fido);


    let katti = Cat {
        name: String::from("Katti"),
        age: 5,
    };
    katti.greet();
    println!("age: {}", katti.age());
    println!("debug: {:?}", katti); // derive.Debug
    println!("cloned: {:?}", katti.clone()); // derive.Clone
    println!("default: {:?}", Cat::default()); // derive.Default
    

    println!("{:?}", Meters(10).multiply(&Meters(20)));

}
