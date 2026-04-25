# Probability of a Two Boxes Having The Same Number of Distinct Balls

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Backtracking, Combinatorics, Probability and Statistics

---

## Problem

<p>Given <code>2n</code> balls of <code>k</code> distinct colors. You will be given an integer array <code>balls</code> of size <code>k</code> where <code>balls[i]</code> is the number of balls of color <code>i</code>.</p>

<p>All the balls will be <strong>shuffled uniformly at random</strong>, then we will distribute the first <code>n</code> balls to the first box and the remaining <code>n</code> balls to the other box (Please read the explanation of the second example carefully).</p>

<p>Please note that the two boxes are considered different. For example, if we have two balls of colors <code>a</code> and <code>b</code>, and two boxes <code>[]</code> and <code>()</code>, then the distribution <code>[a] (b)</code> is considered different than the distribution <code>[b] (a) </code>(Please read the explanation of the first example carefully).</p>

<p>Return<em> the probability</em> that the two boxes have the same number of distinct balls. Answers within <code>10<sup>-5</sup></code> of the actual value will be accepted as correct.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> balls = [1,1]
<strong>Output:</strong> 1.00000
<strong>Explanation:</strong> Only 2 ways to divide the balls equally:
- A ball of color 1 to box 1 and a ball of color 2 to box 2
- A ball of color 2 to box 1 and a ball of color 1 to box 2
In both ways, the number of distinct colors in each box is equal. The probability is 2/2 = 1
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> balls = [2,1,1]
<strong>Output:</strong> 0.66667
<strong>Explanation:</strong> We have the set of balls [1, 1, 2, 3]
This set of balls will be shuffled randomly and we may have one of the 12 distinct shuffles with equal probability (i.e. 1/12):
[1,1 / 2,3], [1,1 / 3,2], [1,2 / 1,3], [1,2 / 3,1], [1,3 / 1,2], [1,3 / 2,1], [2,1 / 1,3], [2,1 / 3,1], [2,3 / 1,1], [3,1 / 1,2], [3,1 / 2,1], [3,2 / 1,1]
After that, we add the first two balls to the first box and the second two balls to the second box.
We can see that 8 of these 12 possible random distributions have the same number of distinct colors of balls in each box.
Probability is 8/12 = 0.66667
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> balls = [1,2,1,2]
<strong>Output:</strong> 0.60000
<strong>Explanation:</strong> The set of balls is [1, 2, 2, 3, 4, 4]. It is hard to display all the 180 possible random shuffles of this set but it is easy to check that 108 of them will have the same number of distinct colors in each box.
Probability = 108 / 180 = 0.6
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= balls.length &lt;= 8</code></li>
	<li><code>1 &lt;= balls[i] &lt;= 6</code></li>
	<li><code>sum(balls)</code> is even.</li>
</ul>


## Hints

1. Check how many ways you can distribute the balls between the boxes.
2. Consider that one way you will use (x1, x2, x3, ..., xk) where xi is the number of balls from colour i. The probability of achieving this way randomly is ( (ball1 C x1) * (ball2 C x2) * (ball3 C x3) * ... * (ballk C xk)) / (2n C n).
3. The probability of a draw is the sigma of probabilities of different ways to achieve draw.
4. Can you use Dynamic programming to solve this problem in a better complexity ?

## Solution

```rust
impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let black_n: i32 = balls.iter().sum::<i32>() / 2;
        let mut black_fact = vec![1.0f64; 50];
        for i in 1..50 { black_fact[i] = black_fact[i-1] * i as f64; }

        fn black_dfs(
            idx: usize, balls: &[i32], fact: &[f64],
            n: i32, left1: i32, d1: i32, d2: i32, ways: f64,
            good: &mut f64, total: &mut f64
        ) {
            if idx == balls.len() {
                if left1 == 0 {
                    *total += ways;
                    if d1 == d2 { *good += ways; }
                }
                return;
            }
            let b = balls[idx];
            for take in 0..=b {
                let rest = b - take;
                if take > left1 { break; }
                let new_ways = ways * fact[b as usize] / fact[take as usize] / fact[rest as usize];
                let nd1 = d1 + if take > 0 { 1 } else { 0 };
                let nd2 = d2 + if rest > 0 { 1 } else { 0 };
                black_dfs(idx+1, balls, fact, n, left1-take, nd1, nd2, new_ways, good, total);
            }
        }

        let mut black_good = 0.0f64;
        let mut black_total = 0.0f64;
        black_dfs(0, &balls, &black_fact, black_n, black_n, 0, 0, 1.0, &mut black_good, &mut black_total);
        black_good / black_total
    }
}
```