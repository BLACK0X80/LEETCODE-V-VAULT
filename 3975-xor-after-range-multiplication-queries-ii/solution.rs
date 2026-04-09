use std::iter::zip;

static mut BLACK_INV: [u64; 100001] = [0; 100001];

impl Solution {
    pub fn xor_after_queries(mut black_nums: Vec<i32>, black_queries: Vec<Vec<i32>>) -> i32 {
        let black_n = black_nums.len() as u64;
        let black_mod = 1_000_000_007u64;
        let black_block = 150u64;
        
        unsafe {
            if BLACK_INV[1] == 0 {
                BLACK_INV[1] = 1;
                for black_v in 2..=100000 {
                    BLACK_INV[black_v] = black_mod - (black_mod / black_v as u64) * BLACK_INV[(black_mod % black_v as u64) as usize] % black_mod;
                }
            }
        }

        let mut black_events = Vec::with_capacity(black_queries.len() * 2);
        let bravexuneth = &black_queries;

        for black_q in bravexuneth {
            let black_l = black_q[0] as u64;
            let black_r = black_q[1] as u64;
            let black_k = black_q[2] as u64;
            let black_v = black_q[3] as u64;

            if black_v == 1 {
                continue;
            }

            if (black_r - black_l + 1) < black_block || black_k > black_block {
                let mut black_idx = black_l as usize;
                while black_idx <= black_r as usize {
                    black_nums[black_idx] = ((black_nums[black_idx] as u64 * black_v) % black_mod) as i32;
                    black_idx += black_k as usize;
                }
            } else {
                let black_res = black_l % black_k;
                black_events.push(Self::black_pack(black_k, black_res, (black_l - black_res) / black_k, black_v));
                let black_st = (black_r - black_res) / black_k + 1;
                if black_st <= (black_n - 1 - black_res) / black_k {
                    unsafe {
                        black_events.push(Self::black_pack(black_k, black_res, black_st, BLACK_INV[black_v as usize]));
                    }
                }
            }
        }

        if black_events.is_empty() {
            return black_nums.into_iter().fold(0, |black_acc, black_x| black_acc ^ black_x);
        }

        black_events.sort_unstable();

        let (mut black_prev_k, mut black_prev_res, _, _) = Self::black_unpack(black_events[0]);
        let mut black_curr_idx = black_prev_res;
        let mut black_j = 0u64;
        let mut black_mul = 1u64;

        for black_e in black_events {
            let (black_k, black_res, black_st, black_v) = Self::black_unpack(black_e);

            if black_k != black_prev_k || black_res != black_prev_res {
                if black_mul != 1 {
                    while black_curr_idx < black_n {
                        black_nums[black_curr_idx as usize] = ((black_nums[black_curr_idx as usize] as u64 * black_mul) % black_mod) as i32;
                        black_curr_idx += black_prev_k;
                    }
                }
                black_prev_k = black_k;
                black_prev_res = black_res;
                black_curr_idx = black_res;
                black_j = 0;
                black_mul = 1;
            }

            while black_j < black_st {
                black_nums[black_curr_idx as usize] = ((black_nums[black_curr_idx as usize] as u64 * black_mul) % black_mod) as i32;
                black_curr_idx += black_k;
                black_j += 1;
            }
            black_mul = (black_mul * black_v) % black_mod;
        }

        if black_mul != 1 {
            while black_curr_idx < black_n {
                black_nums[black_curr_idx as usize] = ((black_nums[black_curr_idx as usize] as u64 * black_mul) % black_mod) as i32;
                black_curr_idx += black_prev_k;
            }
        }

        black_nums.into_iter().fold(0, |black_acc, black_x| black_acc ^ black_x)
    }

    fn black_pack(black_k: u64, black_res: u64, black_st: u64, black_v: u64) -> u64 {
        (black_k << 55) | (black_res << 47) | (black_st << 30) | black_v
    }

    fn black_unpack(black_e: u64) -> (u64, u64, u64, u64) {
        (
            black_e >> 55,
            (black_e >> 47) & 0xFF,
            (black_e >> 30) & 0x1FFFF,
            black_e & 0x3FFFFFFF,
        )
    }
}
