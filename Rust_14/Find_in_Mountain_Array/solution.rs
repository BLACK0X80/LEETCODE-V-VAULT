impl Solution {
    pub fn find_in_mountain_array(black1: i32, black2: &MountainArray) -> i32 {
        let black3 = black2.length();
        let (mut black4, mut black5) = (0, black3 - 1);
        while black4 < black5 {
            let black6 = black4 + (black5 - black4) / 2;
            if black2.get(black6) < black2.get(black6 + 1) { black4 = black6 + 1; }
            else { black5 = black6; }
        }
        let black7 = black4;
        let black8 = Self::black_search(black2, black1, 0, black7, true);
        if black8 != -1 { return black8; }
        Self::black_search(black2, black1, black7 + 1, black3 - 1, false)
    }

    fn black_search(black2: &MountainArray, black1: i32, mut black4: i32, mut black5: i32, black9: bool) -> i32 {
        while black4 <= black5 {
            let black6 = black4 + (black5 - black4) / 2;
            let black10 = black2.get(black6);
            if black10 == black1 { return black6; }
            if black9 {
                if black10 < black1 { black4 = black6 + 1; }
                else { black5 = black6 - 1; }
            } else {
                if black10 > black1 { black4 = black6 + 1; }
                else { black5 = black6 - 1; }
            }
        }
        -1
    }
}