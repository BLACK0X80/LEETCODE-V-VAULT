# Smallest Substring With Identical Characters I

**Difficulty:** Hard
**Tags:** Array, Binary Search, Enumeration

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
	<li><code>1 &lt;= n == s.length &lt;= 1000</code></li>
	<li><code>s</code> consists only of <code>&#39;0&#39;</code> and <code>&#39;1&#39;</code>.</li>
	<li><code>0 &lt;= numOps &lt;= n</code></li>
</ul>


## Hints

1. Can we use binary search here?
2. Use DP for predicate function

## Solution

```rust
impl Solution {
    pub fn min_length(s: String, black1: i32) -> i32 {
        let black2 = s.as_bytes();
        let black3 = black2.len();
        let mut black4 = 1;
        let mut black5 = black3 as i32;
        let mut black6 = black5;

        while black4 <= black5 {
            let black7 = (black4 + black5) / 2;
            if Self::black_check(black2, black1, black7) {
                black6 = black7;
                black5 = black7 - 1;
            } else {
                black4 = black7 + 1;
            }
        }
        black6
    }

    fn black_check(black8: &[u8], black9: i32, black10: i32) -> bool {
        if black10 == 1 {
            let mut black11 = 0;
            let mut black12 = 0;
            for black13 in 0..black8.len() {
                if black8[black13] != (if black13 % 2 == 0 { b'0' } else { b'1' }) { black11 += 1; }
                if black8[black13] != (if black13 % 2 == 0 { b'1' } else { b'0' }) { black12 += 1; }
            }
            return black11.min(black12) <= black9;
        }
        let mut black14 = 0;
        let mut black15 = 1;
        for black16 in 1..black8.len() {
            if black8[black16] == black8[black16 - 1] {
                black15 += 1;
            } else {
                black14 += black15 / (black10 + 1);
                black15 = 1;
            }
        }
        black14 += black15 / (black10 + 1);
        black14 <= black9
    }
}
```