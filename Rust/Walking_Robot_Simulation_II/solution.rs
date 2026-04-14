struct Robot {
    black1: i32,
    black2: i32,
    black3: i32,
    black4: i32,
    black5: bool,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Self {
            black1: width,
            black2: height,
            black3: 0,
            black4: (width + height - 2) * 2,
            black5: false,
        }
    }

    fn step(&mut self, num: i32) {
        self.black5 = true;
        self.black3 = (self.black3 + num) % self.black4;
    }

    fn get_pos(&self) -> Vec<i32> {
        let (w, h) = (self.black1, self.black2);
        let s = self.black3;
        if s < w {
            vec![s, 0]
        } else if s < w + h - 1 {
            vec![w - 1, s - (w - 1)]
        } else if s < 2 * w + h - 2 {
            vec![w - 1 - (s - (w + h - 2)), h - 1]
        } else {
            vec![0, h - 1 - (s - (2 * w + h - 3))]
        }
    }

    fn get_dir(&self) -> String {
        let (w, h) = (self.black1, self.black2);
        let s = self.black3;
        if !self.black5 { return "East".to_string(); }
        if s > 0 && s < w {
            "East".to_string()
        } else if s >= w && s < w + h - 1 {
            "North".to_string()
        } else if s >= w + h - 1 && s < 2 * w + h - 2 {
            "West".to_string()
        } else if s >= 2 * w + h - 2 && s < self.black4 {
            "South".to_string()
        } else {
            "South".to_string()
        }
    }
}