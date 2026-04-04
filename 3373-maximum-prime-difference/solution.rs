impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        let is_prime = |x: i32| x > 1 && (2..=(x as f64).sqrt() as i32).all(|i| x % i != 0);
        let primes: Vec<usize> = nums.iter().enumerate().filter(|(_, v)| is_prime(**v)).map(|(i, _)| i).collect();
        (primes.last().unwrap() - primes.first().unwrap()) as i32
    }
}
