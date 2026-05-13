impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let dirs: Vec<char> = directions.chars().collect();
        let mut healths = healths;

        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_unstable_by_key(|&i| positions[i]);

        let mut stack: Vec<usize> = Vec::new();

        for &i in &indices {
            if dirs[i] == 'R' {
                stack.push(i);
            } else {
                let mut current_health = healths[i];
                let mut survived = true;

                while let Some(&top) = stack.last() {
                    match current_health.cmp(&healths[top]) {
                        std::cmp::Ordering::Greater => {
                            healths[top] = 0;
                            stack.pop();
                            current_health -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            healths[top] -= 1;
                            survived = false;
                            break;
                        }
                        std::cmp::Ordering::Equal => {
                            healths[top] = 0;
                            stack.pop();
                            survived = false;
                            break;
                        }
                    }
                }

                healths[i] = if survived { current_health } else { 0 };
            }
        }

        (0..n).filter(|&i| healths[i] > 0).map(|i| healths[i]).collect()
    }
}