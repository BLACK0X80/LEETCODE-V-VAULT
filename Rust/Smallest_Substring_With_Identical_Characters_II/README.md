# Smallest Substring With Identical Characters II

**Difficulty:** Hard
**Tags:** String, Binary Search

---

## Problem

<p>You are given a binary string <code>s</code> of length <code>n</code> and an integer <code>numOps</code>.</p>

<p>You are allowed to perform the following operation on <code>s</code> <strong>at most</strong> <code>numOps</code> times:</p>

<ul>
	<li>Select any index <code>i</code> (where <code>0 &lt;= i &lt; n</code>) and <strong>flip</strong> <code>s[i]</code>. If <code>s[i] == &#39;1&#39;</code>, change <code>s[i]</code> to <code>&#39;0&#39;</code> and vice versa.</li>
</ul>

<p>You need to <strong>minimize</strong> the length of the <strong>longest</strong> <span data-keyword="substring-nonempty">substring</span> of <code>s</code> such that all the characters in the substring are <strong>identical</strong>.</p>

<p>Return the <strong>minimum</strong> length after the operations.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;000001&quot;, numOps = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p>By changing <code>s[2]</code> to <code>&#39;1&#39;</code>, <code>s</code> becomes <code>&quot;001001&quot;</code>. The longest substrings with identical characters are <code>s[0..1]</code> and <code>s[3..4]</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;0000&quot;, numOps = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong>&nbsp;</p>

<p>By changing <code>s[0]</code> and <code>s[2]</code> to <code>&#39;1&#39;</code>, <code>s</code> becomes <code>&quot;1010&quot;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;0101&quot;, numOps = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists only of <code>&#39;0&#39;</code> and <code>&#39;1&#39;</code>.</li>
	<li><code>0 &lt;= numOps &lt;= n</code></li>
</ul>


## Hints

1. Binary search for the answer.
2. Group the same digits by size of <code>(mid + 1)</code> and ignore any remainder. Flip one in each group (the last one).
3. For the last group, we can flip the 2nd last one.
4. What if the answer was 1?

## Solution

```rust
impl Solution {
    pub fn min_length(black_s: String, black_num_ops: i32) -> i32 {
        let black_b = black_s.as_bytes();
        let black_n = black_b.len();
        let mut black_check = |black_mid: i32| -> bool {
            if black_mid == 1 {
                let mut black_c0 = 0;
                let mut black_c1 = 0;
                for i in 0..black_n {
                    if black_b[i] != (if i % 2 == 0 { b'0' } else { b'1' }) { black_c0 += 1; }
                    if black_b[i] != (if i % 2 == 0 { b'1' } else { b'0' }) { black_c1 += 1; }
                }
                return black_c0.min(black_c1) <= black_num_ops;
            }
            let (mut black_ops, mut black_cnt) = (0, 1);
            for i in 1..black_n {
                if black_b[i] == black_b[i-1] { black_cnt += 1; }
                else { black_ops += black_cnt / (black_mid + 1); black_cnt = 1; }
            }
            black_ops += black_cnt / (black_mid + 1);
            black_ops <= black_num_ops
        };
        let (mut black_low, mut black_high) = (1, black_n as i32);
        let mut black_res = black_high;
        while black_low <= black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if black_check(black_mid) { black_res = black_mid; black_high = black_mid - 1; }
            else { black_low = black_mid + 1; }
        }
        black_res
    }
}
```