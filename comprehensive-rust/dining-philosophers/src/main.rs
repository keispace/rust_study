use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    is_odd: bool,
    name: String,
    left_fork: Arc<Mutex<Fork>>, // 선점(mutex) 를 스레드(철학자) 공유(arc) 하는 포크
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
        //포크 선점 - 데드락 회피: 위치에따라 좌우 포크 드는 순서 변경
        if self.is_odd {
            println!("{} pick up left fork", &self.name);
            let _left_fork = self.left_fork.lock().unwrap();
            println!("{} pick up right fork", &self.name);
            let _right_fork = self.right_fork.lock().unwrap();
        } else {
            println!("{} pick up right fork", &self.name);
            let _right_fork = self.right_fork.lock().unwrap();
            println!("{} pick up left fork", &self.name);
            let _left_fork = self.left_fork.lock().unwrap();
        }
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &[
    "소크라테스",
    "플라톤",
    "아리스토텔레스",
    "탈레스",
    "피타고라스",
];

fn main() {
    // 스레드 통신 채널 생성(포크 동기 채널)
    // 에초에 사전 코드에 send가 있어서 채널 써야함.
    let (tx, rx) = mpsc::sync_channel(10);
    // Create forks
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    // Create philosophers
    for i in 0..PHILOSOPHERS.len() {
        let tx = tx.clone();
        let left_fork = forks[i].clone();
        let right_fork = forks[(i + 1) % forks.len()].clone();
        let philosopher = Philosopher {
            is_odd: i % 2 == 0,
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
        };
        // Make them think and eat
        thread::spawn(move || {
            for _ in 0..120 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }

    // Output their thoughts

    for thought in rx {
        println!("{}", thought);
    }
    println!("done! avoid deadlock!")
}
