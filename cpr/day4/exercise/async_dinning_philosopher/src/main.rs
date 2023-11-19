use std::sync::Arc;
use tokio::time;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name)).await
            .unwrap();
    }

    async fn eat(&self) {
        // Pick up forks...
        let _left = self.left_fork.lock().await;
        time::sleep(time::Duration::from_millis(1)).await;
        let _right = self.right_fork.lock().await;
        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

#[tokio::main]
async fn main() {
    // Create forks
    let forks = (0..PHILOSOPHERS.len()).map(|_| {
        Arc::new(Mutex::new(Fork))
    }).collect::<Vec<_>>();

    // Create philosophers
    let (tx, mut rx) = mpsc::channel(10);
    for i in 0..PHILOSOPHERS.len() {
        let (left_fork, right_fork) = match i % 2 {
            0 =>  (Arc::clone(&forks[i]), Arc::clone(&forks[(i + 1) % forks.len()])),
            _ =>  (Arc::clone(&forks[(i + 1) % forks.len()]), Arc::clone(&forks[i]))
        };
        let thoughts = tx.clone();
        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts,
        };

        // Make them think and eat
        tokio::spawn(async move {
            for _ in 0..100 {
                philosopher.think().await;
                philosopher.eat().await;
            }
        });
    }
    // Output their thoughts
    drop(tx);
    while let Some(thought) = rx.recv().await {
        println!("{thought}");
    }
}