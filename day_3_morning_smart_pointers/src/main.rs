#![allow(dead_code, unused, unused_variables)]

use std::mem::size_of_val;

use std::rc::Rc;

#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

// will lead to error that it has infinite size
// enum InvalidList<T> {
//     Element(T, InvalidList<T>),
//     Nil,
// }

struct Item(String);

struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn main() {
    let five = Box::new(5);
    println!("five: {}", *five + 5); // deref to get normal ops

     // recursive datatype needs to use smart pointers
    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));

    // will lead to error that it has infinite size
    //let fail = InvalidList::Element(1, InvalidList::Element(2, InvalidList::Nil));
    println!("{list:?}");


    // For example, Option<Box<T>> has the same size, as just Box<T>, because compiler 
    // uses NULL-value to discriminate variants instead of using explicit tag 
    // (“Null Pointer Optimization”):
    //      https://doc.rust-lang.org/std/option/#representation
    let just_box: Box<Item> = Box::new(Item("Just box".into()));
    let optional_box: Option<Box<Item>> =
        Some(Box::new(Item("Optional box".into())));
    let none: Option<Box<Item>> = None;

    assert_eq!(size_of_val(&just_box), size_of_val(&optional_box));
    assert_eq!(size_of_val(&just_box), size_of_val(&none));

    println!("Size of just_box: {}", size_of_val(&just_box));
    println!("Size of optional_box: {}", size_of_val(&optional_box));
    println!("Size of none: {}", size_of_val(&none));

    println!("RC stuff ----");
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    let ptr = Rc::as_ptr(&a);
    let ptr2 = Rc::as_ptr(&b);
    let strong_count_referenced = Rc::strong_count(&a);
    // Since a Weak reference does not count towards ownership, it will not prevent the value stored in the allocation from being dropped, 
    // and Weak itself makes no guarantees about the value still being present.
    let weak = Rc::downgrade(&a);
    let weak_count_referenced = Rc::weak_count(&a);
    assert_eq!(ptr, ptr2); // both should point into same address
    println!("a: {a}");
    println!("b: {b}");
    println!("weak (upgrade-is_some()): {:?}", weak.upgrade().is_some());
    println!("a ptr: {ptr:?}");
    println!("b ptr: {ptr2:?}");
    println!("strong count referenced: {strong_count_referenced:?}");
    println!("weak count referenced: {weak_count_referenced:?}");

    // drop RC
    drop(a);
    println!("after drop(a) = b: {b}");
    println!("after drop(a) = weak (upgrade-is_some()): {:?}", weak.upgrade().is_some());
    assert!(weak.upgrade().is_some());

    drop(b);
    println!("after drop(b) = c: {c}");
    println!("after drop(b) = weak (upgrade-is_some()): {:?}", weak.upgrade().is_some());
    assert!(weak.upgrade().is_some());

    drop(c);
    // now the weak should be none
    println!("after drop(c) = weak (upgrade-is_some()): {:?}", weak.upgrade().is_some());
    assert!(weak.upgrade().is_none());


    // list contains elements implementing trait N need to be wrapped in Box
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }

    // "Types that implement a given trait may be of different sizes."
    // you cant have vec of dyn Trait since the elements might be different size, that is why Box is needed
    // in this case sizeOf Dog > sizeOf Cat
    println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
    println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());


}