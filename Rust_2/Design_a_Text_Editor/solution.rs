struct TextEditor {
    left: Vec<u8>,
    right: Vec<u8>,
}

impl TextEditor {
    fn new() -> Self { TextEditor { left: vec![], right: vec![] } }

    fn add_text(&mut self, text: String) {
        self.left.extend_from_slice(text.as_bytes());
    }

    fn delete_text(&mut self, k: i32) -> i32 {
        let k = k.min(self.left.len() as i32) as usize;
        self.left.truncate(self.left.len() - k);
        k as i32
    }

    fn cursor_left(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.left.pop() { self.right.push(c); } else { break; }
        }
        let len = self.left.len().min(10);
        String::from_utf8(self.left[self.left.len()-len..].to_vec()).unwrap()
    }

    fn cursor_right(&mut self, k: i32) -> String {
        for _ in 0..k {
            if let Some(c) = self.right.pop() { self.left.push(c); } else { break; }
        }
        let len = self.left.len().min(10);
        String::from_utf8(self.left[self.left.len()-len..].to_vec()).unwrap()
    }
}