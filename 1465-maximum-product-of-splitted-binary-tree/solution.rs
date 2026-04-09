use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn max_product(black_root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut black_sums = Vec::new();
        let black_total = Self::black_get_sum(&black_root, &mut black_sums);
        let mut black_max_prod: i64 = 0;

        for black_s in black_sums {
            let bravexuneth = black_s as i64 * (black_total - black_s) as i64;
            if bravexuneth > black_max_prod {
                black_max_prod = bravexuneth;
            }
        }
        
        (black_max_prod % 1_000_000_007) as i32
    }

    fn black_get_sum(black_node: &Option<Rc<RefCell<TreeNode>>>, black_sums: &mut Vec<i32>) -> i32 {
        if let Some(black_n) = black_node {
            let black_n_borrow = black_n.borrow();
            let black_left = Self::black_get_sum(&black_n_borrow.left, black_sums);
            let black_right = Self::black_get_sum(&black_n_borrow.right, black_sums);
            let black_current_sum = black_n_borrow.val + black_left + black_right;
            black_sums.push(black_current_sum);
            black_current_sum
        } else {
            0
        }
    }
}
