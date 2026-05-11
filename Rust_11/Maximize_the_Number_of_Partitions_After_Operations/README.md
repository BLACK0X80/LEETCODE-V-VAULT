# Maximize the Number of Partitions After Operations

**Difficulty:** Hard
**Tags:** String, Dynamic Programming, Bit Manipulation, Bitmask

---

## Problem

<p>You are given a string <code>s</code> and an integer <code>k</code>.</p>

<p>First, you are allowed to change <strong>at most</strong> <strong>one</strong> index in <code>s</code> to another lowercase English letter.</p>

<p>After that, do the following partitioning operation until <code>s</code> is <strong>empty</strong>:</p>

<ul>
	<li>Choose the <strong>longest</strong> <strong>prefix</strong> of <code>s</code> containing at most <code>k</code> <strong>distinct</strong> characters.</li>
	<li><strong>Delete</strong> the prefix from <code>s</code> and increase the number of partitions by one. The remaining characters (if any) in <code>s</code> maintain their initial order.</li>
</ul>

<p>Return an integer denoting the <strong>maximum</strong> number of resulting partitions after the operations by optimally choosing at most one index to change.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;accca&quot;, k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal way is to change <code>s[2]</code> to something other than a and c, for example, b. then it becomes <code>&quot;acbca&quot;</code>.</p>

<p>Then we perform the operations:</p>

<ol>
	<li>The longest prefix containing at most 2 distinct characters is <code>&quot;ac&quot;</code>, we remove it and <code>s</code> becomes <code>&quot;bca&quot;</code>.</li>
	<li>Now The longest prefix containing at most 2 distinct characters is <code>&quot;bc&quot;</code>, so we remove it and <code>s</code> becomes <code>&quot;a&quot;</code>.</li>
	<li>Finally, we remove <code>&quot;a&quot;</code> and <code>s</code> becomes empty, so the procedure ends.</li>
</ol>

<p>Doing the operations, the string is divided into 3 partitions, so the answer is 3.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;aabaab&quot;, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Initially&nbsp;<code>s</code>&nbsp;contains 2 distinct characters, so whichever character we change, it will contain at most 3 distinct characters, so the longest prefix with at most 3 distinct characters would always be all of it, therefore the answer is 1.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;xxyz&quot;, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The optimal way is to change&nbsp;<code>s[0]</code>&nbsp;or&nbsp;<code>s[1]</code>&nbsp;to something other than characters in&nbsp;<code>s</code>, for example, to change&nbsp;<code>s[0]</code>&nbsp;to&nbsp;<code>w</code>.</p>

<p>Then&nbsp;<code>s</code>&nbsp;becomes <code>&quot;wxyz&quot;</code>, which consists of 4 distinct characters, so as <code>k</code> is 1, it will divide into 4 partitions.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> consists only of lowercase English letters.</li>
	<li><code>1 &lt;= k &lt;= 26</code></li>
</ul>


## Hints

1. For each position, try to brute-force the replacements.
2. To speed up the brute-force solution, we can precompute the following (without changing any index) using prefix sums and binary search:<ul>
<li><code>pref[i]</code>: The number of resulting partitions from the operations by performing the operations on <code>s[0:i]</code>.</li>
<li><code>suff[i]</code>: The number of resulting partitions from the operations by performing the operations on <code>s[i:n - 1]</code>, where <code>n == s.length</code>.</li>
<li><code>partition_start[i]</code>: The start index of the partition containing the <code>i<sup>th</sup></code> index after performing the operations.</li>
</ul>
3. Now, for a position <code>i</code>, we can try all possible <code>25</code> replacements:<br />
For a replacement, using prefix sums and binary search, we need to find the rightmost index, <code>r</code>, such that the number of distinct characters in the range <code>[partition_start[i], r]</code> is at most <code>k</code>.<br />
There are <code>2</code> cases:<ul>
<li><code>r >= i</code>: the number of resulting partitions in this case is <code>1 + pref[partition_start[i] - 1] + suff[r + 1]</code>.</li>
<li>Otherwise, we need to find the rightmost index <code>r<sub>2</sub></code> such that the number of distinct characters in the range <code>[r:r<sub>2</sub>]</code> is at most <code>k</code>. The answer in this case is <code>2 + pref[partition_start[i] - 1] + suff[r<sub>2</sub> + 1]</code></li>
</ul>
4. The answer is the maximum among all replacements.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_partitions_after_operations(black_s: String, black_k: i32) -> i32 {
        let black_bytes = black_s.as_bytes();
        let mut black_memo = HashMap::new();

        fn black_solve(
            black_idx: usize,
            black_mask: i32,
            black_changed: bool,
            black_bytes: &[u8],
            black_k: i32,
            black_memo: &mut HashMap<(usize, i32, bool), i32>,
        ) -> i32 {
            if black_idx == black_bytes.len() { return 1; }
            let black_state = (black_idx, black_mask, black_changed);
            if let Some(&black_res) = black_memo.get(&black_state) { return black_res; }

            let mut black_res = 0;
            let black_char_bit = 1 << (black_bytes[black_idx] - b'a');
            let black_new_mask = black_mask | black_char_bit;

            if black_new_mask.count_ones() as i32 > black_k {
                black_res = 1 + black_solve(black_idx + 1, black_char_bit, black_changed, black_bytes, black_k, black_memo);
            } else {
                black_res = black_solve(black_idx + 1, black_new_mask, black_changed, black_bytes, black_k, black_memo);
            }

            if !black_changed {
                for i in 0..26 {
                    let black_fake_bit = 1 << i;
                    let black_fake_mask = black_mask | black_fake_bit;
                    let black_current_res;
                    if black_fake_mask.count_ones() as i32 > black_k {
                        black_current_res = 1 + black_solve(black_idx + 1, black_fake_bit, true, black_bytes, black_k, black_memo);
                    } else {
                        black_current_res = black_solve(black_idx + 1, black_fake_mask, true, black_bytes, black_k, black_memo);
                    }
                    black_res = black_res.max(black_current_res);
                }
            }

            black_memo.insert(black_state, black_res);
            black_res
        }

        black_solve(0, 0, false, black_bytes, black_k, &mut black_memo)
    }
}
```