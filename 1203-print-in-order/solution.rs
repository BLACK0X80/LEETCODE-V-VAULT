struct Foo {
    m: Arc<(Mutex<u8>, Condvar)>,
}

impl Foo {
    fn new() -> Self {
        Foo {
            m: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }

    fn first(&self, print_first: impl FnOnce()) {
        let (lock, cvar) = &*self.m;
        print_first();
        *lock.lock().unwrap() = 1;
        cvar.notify_all();
    }

    fn second(&self, print_second: impl FnOnce()) {
        let (lock, cvar) = &*self.m;
        let mut g = lock.lock().unwrap();
        while *g < 1 { g = cvar.wait(g).unwrap(); }
        print_second();
        *g = 2;
        cvar.notify_all();
    }

    fn third(&self, print_third: impl FnOnce()) {
        let (lock, cvar) = &*self.m;
        let mut g = lock.lock().unwrap();
        while *g < 2 { g = cvar.wait(g).unwrap(); }
        print_third();
    }
}
