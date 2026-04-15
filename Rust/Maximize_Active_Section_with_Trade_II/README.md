# Maximize Active Section with Trade II

**Difficulty:** Hard
**Tags:** Array, String, Binary Search, Segment Tree

---

## Problem

<p>You are given a binary string <code>s</code> of length <code>n</code>, where:</p>

<ul>
	<li><code>&#39;1&#39;</code> represents an <strong>active</strong> section.</li>
	<li><code>&#39;0&#39;</code> represents an <strong>inactive</strong> section.</li>
</ul>

<p>You can perform <strong>at most one trade</strong> to maximize the number of active sections in <code>s</code>. In a trade, you:</p>

<ul>
	<li>Convert a contiguous block of <code>&#39;1&#39;</code>s that is surrounded by <code>&#39;0&#39;</code>s to all <code>&#39;0&#39;</code>s.</li>
	<li>Afterward, convert a contiguous block of <code>&#39;0&#39;</code>s that is surrounded by <code>&#39;1&#39;</code>s to all <code>&#39;1&#39;</code>s.</li>
</ul>

<p>Additionally, you are given a <strong>2D array</strong> <code>queries</code>, where <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code> represents a <span data-keyword="substring-nonempty">substring</span> <code>s[l<sub>i</sub>...r<sub>i</sub>]</code>.</p>

<p>For each query, determine the <strong>maximum</strong> possible number of active sections in <code>s</code> after making the optimal trade on the substring <code>s[l<sub>i</sub>...r<sub>i</sub>]</code>.</p>

<p>Return an array <code>answer</code>, where <code>answer[i]</code> is the result for <code>queries[i]</code>.</p>

<p><strong>Note</strong></p>

