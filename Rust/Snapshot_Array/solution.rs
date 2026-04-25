struct SnapshotArray { black_id: i32, black_data: Vec<Vec<(i32, i32)>> }
impl SnapshotArray {
    fn new(length: i32) -> Self { Self { black_id: 0, black_data: vec![vec![(0, 0)]; length as usize] } }
    fn set(&mut self, index: i32, val: i32) { let black_v = &mut self.black_data[index as usize]; if black_v.last().unwrap().0 == self.black_id { black_v.last_mut().unwrap().1 = val; } else { black_v.push((self.black_id, val)); } }
    fn snap(&mut self) -> i32 { self.black_id += 1; self.black_id - 1 }
    fn get(&self, index: i32, snap_id: i32) -> i32 { let black_v = &self.black_data[index as usize]; match black_v.binary_search_by_key(&snap_id, |&(s, _)| s) { Ok(i) => black_v[i].1, Err(i) => black_v[i-1].1 } }
}