impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut black = 0;
        while black < arr.len() { if arr[black] == 0 { arr.insert(black + 1, 0); arr.pop(); black += 2; } else { black += 1; } }
    }
}