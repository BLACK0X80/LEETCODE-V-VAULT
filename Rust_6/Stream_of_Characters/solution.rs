struct StreamChecker {
    children: Vec<[i32; 26]>,
    is_end: Vec<bool>,
    stream: Vec<usize>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut s = StreamChecker {
            children: vec![[-1i32; 26]],
            is_end: vec![false],
            stream: vec![],
        };
        for w in &words {
            let mut node = 0usize;
            for &b in w.as_bytes().iter().rev() {
                let c = (b - b'a') as usize;
                if s.children[node][c] == -1 {
                    s.children[node][c] = s.children.len() as i32;
                    s.children.push([-1; 26]);
                    s.is_end.push(false);
                }
                node = s.children[node][c] as usize;
            }
            s.is_end[node] = true;
        }
        s
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push((letter as u8 - b'a') as usize);
        let mut node = 0usize;
        for &c in self.stream.iter().rev() {
            let n = self.children[node][c];
            if n == -1 { break; }
            node = n as usize;
            if self.is_end[node] { return true; }
        }
        false
    }
}