# Longest Chunked Palindrome Decomposition

**Difficulty:** Hard
**Tags:** Two Pointers, String, Dynamic Programming, Greedy, Rolling Hash, Hash Function

---

## Problem

<p>You are given a string <code>text</code>. You should split it to k substrings <code>(subtext<sub>1</sub>, subtext<sub>2</sub>, ..., subtext<sub>k</sub>)</code> such that:</p>

<ul>
	<li><code>subtext<sub>i</sub></code> is a <strong>non-empty</strong> string.</li>
	<li>The concatenation of all the substrings is equal to <code>text</code> (i.e., <code>subtext<sub>1</sub> + subtext<sub>2</sub> + ... + subtext<sub>k</sub> == text</code>).</li>
	<li><code>subtext<sub>i</sub> == subtext<sub>k - i + 1</sub></code> for all valid values of <code>i</code> (i.e., <code>1 &lt;= i &lt;= k</code>).</li>
</ul>

<p>Return the largest possible value of <code>k</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;ghiabcdefhelloadamhelloabcdefghi&quot;
<strong>Output:</strong> 7
<strong>Explanation:</strong> We can split the string on &quot;(ghi)(abcdef)(hello)(adam)(hello)(abcdef)(ghi)&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;merchant&quot;
<strong>Output:</strong> 1
<strong>Explanation:</strong> We can split the string on &quot;(merchant)&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> text = &quot;antaprezatepzapreanta&quot;
<strong>Output:</strong> 11
<strong>Explanation:</strong> We can split the string on &quot;(a)(nt)(a)(pre)(za)(tep)(za)(pre)(a)(nt)(a)&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= text.length &lt;= 1000</code></li>
	<li><code>text</code> consists only of lowercase English characters.</li>
</ul>


## Hints

1. Using a rolling hash, we can quickly check whether two strings are equal.
2. Use that as the basis of a dp.

## Solution

```rust
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let b = text.as_bytes();
        let n = b.len();
        
        for l in 1..=n/2 {
            if b[..l] == b[n-l..] {
                return 2 + Solution::longest_decomposition(
                    String::from_utf8(b[l..n-l].to_vec()).unwrap()
                );
            }
        }
        
        if n > 0 { 1 } else { 0 }
    }
}
```