use std::sync::{Mutex, Arc};

struct DiningPhilosophers {
    forks: [Mutex<()>; 5],
}

impl DiningPhilosophers {
    fn new() -> Self {
        DiningPhilosophers {
            forks: [Mutex::new(()), Mutex::new(()), Mutex::new(()),
                    Mutex::new(()), Mutex::new(())],
        }
    }

    fn wants_to_eat<F1,F2,F3,F4,F5>(
        &self, philosopher: i32,
        pick_left_fork: F1, pick_right_fork: F2,
        eat: F3,
        put_left_fork: F4, put_right_fork: F5,
    ) where F1:FnOnce(),F2:FnOnce(),F3:FnOnce(),F4:FnOnce(),F5:FnOnce() {
        let left  = philosopher as usize;
        let right = (philosopher + 1) as usize % 5;

        let (first, second) = if philosopher % 2 == 0 {
            (left, right)
        } else {
            (right, left)
        };

        let _a = self.forks[first].lock().unwrap();
        let _b = self.forks[second].lock().unwrap();

        if philosopher % 2 == 0 {
            pick_left_fork(); pick_right_fork(); eat(); put_left_fork(); put_right_fork();
        } else {
            pick_right_fork(); pick_left_fork(); eat(); put_right_fork(); put_left_fork();
        }
    }
}
