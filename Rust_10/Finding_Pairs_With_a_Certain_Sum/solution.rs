struct FindSumPairs { black_n1: Vec<i32>, black_n2: Vec<i32>, black_cnt: std::collections::HashMap<i32, i32> }
impl FindSumPairs {
    fn new(black_nums1: Vec<i32>, black_nums2: Vec<i32>) -> Self { let mut black_cnt = std::collections::HashMap::new(); for &black_x in &black_nums2 { *black_cnt.entry(black_x).or_insert(0) += 1; } Self { black_n1: black_nums1, black_n2: black_nums2, black_cnt } }
    fn add(&mut self, black_idx: i32, black_val: i32) { let black_old = self.black_n2[black_idx as usize]; *self.black_cnt.get_mut(&black_old).unwrap() -= 1; self.black_n2[black_idx as usize] += black_val; *self.black_cnt.entry(self.black_n2[black_idx as usize]).or_insert(0) += 1; }
    fn count(&self, black_tot: i32) -> i32 { self.black_n1.iter().map(|&black_x| self.black_cnt.get(&(black_tot - black_x)).unwrap_or(&0)).sum() }
}