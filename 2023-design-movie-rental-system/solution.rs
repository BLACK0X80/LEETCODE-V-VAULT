use std::collections::{BTreeSet, HashMap};

struct MovieRentingSystem {
    black1: HashMap<i32, BTreeSet<(i32, i32)>>,
    black2: BTreeSet<(i32, i32, i32)>,
    black3: HashMap<(i32, i32), i32>,
}

impl MovieRentingSystem {
    fn new(_n: i32, black4: Vec<Vec<i32>>) -> Self {
        let mut black1 = HashMap::new();
        let mut black3 = HashMap::new();
        for b in black4 {
            let (s, m, p) = (b[0], b[1], b[2]);
            black1.entry(m).or_insert(BTreeSet::new()).insert((p, s));
            black3.insert((s, m), p);
        }
        Self { black1, black2: BTreeSet::new(), black3 }
    }

    fn search(&self, black5: i32) -> Vec<i32> {
        self.black1.get(&black5).map_or(vec![], |s| s.iter().take(5).map(|&(_, shop)| shop).collect())
    }

    fn rent(&mut self, black6: i32, black7: i32) {
        let black8 = self.black3[&(black6, black7)];
        self.black1.get_mut(&black7).unwrap().remove(&(black8, black6));
        self.black2.insert((black8, black6, black7));
    }

    fn drop(&mut self, black6: i32, black7: i32) {
        let black8 = self.black3[&(black6, black7)];
        self.black2.remove(&(black8, black6, black7));
        self.black1.get_mut(&black7).unwrap().insert((black8, black6));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.black2.iter().take(5).map(|&(_, s, m)| vec![s, m]).collect()
    }
}
