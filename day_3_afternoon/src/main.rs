use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            ..Node::default()
        }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

fn main() {
    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(5));
    let subtree = Node::new(10);

    // interior mutability with RefCell
    subtree.borrow_mut().children.push(Node::new(11));
    subtree.borrow_mut().children.push(Node::new(12));

    let cloned = subtree.borrow().children.clone();

    // modify all children value
    cloned.iter().for_each(|x| x.borrow_mut().value *= 2);

    let newnode = Node::new(66);
    newnode.borrow_mut().children.extend(cloned);

    root.borrow_mut().children.push(subtree);
    root.borrow_mut().children.push(newnode);

    println!("graph: {root:#?}");
    println!("graph sum: {}", root.borrow().sum());
}
