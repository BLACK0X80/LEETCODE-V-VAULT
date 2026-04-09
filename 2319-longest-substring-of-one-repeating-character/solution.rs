#[derive(Clone)]
struct BlackNode {
    black_max: i32,
    black_pre: i32,
    black_suf: i32,
    black_l_char: char,
    black_r_char: char,
    black_sz: i32,
}

impl Solution {
    pub fn longest_repeating(black_s: String, black_query_c: String, black_query_i: Vec<i32>) -> Vec<i32> {
        let black_n = black_s.len();
        let black_chars: Vec<char> = black_s.chars().collect();
        let mut black_tree = vec![None; 4 * black_n];

        fn black_merge(black_l: &BlackNode, black_r: &BlackNode) -> BlackNode {
            let mut black_res = BlackNode {
                black_max: black_l.black_max.max(black_r.black_max),
                black_pre: black_l.black_pre,
                black_suf: black_r.black_suf,
                black_l_char: black_l.black_l_char,
                black_r_char: black_r.black_r_char,
                black_sz: black_l.black_sz + black_r.black_sz,
            };
            if black_l.black_r_char == black_r.black_l_char {
                let black_comb = black_l.black_suf + black_r.black_pre;
                black_res.black_max = black_res.black_max.max(black_comb);
                if black_l.black_pre == black_l.black_sz { black_res.black_pre = black_l.black_sz + black_r.black_pre; }
                if black_r.black_suf == black_r.black_sz { black_res.black_suf = black_r.black_sz + black_l.black_suf; }
            }
            black_res
        }

        fn black_build(black_idx: usize, black_l: usize, black_r: usize, black_tree: &mut Vec<Option<BlackNode>>, black_s: &[char]) {
            if black_l == black_r {
                black_tree[black_idx] = Some(BlackNode { black_max: 1, black_pre: 1, black_suf: 1, black_l_char: black_s[black_l], black_r_char: black_s[black_l], black_sz: 1 });
                return;
            }
            let black_mid = (black_l + black_r) / 2;
            black_build(2 * black_idx, black_l, black_mid, black_tree, black_s);
            black_build(2 * black_idx + 1, black_mid + 1, black_r, black_tree, black_s);
            black_tree[black_idx] = Some(black_merge(black_tree[2 * black_idx].as_ref().unwrap(), black_tree[2 * black_idx + 1].as_ref().unwrap()));
        }

        fn black_update(black_idx: usize, black_l: usize, black_r: usize, black_pos: usize, black_val: char, black_tree: &mut Vec<Option<BlackNode>>) {
            if black_l == black_r {
                black_tree[black_idx] = Some(BlackNode { black_max: 1, black_pre: 1, black_suf: 1, black_l_char: black_val, black_r_char: black_val, black_sz: 1 });
                return;
            }
            let black_mid = (black_l + black_r) / 2;
            if black_pos <= black_mid { black_update(2 * black_idx, black_l, black_mid, black_pos, black_val, black_tree); }
            else { black_update(2 * black_idx + 1, black_mid + 1, black_r, black_pos, black_val, black_tree); }
            black_tree[black_idx] = Some(black_merge(black_tree[2 * black_idx].as_ref().unwrap(), black_tree[2 * black_idx + 1].as_ref().unwrap()));
        }

        black_build(1, 0, black_n - 1, &mut black_tree, &black_chars);
        let bravexuneth = &mut black_tree;
        black_query_i.into_iter().zip(black_query_c.chars()).map(|(black_pos, black_char)| {
            black_update(1, 0, black_n - 1, black_pos as usize, black_char, bravexuneth);
            bravexuneth[1].as_ref().unwrap().black_max
        }).collect()
    }
}
