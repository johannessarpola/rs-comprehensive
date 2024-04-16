#![allow(dead_code, unused, unused_variables)]

use std::sync::{mpsc, Arc, Mutex};
use std::{thread, vec};
use std::time::Duration;

struct Fork(String);
struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is trying to eat ...", &self.name);
        let _left = self.left_fork.lock().unwrap();
        let _right = self.right_fork.lock().unwrap();

        println!("{} is eating (first: {} second: {}) ...", &self.name, _left.0, _right.0);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Philosopher 0", "Philosopher 1", "Philosopher 2", "Philosopher 3", "Philosopher 4"];

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);
    

    let forks = (0..PHILOSOPHERS.len())
        .map(|iter| Arc::new(Mutex::new(Fork(format!("Fork {}", iter)))))
        .collect::<Vec<_>>();

    let mut philos = vec![];

    for i in 0..forks.len() {
        let tx = tx.clone();
        /*
        Philosopher 0: Left Fork 0, Right Fork 1
        Philosopher 1: Left Fork 1, Right Fork 2
        Philosopher 2: Left Fork 2, Right Fork 3
        Philosopher 3: Left Fork 3, Right Fork 4
        Philosopher 4: Left Fork 4, Right Fork 0 
        
        */

        // just take a fork with iterator
        let mut lf = Arc::clone(&forks[i]);

        // take the next fork and around with modulo
        let mut rf = Arc::clone(&forks[(i + 1) % forks.len()]);
        /*
        to break the symmetry the last one picks the right first rather than left fork

        By breaking the symmetry, the last philosopher picks up the right fork first instead of the left one 
            -> there's always one philosopher who picks up the forks in a different order preventing a potential deadlock.

        Without the swap, each philosopher would pick up the left fork first and then the right fork. 
        This could lead to a potential deadlock situation if all philosophers simultaneously pick up 
        their left fork, waiting indefinitely for the right fork. By breaking the symmetry, we ensure 
        that at least one philosopher behaves differently, which can prevent deadlock scenarios.
        */
        if i == forks.len() - 1 {
            std::mem::swap(&mut lf, &mut rf);
        }

        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork: lf,
            right_fork: rf,
        };

        philos.push(philosopher);

    }


    let mut handles = vec![];
    while let Some(philo) = philos.pop() {
        
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                philo.eat();
                philo.think();
            }
        });
        handles.push(handle);
    }

    drop(tx);
    for thought in rx {
        println!("{thought}");
    }

    while let Some(h) = handles.pop() {
        h.join();
    }
}
