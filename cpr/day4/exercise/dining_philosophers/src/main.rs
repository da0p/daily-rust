use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

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
        // Pick up forks...
        let _left = self.left_fork.lock().unwrap();
        let _right = self.right_fork.lock().unwrap();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create forks
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();
    // Create philosophers

    let (tx, rx) = mpsc::sync_channel(10);
    for i in 0..PHILOSOPHERS.len() {
        // Dead-lock avoidance:
        // Make half of the philosophers take left fork first, then right fork
        // The other half takes right fork first, then left fork
        let (left_fork, right_fork) = match i % 2 {
            0 => {
                (Arc::clone(&forks[i]), Arc::clone(&forks[(i + 1) % forks.len()]))
            },
            _ => {
                (Arc::clone(&forks[(i + 1) % forks.len()]), Arc::clone(&forks[i]))
            }
        };
        let txp = tx.clone();
        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts: txp,
        };
        // Make each of them think and eat 100 times
        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }
    drop(tx);
    // Output their thoughts
    for thought in rx {
        println!("{}", thought);
    }
}