<ul>
	<li>For each query, treat <code>s[l<sub>i</sub>...r<sub>i</sub>]</code> as if it is <strong>augmented</strong> with a <code>&#39;1&#39;</code> at both ends, forming <code>t = &#39;1&#39; + s[l<sub>i</sub>...r<sub>i</sub>] + &#39;1&#39;</code>. The augmented <code>&#39;1&#39;</code>s <strong>do not</strong> contribute to the final count.</li>
	<li>The queries are independent of each other.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;01&quot;, queries = [[0,1]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[1]</span></p>

<p><strong>Explanation:</strong></p>

<p>Because there is no block of <code>&#39;1&#39;</code>s surrounded by <code>&#39;0&#39;</code>s, no valid trade is possible. The maximum number of active sections is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;0100&quot;, queries = [[0,3],[0,2],[1,3],[2,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[4,3,1,1]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>
	<p>Query <code>[0, 3]</code> &rarr; Substring <code>&quot;0100&quot;</code> &rarr; Augmented to <code>&quot;101001&quot;</code><br />
	Choose <code>&quot;0100&quot;</code>, convert <code>&quot;0100&quot;</code> &rarr; <code>&quot;0000&quot;</code> &rarr; <code>&quot;1111&quot;</code>.<br />
	The final string without augmentation is <code>&quot;1111&quot;</code>. The maximum number of active sections is 4.</p>
	</li>
	<li>
	<p>Query <code>[0, 2]</code> &rarr; Substring <code>&quot;010&quot;</code> &rarr; Augmented to <code>&quot;10101&quot;</code><br />
	Choose <code>&quot;010&quot;</code>, convert <code>&quot;010&quot;</code> &rarr; <code>&quot;000&quot;</code> &rarr; <code>&quot;111&quot;</code>.<br />
	The final string without augmentation is <code>&quot;1110&quot;</code>. The maximum number of active sections is 3.</p>
	</li>
	<li>
	<p>Query <code>[1, 3]</code> &rarr; Substring <code>&quot;100&quot;</code> &rarr; Augmented to <code>&quot;11001&quot;</code><br />
	Because there is no block of <code>&#39;1&#39;</code>s surrounded by <code>&#39;0&#39;</code>s, no valid trade is possible. The maximum number of active sections is 1.</p>
	</li>
	<li>
	<p>Query <code>[2, 3]</code> &rarr; Substring <code>&quot;00&quot;</code> &rarr; Augmented to <code>&quot;1001&quot;</code><br />
	Because there is no block of <code>&#39;1&#39;</code>s surrounded by <code>&#39;0&#39;</code>s, no valid trade is possible. The maximum number of active sections is 1.</p>
	</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1000100&quot;, queries = [[1,5],[0,6],[0,4]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[6,7,2]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li data-end="383" data-start="217">
	<p data-end="383" data-start="219">Query <code>[1, 5]</code> &rarr; Substring <code data-end="255" data-start="246">&quot;00010&quot;</code> &rarr; Augmented to <code data-end="282" data-start="271">&quot;1000101&quot;</code><br data-end="285" data-start="282" />
	Choose <code data-end="303" data-start="294">&quot;00010&quot;</code>, convert <code data-end="322" data-start="313">&quot;00010&quot;</code> &rarr; <code data-end="322" data-start="313">&quot;00000&quot;</code> &rarr; <code data-end="334" data-start="325">&quot;11111&quot;</code>.<br />
	The final string without augmentation is <code data-end="404" data-start="396">&quot;1111110&quot;</code>. The maximum number of active sections is 6.</p>
	</li>
	<li data-end="561" data-start="385">
	<p data-end="561" data-start="387">Query <code>[0, 6]</code> &rarr; Substring <code data-end="425" data-start="414">&quot;1000100&quot;</code> &rarr; Augmented to <code data-end="454" data-start="441">&quot;110001001&quot;</code><br data-end="457" data-start="454" />
	Choose <code data-end="477" data-start="466">&quot;000100&quot;</code>, convert <code data-end="498" data-start="487">&quot;000100&quot;</code> &rarr; <code data-end="498" data-start="487">&quot;000000&quot;</code> &rarr; <code data-end="512" data-start="501">&quot;111111&quot;</code>.<br />
	The final string without augmentation is <code data-end="404" data-start="396">&quot;1111111&quot;</code>. The maximum number of active sections is 7.</p>
	</li>
	<li data-end="741" data-start="563">
	<p data-end="741" data-start="565">Query <code>[0, 4]</code> &rarr; Substring <code data-end="601" data-start="592">&quot;10001&quot;</code> &rarr; Augmented to <code data-end="627" data-start="617">&quot;1100011&quot;</code><br data-end="630" data-start="627" />
	Because there is no block of <code>&#39;1&#39;</code>s surrounded by <code>&#39;0&#39;</code>s, no valid trade is possible. The maximum number of active sections is 2.</p>
	</li>
</ul>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;01010&quot;, queries = [[0,3],[1,4],[1,3]]</span></p>

<p><strong>Output:</strong> <span class="example-io">[4,4,2]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>
	<p>Query <code>[0, 3]</code> &rarr; Substring <code>&quot;0101&quot;</code> &rarr; Augmented to <code>&quot;101011&quot;</code><br />
	Choose <code>&quot;010&quot;</code>, convert <code>&quot;010&quot;</code> &rarr; <code>&quot;000&quot;</code> &rarr; <code>&quot;111&quot;</code>.<br />
	The final string without augmentation is <code>&quot;11110&quot;</code>. The maximum number of active sections is 4.</p>
	</li>
	<li>
	<p>Query <code>[1, 4]</code> &rarr; Substring <code>&quot;1010&quot;</code> &rarr; Augmented to <code>&quot;110101&quot;</code><br />
	Choose <code>&quot;010&quot;</code>, convert <code>&quot;010&quot;</code> &rarr; <code>&quot;000&quot;</code> &rarr; <code>&quot;111&quot;</code>.<br />
	The final string without augmentation is <code>&quot;01111&quot;</code>. The maximum number of active sections is 4.</p>
	</li>
	<li>
	<p>Query <code>[1, 3]</code> &rarr; Substring <code>&quot;101&quot;</code> &rarr; Augmented to <code>&quot;11011&quot;</code><br />
	Because there is no block of <code>&#39;1&#39;</code>s surrounded by <code>&#39;0&#39;</code>s, no valid trade is possible. The maximum number of active sections is 2.</p>
	</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= queries.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code>.</li>
	<li><code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code></li>
	<li><code>0 &lt;= l<sub>i</sub> &lt;= r<sub>i</sub> &lt; n</code></li>
</ul>


## Hints

1. Split consecutive zeros and ones into segments and give each segment an ID.
2. The answer should be the maximum of <code>ans[i] = len[i - 1] + len[i + 1]</code>, where <code>i</code> is a one-segment.
3. For a zero-segment, define <code>ans[i] = 0</code>.
4. Note that all three segments (<code>i - 1</code>, <code>i</code>, and <code>i + 1</code>) should be fully covered by the substring.
5. Use a segment tree to perform range maximum queries on the answer. The query to the segment tree is not straightforward since we need to ensure the zero-segments are fully covered. Handle the first and last segments separately.

## Solution

```rust
impl Solution {
    pub fn max_active_sections_after_trade(black_s: String, black_queries: Vec<Vec<i32>>) -> Vec<i32> {
        let black_n = black_s.len();
        let black_b = black_s.as_bytes();
        let mut black_active_count = 0;
        for &black_byte in black_b {
            if black_byte == b'1' { black_active_count += 1; }
        }

        let mut black_segments = Vec::new();
        let mut black_start = 0;
        for black_i in 0..black_n {
            if black_i == black_n - 1 || black_b[black_i] != black_b[black_i + 1] {
                black_segments.push((black_start, black_i - black_start + 1));
                black_start = black_i + 1;
            }
        }

        let black_segment_count = black_segments.len();
        let black_max_power = 20;
        let black_inf = 1_000_000_000;
        let mut black_rmq = vec![vec![-black_inf; black_segment_count]; black_max_power];

        for black_i in 0..black_segment_count {
            if black_b[black_segments[black_i].0] == b'0' && black_i + 2 < black_segment_count {
                black_rmq[0][black_i] = (black_segments[black_i].1 + black_segments[black_i + 2].1) as i32;
            }
        }

        for black_power in 1..black_max_power {
            let black_range_len = 1 << black_power;
            let black_half = 1 << (black_power - 1);
            if black_range_len > black_segment_count { break; }
            for black_i in 0..=(black_segment_count - black_range_len) {
                black_rmq[black_power][black_i] = black_rmq[black_power - 1][black_i].max(black_rmq[black_power - 1][black_i + black_half]);
            }
        }

        let black_get_max_in_range = |black_l: usize, black_r: usize| -> i32 {
            if black_l > black_r { return -black_inf; }
            let black_range_size = black_r - black_l + 1;
            let black_p = (black_range_size as f64).log2().floor() as usize;
            black_rmq[black_p][black_l].max(black_rmq[black_p][black_r - (1 << black_p) + 1])
        };

        let mut black_result = Vec::with_capacity(black_queries.len());

        for black_query in black_queries {
            let black_left = black_query[0] as usize;
            let black_right = black_query[1] as usize;

            let black_left_idx = match black_segments.binary_search_by_key(&black_left, |&(black_s, _)| black_s) {
                Ok(black_found) => black_found,
                Err(black_e) => black_e - 1,
            };
            let black_right_idx = match black_segments.binary_search_by_key(&black_right, |&(black_s, _)| black_s) {
                Ok(black_found) => black_found,
                Err(black_e) => black_e - 1,
            };

            if black_right_idx - black_left_idx + 1 <= 2 {
                black_result.push(black_active_count);
                continue;
            }

            let black_get_size = |black_idx: usize| -> i32 {
                if black_idx == black_left_idx {
                    (black_segments[black_left_idx].1 - (black_left - black_segments[black_left_idx].0)) as i32
                } else if black_idx == black_right_idx {
                    (black_right - black_segments[black_right_idx].0 + 1) as i32
                } else {
                    black_segments[black_idx].1 as i32
                }
            };

            let black_calc = |black_idx: usize| -> i32 {
                if black_b[black_segments[black_idx].0] == b'1' { return -black_inf; }
                black_get_size(black_idx) + black_get_size(black_idx + 2)
            };

            let mut black_best = 0.max(black_get_max_in_range(black_left_idx + 1, black_right_idx.saturating_sub(3)));
            black_best = black_best.max(black_calc(black_left_idx));
            if black_right_idx >= 2 {
                black_best = black_best.max(black_calc(black_right_idx - 2));
            }

            black_result.push(black_active_count + black_best);
        }

        black_result
    }
}
```