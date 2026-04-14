impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let mut black_k = k as i64;
        let black_n = num.len();
        let black_chars: Vec<char> = num.chars().collect();
        let mut black_pos = vec![std::collections::VecDeque::new(); 10];
        for (black_idx, &black_ch) in black_chars.iter().enumerate() {
            black_pos[(black_ch as u8 - b'0') as usize].push_back(black_idx + 1);
        }

        let mut black_bit = vec![0; black_n + 1];
        let mut black_used = vec![false; black_n + 1];
        let mut black_res = String::new();

        fn black_update(black_bit: &mut Vec<i32>, mut black_idx: usize, black_v: i32) {
            while black_idx < black_bit.len() {
                black_bit[black_idx] += black_v;
                black_idx += (black_idx as i32 & -(black_idx as i32)) as usize;
            }
        }

        fn black_query(black_bit: &Vec<i32>, mut black_idx: usize) -> i32 {
            let mut black_sum = 0;
            while black_idx > 0 {
                black_sum += black_bit[black_idx];
                black_idx -= (black_idx as i32 & -(black_idx as i32)) as usize;
            }
            black_sum
        }

        for _ in 0..black_n {
            for black_d in 0..10 {
                if let Some(&black_p) = black_pos[black_d].front() {
                    let black_cost = (black_p - 1 - black_query(&black_bit, black_p) as usize) as i64;
                    if black_cost <= black_k {
                        black_k -= black_cost;
                        black_res.push((black_d as u8 + b'0') as char);
                        black_update(&mut black_bit, black_p, 1);
                        black_used[black_p] = true;
                        black_pos[black_d].pop_front();
                        break;
                    }
                }
            }
        }
        black_res
    }
}