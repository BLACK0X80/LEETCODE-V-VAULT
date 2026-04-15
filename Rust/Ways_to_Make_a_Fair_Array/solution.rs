impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (mut odd, mut even) = (0, 0);
        for i in 0..n {
            if i & 1 == 1 { odd += nums[i]; } else { even += nums[i]; }
        }
        let (mut o, mut e, mut cnt) = (0, 0, 0);
        for i in 0..n {
            let no = odd - o - if i & 1 == 1 { nums[i] } else { 0 };
            let ne = even - e - if i & 1 == 0 { nums[i] } else { 0 };
            if e + no == o + ne { cnt += 1; }
            if i & 1 == 1 { o += nums[i]; } else { e += nums[i]; }
        }
        cnt
    }
}