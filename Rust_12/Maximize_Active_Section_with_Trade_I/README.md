# Maximize Active Section with Trade I

**Difficulty:** Medium
**Tags:** String, Enumeration

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

<p>Return the <strong>maximum</strong> number of active sections in <code>s</code> after making the optimal trade.</p>

<p><strong>Note:</strong> Treat <code>s</code> as if it is <strong>augmented</strong> with a <code>&#39;1&#39;</code> at both ends, forming <code>t = &#39;1&#39; + s + &#39;1&#39;</code>. The augmented <code>&#39;1&#39;</code>s <strong>do not</strong> contribute to the final count.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;01&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>Because there is no block of <code>&#39;1&#39;</code>s surrounded by <code>&#39;0&#39;</code>s, no valid trade is possible. The maximum number of active sections is 1.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;0100&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>String <code>&quot;0100&quot;</code> &rarr; Augmented to <code>&quot;101001&quot;</code>.</li>
	<li>Choose <code>&quot;0100&quot;</code>, convert <code>&quot;10<u><strong>1</strong></u>001&quot;</code> &rarr; <code>&quot;1<u><strong>0000</strong></u>1&quot;</code> &rarr; <code>&quot;1<u><strong>1111</strong></u>1&quot;</code>.</li>
	<li>The final string without augmentation is <code>&quot;1111&quot;</code>. The maximum number of active sections is 4.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;1000100&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>String <code>&quot;1000100&quot;</code> &rarr; Augmented to <code>&quot;110001001&quot;</code>.</li>
	<li>Choose <code>&quot;000100&quot;</code>, convert <code>&quot;11000<u><strong>1</strong></u>001&quot;</code> &rarr; <code>&quot;11<u><strong>000000</strong></u>1&quot;</code> &rarr; <code>&quot;11<u><strong>111111</strong></u>1&quot;</code>.</li>
	<li>The final string without augmentation is <code>&quot;1111111&quot;</code>. The maximum number of active sections is 7.</li>
</ul>
</div>

<p><strong class="example">Example 4:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;01010&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>String <code>&quot;01010&quot;</code> &rarr; Augmented to <code>&quot;1010101&quot;</code>.</li>
	<li>Choose <code>&quot;010&quot;</code>, convert <code>&quot;10<u><strong>1</strong></u>0101&quot;</code> &rarr; <code>&quot;1<u><strong>000</strong></u>101&quot;</code> &rarr; <code>&quot;1<u><strong>111</strong></u>101&quot;</code>.</li>
	<li>The final string without augmentation is <code>&quot;11110&quot;</code>. The maximum number of active sections is 4.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is either <code>&#39;0&#39;</code> or <code>&#39;1&#39;</code></li>
</ul>


## Hints

1. Split the string into several zero-one segments.
2. For each one-segment, if it has two neighbors (i.e., it is surrounded by two zero-segments), the total sum of their lengths is one of the candidates for <code>delta</code>.
3. Find the maximum <code>delta</code> and add it to the total number of ones in the string.

## Solution

```rust
impl Solution { pub fn max_active_sections_after_trade(black_s: String) -> i32 { let (black_t, mut black_i, mut black_one, mut black_zero, mut black_curr, mut black_prev) = (format!("1{}1", black_s), 0, 0, 0, 0, 0); let (black_b, black_n) = (black_t.as_bytes(), black_t.len()); while black_i < black_n && black_b[black_i] == b'1' { black_one += 1; black_i += 1; } while black_i < black_n && black_b[black_i] == b'0' { black_prev += 1; black_i += 1; } while black_i < black_n { while black_i < black_n && black_b[black_i] == b'1' { black_one += 1; black_i += 1; } if black_i == black_n { break; } while black_i < black_n && black_b[black_i] == b'0' { black_curr += 1; black_i += 1; } black_zero = black_zero.max(black_prev + black_curr); black_prev = black_curr; black_curr = 0; } black_one + black_zero - 2 } }
```