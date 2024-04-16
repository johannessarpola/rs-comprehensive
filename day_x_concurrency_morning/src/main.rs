use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex}; // Multi-Producer, Single-Consumer

// Normal thread cannot borrow from context: 
// fn foo() {
//     let s = String::from("Hello");
//     thread::spawn(|| {
//         println!("Length: {}", s.len());
//     });
// }

// but thread::scope can 
fn scoped_foo() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("scoped->Length: {}", s.len());
        });
    });
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
        // gets automatically wrapped into result
        "done"
    });

    // handle returns result of the thread
    let rs = handle.join();

    match rs {
        Ok(v) => println!("{}", v),
        _ => panic!("panik"),
    }

    let phandle = thread::spawn(|| panic!("panik panik"));

    match phandle.join() {
        Err(e) if e.is::<&str>() => {
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("Thread panicked with error: {}", s);
            }
        }
        Err(_) => println!("Thread panicked with an unknown error"),
        _ => panic!("panik"),
    }

    let numbs = vec![101, 202];
    let nhandle = thread::spawn(move || {
        println!("number is {:?}", numbs);
        let mut ns = 0;
        for n in numbs {
            ns += n;
        }
        ns
    });

    // cant use since numbs is moved to thread
    //    let sz = numbs.capacity();

    match nhandle.join() {
        Ok(sum) => println!("sum: {}", sum),
        _ => panic!("panik"),
    }

    scoped_foo();

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    // channels
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());

    // unbounded channels 
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));
    for msg in rx.iter() {
        println!("Main(unbounded): got {msg}");
    }

    // bounded channel
    let (tx, rx) = mpsc::sync_channel(3);

    // this will only send 3 messages maximun to the queue
    // after it will wait untill someone reads the message before sending more
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            // it will block if the bound is full
            tx.send(format!("Message (bounded) {i}")).unwrap();
            println!("{thread_id:?}: bounded sent Message {i}");
        }
        println!("{thread_id:?}: bounded done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main(bounded): got {msg}");
    }

    // arc with multithreaded can be cloned
    // arc === Atomic Reference Counted (Thread safe version of RC)
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");

    // mutex
    let v = Mutex::new(vec![10, 20, 30]);
    println!("v: {:?}", v.lock().unwrap());

    {
        let mut guard = v.lock().unwrap();
        guard.push(40);
    }

    println!("v: {:?}", v.lock().unwrap());


    // arc + mutex
    let arc_mutex_vec = Arc::new(Mutex::new(vec![10, 20, 30]));

    let cloned = Arc::clone(&arc_mutex_vec);
    let handle = thread::spawn(move || {
        let mut cloned = cloned.lock().unwrap();
        cloned.push(10);
    });

    {
        let mut accessed_vec = arc_mutex_vec.lock().unwrap();
        accessed_vec.push(1000);
    }

    handle.join().unwrap();

    println!("v: {arc_mutex_vec:?}");


}
