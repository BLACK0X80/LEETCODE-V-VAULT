# Count Ways to Make Array With Product

**Difficulty:** Hard
**Tags:** Array, Math, Dynamic Programming, Combinatorics, Number Theory

---

## Problem

<p>You are given a 2D integer array, <code>queries</code>. For each <code>queries[i]</code>, where <code>queries[i] = [n<sub>i</sub>, k<sub>i</sub>]</code>, find the number of different ways you can place positive integers into an array of size <code>n<sub>i</sub></code> such that the product of the integers is <code>k<sub>i</sub></code>. As the number of ways may be too large, the answer to the <code>i<sup>th</sup></code> query is the number of ways <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>Return <em>an integer array </em><code>answer</code><em> where </em><code>answer.length == queries.length</code><em>, and </em><code>answer[i]</code><em> is the answer to the </em><code>i<sup>th</sup></code><em> query.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> queries = [[2,6],[5,1],[73,660]]
<strong>Output:</strong> [4,1,50734910]
<strong>Explanation:</strong>&nbsp;Each query is independent.
[2,6]: There are 4 ways to fill an array of size 2 that multiply to 6: [1,6], [2,3], [3,2], [6,1].
[5,1]: There is 1 way to fill an array of size 5 that multiply to 1: [1,1,1,1,1].
[73,660]: There are 1050734917 ways to fill an array of size 73 that multiply to 660. 1050734917 modulo 10<sup>9</sup> + 7 = 50734910.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> queries = [[1,1],[2,2],[3,3],[4,4],[5,5]]
<strong>Output:</strong> [1,2,3,10,5]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>4</sup> </code></li>
	<li><code>1 &lt;= n<sub>i</sub>, k<sub>i</sub> &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Prime-factorize ki and count how many ways you can distribute the primes among the ni positions.
2. After prime factorizing ki, suppose there are x amount of prime factor. There are (x + n - 1) choose (n - 1) ways to distribute the x prime factors into n positions, allowing repetitions.

## Solution

```rust
impl Solution {
    pub fn ways_to_fill_array(black1: Vec<Vec<i32>>) -> Vec<i32> {
        let black2 = 1_000_000_007i64;
        let mut black3 = vec![];
        
        // Precompute combinations
        let mut black4 = vec![vec![0i64; 15]; 10015];
        for i in 0..10015 {
            black4[i][0] = 1;
            for j in 1..=i.min(14) {
                black4[i][j] = (black4[i-1][j-1] + black4[i-1][j]) % black2;
            }
        }

        for q in black1 {
            let n = q[0] as usize;
            let mut k = q[1];
            let mut res = 1i64;
            let mut d = 2;
            while d * d <= k {
                if k % d == 0 {
                    let mut e = 0;
                    while k % d == 0 { e += 1; k /= d; }
                    res = (res * black4[n + e - 1][e]) % black2;
                }
                d += 1;
            }
            if k > 1 { res = (res * n as i64) % black2; }
            black3.push(res as i32);
        }
        black3
    }
}
```