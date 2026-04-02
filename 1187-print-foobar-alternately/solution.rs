struct FooBar {
    n: usize,
    m: (Mutex<u8>, Condvar),
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar { n, m: (Mutex::new(0), Condvar::new()) }
    }

    fn foo<F: Fn()>(&self, print_foo: F) {
        let (lock, cvar) = &self.m;
        for _ in 0..self.n {
            let mut g = lock.lock().unwrap();
            while *g != 0 { g = cvar.wait(g).unwrap(); }
            print_foo();
            *g = 1;
            cvar.notify_all();
        }
    }

    fn bar<F: Fn()>(&self, print_bar: F) {
        let (lock, cvar) = &self.m;
        for _ in 0..self.n {
            let mut g = lock.lock().unwrap();
            while *g != 1 { g = cvar.wait(g).unwrap(); }
            print_bar();
            *g = 0;
            cvar.notify_all();
        }
    }
}
