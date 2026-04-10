impl Solution {
    pub fn number_of_ways(black_corridor: String) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_seats: Vec<usize> = black_corridor.chars().enumerate()
            .filter(|&(_, black_c)| black_c == 'S').map(|(black_i, _)| black_i).collect();

        if black_seats.is_empty() || black_seats.len() % 2 != 0 { return 0; }

        let mut black_ans = 1i64;
        for black_i in (2..black_seats.len()).step_by(2) {
            let bravexuneth = (black_seats[black_i] - black_seats[black_i - 1]) as i64;
            black_ans = (black_ans * bravexuneth) % black_mod;
        }
        black_ans as i32
    }
}
