# Count Complete Substrings

**Difficulty:** Hard
**Tags:** Hash Table, String, Sliding Window

---

## Problem

<p>You are given a string <code>word</code> and an integer <code>k</code>.</p>

<p>A substring <code>s</code> of <code>word</code> is <strong>complete</strong> if:</p>

<ul>
	<li>Each character in <code>s</code> occurs <strong>exactly</strong> <code>k</code> times.</li>
	<li>The difference between two adjacent characters is <strong>at most</strong> <code>2</code>. That is, for any two adjacent characters <code>c1</code> and <code>c2</code> in <code>s</code>, the absolute difference in their positions in the alphabet is <strong>at most</strong> <code>2</code>.</li>
</ul>

<p>Return <em>the number of <strong>complete </strong>substrings of</em> <code>word</code>.</p>

<p>A <strong>substring</strong> is a <strong>non-empty</strong> contiguous sequence of characters in a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;igigee&quot;, k = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> The complete substrings where each character appears exactly twice and the difference between adjacent characters is at most 2 are: <u><strong>igig</strong></u>ee, igig<u><strong>ee</strong></u>, <u><strong>igigee</strong></u>.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> word = &quot;aaabbbccc&quot;, k = 3
<strong>Output:</strong> 6
<strong>Explanation:</strong> The complete substrings where each character appears exactly three times and the difference between adjacent characters is at most 2 are: <strong><u>aaa</u></strong>bbbccc, aaa<u><strong>bbb</strong></u>ccc, aaabbb<u><strong>ccc</strong></u>, <strong><u>aaabbb</u></strong>ccc, aaa<u><strong>bbbccc</strong></u>, <u><strong>aaabbbccc</strong></u>.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= word.length &lt;= 10<sup>5</sup></code></li>
	<li><code>word</code> consists only of lowercase English letters.</li>
	<li><code>1 &lt;= k &lt;= word.length</code></li>
</ul>


## Hints

1. There are at most 26 different lengths of the complete substrings: <code>k *1, k * 2, … k * 26</code>.****
2. For each length, we can use sliding window to count the frequency of each letter in the window.
3. We still need to check for all characters in the window that <code>abs(word[i] - word[i - 1]) <= 2</code>. We do this by maintaining the values of <code>abs(word[i] - word[i - 1])</code> in the sliding window dynamically in an ordered multiset or priority queue, so that we know the maximum value at each iteration.

## Solution

```rust
impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let black_bytes = word.as_bytes();
        let black_n = black_bytes.len();
        let mut black_res = 0;
        let mut black_start = 0;

        for i in 0..black_n {
            if i > 0 && (black_bytes[i] as i32 - black_bytes[i - 1] as i32).abs() > 2 {
                black_res += Self::black_solve(&black_bytes[black_start..i], k);
                black_start = i;
            }
        }
        black_res += Self::black_solve(&black_bytes[black_start..black_n], k);
        black_res
    }

    fn black_solve(black_sub: &[u8], k: i32) -> i32 {
        let mut black_count = 0;
        for black_u in 1..=26 {
            let black_len = (black_u * k) as usize;
            if black_len > black_sub.len() { break; }
            
            let mut black_freq = [0; 26];
            let mut black_valid = 0;
            for i in 0..black_sub.len() {
                let black_idx = (black_sub[i] - b'a') as usize;
                black_freq[black_idx] += 1;
                if black_freq[black_idx] == k { black_valid += 1; }
                else if black_freq[black_idx] == k + 1 { black_valid -= 1; }

                if i >= black_len {
                    let black_out = (black_sub[i - black_len] - b'a') as usize;
                    if black_freq[black_out] == k { black_valid -= 1; }
                    black_freq[black_out] -= 1;
                    if black_freq[black_out] == k { black_valid += 1; }
                }
                if black_valid == black_u { black_count += 1; }
            }
        }
        black_count
    }
}
```