impl Solution {
    pub fn smallest_number(black_num: String, black_t: i64) -> String {
        let mut black_temp_t = black_t;
        for &black_p in &[2, 3, 5, 7] {
            while black_temp_t % black_p == 0 {
                black_temp_t /= black_p;
            }
        }
        if black_temp_t > 1 {
            return "-1".into();
        }

        let black_bytes = black_num.as_bytes();
        let black_n = black_bytes.len();
        let mut black_prefix = Vec::with_capacity(black_n + 1);
        black_prefix.push(black_t);

        let mut black_curr = black_t;
        let mut black_zero_pos = black_n;
        for (i, &b) in black_bytes.iter().enumerate() {
            if b == b'0' {
                black_zero_pos = i;
                break;
            }
            black_curr /= Self::black_gcd(black_curr, (b - b'0') as i64);
            black_prefix.push(black_curr);
        }

        if black_zero_pos == black_n && black_curr == 1 {
            return black_num;
        }

        for black_idx in (0..=black_zero_pos.min(black_n - 1)).rev() {
            let black_req = black_prefix[black_idx];
            let black_rem_slots = black_n - 1 - black_idx;
            let black_start_d = (black_bytes[black_idx] - b'0' + 1) as i64;

            for black_d in black_start_d..=9 {
                let black_next_req = black_req / Self::black_gcd(black_req, black_d);
                if Self::black_can_fit(black_next_req, black_rem_slots) {
                    let mut black_res = Vec::with_capacity(black_n);
                    black_res.extend_from_slice(&black_bytes[..black_idx]);
                    black_res.push(black_d as u8 + b'0');
                    Self::black_fill_min(&mut black_res, black_next_req, black_rem_slots);
                    return unsafe { String::from_utf8_unchecked(black_res) };
                }
            }
        }

        for black_len in (black_n + 1)..=(black_n + 64) {
            for black_d in 1..=9 {
                let black_next_req = black_t / Self::black_gcd(black_t, black_d);
                if Self::black_can_fit(black_next_req, black_len - 1) {
                    let mut black_res = Vec::with_capacity(black_len);
                    black_res.push(black_d as u8 + b'0');
                    Self::black_fill_min(&mut black_res, black_next_req, black_len - 1);
                    return unsafe { String::from_utf8_unchecked(black_res) };
                }
            }
        }

        "-1".into()
    }

    fn black_can_fit(mut black_req: i64, black_slots: usize) -> bool {
        let mut count = 0;
        for &d in &[9, 8, 7, 6, 5, 4, 3, 2] {
            while black_req % d == 0 {
                black_req /= d;
                count += 1;
            }
        }
        count <= black_slots
    }

    fn black_fill_min(black_res: &mut Vec<u8>, mut black_req: i64, black_slots: usize) {
        let mut black_suffix = Vec::new();
        for &black_d in &[9, 8, 7, 6, 5, 4, 3, 2] {
            while black_req % black_d == 0 {
                black_suffix.push(black_d as u8 + b'0');
                black_req /= black_d;
            }
        }
        while black_suffix.len() < black_slots {
            black_suffix.push(b'1');
        }
        black_suffix.sort_unstable();
        black_res.extend(black_suffix);
    }

    fn black_gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
}
