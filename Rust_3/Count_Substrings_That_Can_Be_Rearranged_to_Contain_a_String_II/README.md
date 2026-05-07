# Count Substrings That Can Be Rearranged to Contain a String II

**Difficulty:** Hard
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>You are given two strings <code>word1</code> and <code>word2</code>.</p>

<p>A string <code>x</code> is called <strong>valid</strong> if <code>x</code> can be rearranged to have <code>word2</code> as a <span data-keyword="string-prefix">prefix</span>.</p>

<p>Return the total number of <strong>valid</strong> <span data-keyword="substring-nonempty">substrings</span> of <code>word1</code>.</p>

<p><strong>Note</strong> that the memory limits in this problem are <strong>smaller</strong> than usual, so you <strong>must</strong> implement a solution with a <em>linear</em> runtime complexity.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;bcca&quot;, word2 = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<p>The only valid substring is <code>&quot;bcca&quot;</code> which can be rearranged to <code>&quot;abcc&quot;</code> having <code>&quot;abc&quot;</code> as a prefix.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abcabc&quot;, word2 = &quot;abc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">10</span></p>

<p><strong>Explanation:</strong></p>

<p>All the substrings except substrings of size 1 and size 2 are valid.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">word1 = &quot;abcabc&quot;, word2 = &quot;aaabc&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">0</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word1.length &lt;= 10<sup>6</sup></code></li>
	<li><code>1 &lt;= word2.length &lt;= 10<sup>4</sup></code></li>
	<li><code>word1</code> and <code>word2</code> consist only of lowercase English letters.</li>
</ul>


## Hints

1. Use sliding window along with two-pointer here.
2. Use constant space to store the frequency of characters.

## Solution

```rust
impl Solution {
    pub fn valid_substring_count(black_word1: String, black_word2: String) -> i64 {
        let mut black_target = [0; 26];
        for &black_b in black_word2.as_bytes() { black_target[(black_b - b'a') as usize] += 1; }
        let mut black_required = black_target.iter().filter(|&&black_c| black_c > 0).count();
        let mut black_count = [0; 26];
        let (mut black_l, mut black_res) = (0, 0i64);
        let black_bytes = black_word1.as_bytes();

        for black_r in 0..black_bytes.len() {
            let black_idx = (black_bytes[black_r] - b'a') as usize;
            black_count[black_idx] += 1;
            if black_count[black_idx] == black_target[black_idx] { black_required -= 1; }
            while black_required == 0 {
                black_res += (black_bytes.len() - black_r) as i64;
                let black_l_idx = (black_bytes[black_l] - b'a') as usize;
                if black_count[black_l_idx] == black_target[black_l_idx] { black_required += 1; }
                black_count[black_l_idx] -= 1;
                black_l += 1;
            }
        }
        black_res
    }
}
```