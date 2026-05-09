# Minimum Possible Integer After at Most K Adjacent Swaps On Digits

**Difficulty:** Hard
**Tags:** String, Greedy, Binary Indexed Tree, Segment Tree

---

## Problem

<p>You are given a string <code>num</code> representing <strong>the digits</strong> of a very large integer and an integer <code>k</code>. You are allowed to swap any two adjacent digits of the integer <strong>at most</strong> <code>k</code> times.</p>

<p>Return <em>the minimum integer you can obtain also as a string</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/06/17/q4_1.jpg" style="width: 500px; height: 40px;" />
<pre>
<strong>Input:</strong> num = &quot;4321&quot;, k = 4
<strong>Output:</strong> &quot;1342&quot;
<strong>Explanation:</strong> The steps to obtain the minimum integer from 4321 with 4 adjacent swaps are shown.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;100&quot;, k = 1
<strong>Output:</strong> &quot;010&quot;
<strong>Explanation:</strong> It&#39;s ok for the output to have leading zeros, but the input is guaranteed not to have any leading zeros.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = &quot;36789&quot;, k = 1000
<strong>Output:</strong> &quot;36789&quot;
<strong>Explanation:</strong> We can keep the number without any swaps.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= num.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>num</code> consists of only <strong>digits</strong> and does not contain <strong>leading zeros</strong>.</li>
	<li><code>1 &lt;= k &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. We want to make the smaller digits the most significant digits in the number.
2. For each index i, check the smallest digit in a window of size k and append it to the answer. Update the indices of all digits in this range accordingly.

## Solution

```rust
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
```