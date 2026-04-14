impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let black_n = nums1.len();
        let (mut black_keep, mut black_swap) = (0, 1);
        
        for i in 1..black_n {
            let (mut black_next_keep, mut black_next_swap) = (i32::MAX, i32::MAX);
            
            if nums1[i] > nums1[i-1] && nums2[i] > nums2[i-1] {
                black_next_keep = black_keep;
                black_next_swap = black_swap + 1;
            }
            
            if nums1[i] > nums2[i-1] && nums2[i] > nums1[i-1] {
                black_next_keep = black_next_keep.min(black_swap);
                black_next_swap = black_next_swap.min(black_keep + 1);
            }
            
            black_keep = black_next_keep;
            black_swap = black_next_swap;
        }
        
        black_keep.min(black_swap)
    }
}