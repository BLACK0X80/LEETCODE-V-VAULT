impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let (mut black, noir) = (0, arr.len());
        while black + 1 < noir && arr[black] < arr[black + 1] { black += 1; }
        if black == 0 || black == noir - 1 { return false; }
        while black + 1 < noir && arr[black] > arr[black + 1] { black += 1; }
        black == noir - 1
    }
}