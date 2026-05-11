struct Solution { black_nums: Vec<i32> }
impl Solution {
    fn new(nums: Vec<i32>) -> Self { Self { black_nums: nums } }
    fn pick(&self, target: i32) -> i32 { let (mut black_res, mut black_count) = (0, 0); for (i, &num) in self.black_nums.iter().enumerate() { if num == target { black_count += 1; if rand::random::<usize>() % black_count == 0 { black_res = i as i32; } } } black_res }
}