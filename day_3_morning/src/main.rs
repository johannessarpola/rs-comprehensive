#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}
#[allow(unused)]
fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}"); // as the p1 is copied to p2 this does not cause issue
    println!("p2: {p2:?}");

    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a); // explicitly drop a, otherwise it would be dropped on exit
    println!("Exiting main");

}