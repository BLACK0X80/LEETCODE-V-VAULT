# Count Beautiful Substrings II

**Difficulty:** Hard
**Tags:** Hash Table, Math, String, Number Theory, Prefix Sum

---

## Problem

<p>You are given a string <code>s</code> and a positive integer <code>k</code>.</p>

<p>Let <code>vowels</code> and <code>consonants</code> be the number of vowels and consonants in a string.</p>

<p>A string is <strong>beautiful</strong> if:</p>

<ul>
	<li><code>vowels == consonants</code>.</li>
	<li><code>(vowels * consonants) % k == 0</code>, in other terms the multiplication of <code>vowels</code> and <code>consonants</code> is divisible by <code>k</code>.</li>
</ul>

<p>Return <em>the number of <strong>non-empty beautiful substrings</strong> in the given string</em> <code>s</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters in a string.</p>

<p><strong>Vowel letters</strong> in English are <code>&#39;a&#39;</code>, <code>&#39;e&#39;</code>, <code>&#39;i&#39;</code>, <code>&#39;o&#39;</code>, and <code>&#39;u&#39;</code>.</p>

<p><strong>Consonant letters</strong> in English are every letter except vowels.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;baeyh&quot;, k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 beautiful substrings in the given string.
- Substring &quot;b<u>aeyh</u>&quot;, vowels = 2 ([&quot;a&quot;,e&quot;]), consonants = 2 ([&quot;y&quot;,&quot;h&quot;]).
You can see that string &quot;aeyh&quot; is beautiful as vowels == consonants and vowels * consonants % k == 0.
- Substring &quot;<u>baey</u>h&quot;, vowels = 2 ([&quot;a&quot;,e&quot;]), consonants = 2 ([&quot;b&quot;,&quot;y&quot;]).
You can see that string &quot;baey&quot; is beautiful as vowels == consonants and vowels * consonants % k == 0.
It can be shown that there are only 2 beautiful substrings in the given string.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abba&quot;, k = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 beautiful substrings in the given string.
- Substring &quot;<u>ab</u>ba&quot;, vowels = 1 ([&quot;a&quot;]), consonants = 1 ([&quot;b&quot;]).
- Substring &quot;ab<u>ba</u>&quot;, vowels = 1 ([&quot;a&quot;]), consonants = 1 ([&quot;b&quot;]).
- Substring &quot;<u>abba</u>&quot;, vowels = 2 ([&quot;a&quot;,&quot;a&quot;]), consonants = 2 ([&quot;b&quot;,&quot;b&quot;]).
It can be shown that there are only 3 beautiful substrings in the given string.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;bcdf&quot;, k = 1
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no beautiful substrings in the given string.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= k &lt;= 1000</code></li>
	<li><code>s</code> consists of only English lowercase letters.</li>
</ul>


## Hints

1. For the given <code>k</code> find all the <code>x</code> integers such that <code>x^2 % k == 0</code>. Notice, that there aren’t many such candidates.
2. We can iterate over all such <code>x</codes> values and count the number of substrings such that <code>vowels == consonants == x</code>.
3. This can be done with prefix sums and hash map.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn beautiful_substrings(black_s: String, black_k: i32) -> i64 {
        let black_minimal = {
            let mut black_temp_k = black_k;
            let mut black_min_v = 1;
            let mut i = 2;
            while i * i <= black_temp_k {
                if black_temp_k % i == 0 {
                    let mut black_p = 0;
                    while black_temp_k % i == 0 {
                        black_temp_k /= i;
                        black_p += 1;
                    }
                    black_min_v *= i.pow((black_p + 1) / 2);
                }
                i += 1;
            }
            if black_temp_k > 1 { black_min_v *= black_temp_k; }
            black_min_v * 2
        };

        let mut black_seen = vec![HashMap::new(); black_minimal as usize];
        black_seen[0].insert(0, 1i64);

        let (mut black_psum, mut black_res) = (0i32, 0i64);
        let black_bytes = black_s.as_bytes();

        for i in 0..black_bytes.len() {
            let black_c = black_bytes[i];
            black_psum += if b"aeiou".contains(&black_c) { 1 } else { -1 };

            let black_group = (i + 1) % black_minimal as usize;
            if let Some(&black_count) = black_seen[black_group].get(&black_psum) {
                black_res += black_count;
            }
            *black_seen[black_group].entry(black_psum).or_insert(0) += 1;
        }

        black_res
    }
}
```