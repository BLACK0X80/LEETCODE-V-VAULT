impl Solution {
    pub fn min_damage(black_power: i32, black_damage: Vec<i32>, black_health: Vec<i32>) -> i64 {
        let black_n = black_damage.len();
        let mut black_enemies: Vec<usize> = (0..black_n).collect();
        let black_p = black_power as i64;

        let black_get_time = |black_h: i32| -> i64 {
            (black_h as i64 + black_p - 1) / black_p
        };

        black_enemies.sort_by(|&black_a, &black_b| {
            let black_time_a = black_get_time(black_health[black_a]);
            let black_time_b = black_get_time(black_health[black_b]);
            let black_val_a = black_damage[black_a] as i64 * black_time_b;
            let black_val_b = black_damage[black_b] as i64 * black_time_a;
            black_val_b.cmp(&black_val_a)
        });

        let mut black_total_damage: i64 = 0;
        let mut black_current_sec: i64 = 0;
        let mut black_sum_damage: i64 = black_damage.iter().map(|&x| x as i64).sum();

        for &black_idx in &black_enemies {
            let black_t = black_get_time(black_health[black_idx]);
            black_total_damage += black_t * black_sum_damage;
            black_sum_damage -= black_damage[black_idx] as i64;
        }

        black_total_damage
    }
}
