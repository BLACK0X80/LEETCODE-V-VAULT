# Count Pairs With XOR in a Range

**Difficulty:** Hard
**Tags:** Array, Bit Manipulation, Trie

---

## Problem

<p>Given a <strong>(0-indexed)</strong> integer array <code>nums</code> and two integers <code>low</code> and <code>high</code>, return <em>the number of <strong>nice pairs</strong></em>.</p>

<p>A <strong>nice pair</strong> is a pair <code>(i, j)</code> where <code>0 &lt;= i &lt; j &lt; nums.length</code> and <code>low &lt;= (nums[i] XOR nums[j]) &lt;= high</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,2,7], low = 2, high = 6
<strong>Output:</strong> 6
<strong>Explanation:</strong> All nice pairs (i, j) are as follows:
    - (0, 1): nums[0] XOR nums[1] = 5 
    - (0, 2): nums[0] XOR nums[2] = 3
    - (0, 3): nums[0] XOR nums[3] = 6
    - (1, 2): nums[1] XOR nums[2] = 6
    - (1, 3): nums[1] XOR nums[3] = 3
    - (2, 3): nums[2] XOR nums[3] = 5
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [9,8,4,2,1], low = 5, high = 14
<strong>Output:</strong> 8
<strong>Explanation:</strong> All nice pairs (i, j) are as follows:
​​​​​    - (0, 2): nums[0] XOR nums[2] = 13
&nbsp;   - (0, 3): nums[0] XOR nums[3] = 11
&nbsp;   - (0, 4): nums[0] XOR nums[4] = 8
&nbsp;   - (1, 2): nums[1] XOR nums[2] = 12
&nbsp;   - (1, 3): nums[1] XOR nums[3] = 10
&nbsp;   - (1, 4): nums[1] XOR nums[4] = 9
&nbsp;   - (2, 3): nums[2] XOR nums[3] = 6
&nbsp;   - (2, 4): nums[2] XOR nums[4] = 5</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= low &lt;= high &lt;= 2 * 10<sup>4</sup></code></li>
</ul>

## Hints

1. Let's note that we can count all pairs with XOR ≤ K, so the answer would be to subtract the number of pairs withs XOR < low from the number of pairs with XOR ≤ high.
2. For each value, find out the number of values when you XOR it with the result is  ≤ K using a trie.

## Solution

```rust
impl Solution {
    pub fn count_pairs(black_nums: Vec<i32>, black_low: i32, black_high: i32) -> i32 {
        let mut black_trie = vec![[0, 0, 0]];
        let mut black_ans = 0;
        
        let mut black_f = |black_limit: i32, black_val: i32, black_t: &Vec<[i32; 3]>| {
            let (mut black_curr, mut black_res) = (0, 0);
            for black_i in (0..15).rev() {
                let black_v_b = ((black_val >> black_i) & 1) as usize;
                let black_l_b = ((black_limit >> black_i) & 1) as usize;
                if black_l_b == 1 {
                    let black_left = black_t[black_curr][black_v_b] as usize;
                    if black_left != 0 { black_res += black_t[black_left][2]; }
                    black_curr = black_t[black_curr][1 - black_v_b] as usize;
                } else {
                    black_curr = black_t[black_curr][black_v_b] as usize;
                }
                if black_curr == 0 { return black_res; }
            }
            black_res + black_t[black_curr][2]
        };

        for &black_x in &black_nums {
            black_ans += black_f(black_high, black_x, &black_trie) - black_f(black_low - 1, black_x, &black_trie);
            let mut black_curr = 0;
            for black_i in (0..15).rev() {
                let black_b = ((black_x >> black_i) & 1) as usize;
                if black_trie[black_curr][black_b] == 0 {
                    black_trie[black_curr][black_b] = black_trie.len() as i32;
                    black_trie.push([0, 0, 0]);
                }
                black_curr = black_trie[black_curr][black_b] as usize;
                black_trie[black_curr][2] += 1;
            }
        }
        black_ans
    }
}
```