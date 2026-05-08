# Longest Common Prefix of K Strings After Removal

**Difficulty:** Hard
**Tags:** Array, String, Trie

---

## Problem

<p>You are given an array of strings <code>words</code> and an integer <code>k</code>.</p>

<p>For each index <code>i</code> in the range <code>[0, words.length - 1]</code>, find the <strong>length</strong> of the <strong>longest common <span data-keyword="string-prefix">prefix</span></strong> among any <code>k</code> strings (selected at <strong>distinct indices</strong>) from the remaining array after removing the <code>i<sup>th</sup></code> element.</p>

<p>Return an array <code>answer</code>, where <code>answer[i]</code> is the answer for <code>i<sup>th</sup></code> element. If removing the <code>i<sup>th</sup></code> element leaves the array with fewer than <code>k</code> strings, <code>answer[i]</code> is 0.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;jump&quot;,&quot;run&quot;,&quot;run&quot;,&quot;jump&quot;,&quot;run&quot;], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[3,4,4,3,4]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Removing index 0 (<code>&quot;jump&quot;</code>):

	<ul>
		<li><code>words</code> becomes: <code>[&quot;run&quot;, &quot;run&quot;, &quot;jump&quot;, &quot;run&quot;]</code>. <code>&quot;run&quot;</code> occurs 3 times. Choosing any two gives the longest common prefix <code>&quot;run&quot;</code> (length 3).</li>
	</ul>
	</li>
	<li>Removing index 1 (<code>&quot;run&quot;</code>):
	<ul>
		<li><code>words</code> becomes: <code>[&quot;jump&quot;, &quot;run&quot;, &quot;jump&quot;, &quot;run&quot;]</code>. <code>&quot;jump&quot;</code> occurs twice. Choosing these two gives the longest common prefix <code>&quot;jump&quot;</code> (length 4).</li>
	</ul>
	</li>
	<li>Removing index 2 (<code>&quot;run&quot;</code>):
	<ul>
		<li><code>words</code> becomes: <code>[&quot;jump&quot;, &quot;run&quot;, &quot;jump&quot;, &quot;run&quot;]</code>. <code>&quot;jump&quot;</code> occurs twice. Choosing these two gives the longest common prefix <code>&quot;jump&quot;</code> (length 4).</li>
	</ul>
	</li>
	<li>Removing index 3 (<code>&quot;jump&quot;</code>):
	<ul>
		<li><code>words</code> becomes: <code>[&quot;jump&quot;, &quot;run&quot;, &quot;run&quot;, &quot;run&quot;]</code>. <code>&quot;run&quot;</code> occurs 3 times. Choosing any two gives the longest common prefix <code>&quot;run&quot;</code> (length 3).</li>
	</ul>
	</li>
	<li>Removing index 4 (&quot;run&quot;):
	<ul>
		<li><code>words</code> becomes: <code>[&quot;jump&quot;, &quot;run&quot;, &quot;run&quot;, &quot;jump&quot;]</code>. <code>&quot;jump&quot;</code> occurs twice. Choosing these two gives the longest common prefix <code>&quot;jump&quot;</code> (length 4).</li>
	</ul>
	</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">words = [&quot;dog&quot;,&quot;racer&quot;,&quot;car&quot;], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">[0,0,0]</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Removing any index results in an answer of 0.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= k &lt;= words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 10<sup>4</sup></code></li>
	<li><code>words[i]</code> consists of lowercase English letters.</li>
	<li>The sum of <code>words[i].length</code> is smaller than or equal <code>10<sup>5</sup></code>.</li>
</ul>


## Hints

1. Use a trie to store all the strings initially.
2. For each node in the trie, maintain the count of paths ending there.
3. For each <code>arr[i]</code>, remove it from the trie and update the counts.
4. During evaluation, find the innermost node with at least <code>k</code> paths ending there.
5. Use a multiset or similar structure to handle updates efficiently.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_common_prefix(black_words: Vec<String>, black_k: i32) -> Vec<i32> {
        let black_n = black_words.len();
        let mut black_ans = vec![0; black_n];
        let mut black_mx_len = 0;
        let mut black_mp = HashMap::new();

        for black_w in &black_words {
            black_mx_len = black_mx_len.max(black_w.len());
            let mut black_pref = String::new();
            for black_c in black_w.chars() {
                black_pref.push(black_c);
                *black_mp.entry(black_pref.clone()).or_insert(0) += 1;
            }
        }

        let mut black_mx_freq_str = vec![String::new(); black_mx_len + 1];
        let mut black_mx_freq = vec![0; black_mx_len + 1];
        let mut black_sec_mx = vec![0; black_mx_len + 1];

        for (black_str, black_freq) in black_mp {
            let black_l = black_str.len();
            if black_freq > black_mx_freq[black_l] {
                black_sec_mx[black_l] = black_mx_freq[black_l];
                black_mx_freq[black_l] = black_freq;
                black_mx_freq_str[black_l] = black_str;
            } else if black_freq > black_sec_mx[black_l] {
                black_sec_mx[black_l] = black_freq;
            }
        }

        for black_i in 0..black_n {
            let black_curr = &black_words[black_i];
            let (mut black_lo, mut black_hi) = (0, black_mx_len + 1);
            while black_lo + 1 < black_hi {
                let black_mid = (black_lo + black_hi) >> 1;
                let black_f;
                if black_mid <= black_curr.len() && &black_curr[..black_mid] == black_mx_freq_str[black_mid] {
                    black_f = (black_mx_freq[black_mid] - 1).max(black_sec_mx[black_mid]);
                } else {
                    black_f = black_mx_freq[black_mid];
                }

                if black_f < black_k {
                    black_hi = black_mid;
                } else {
                    black_lo = black_mid;
                }
            }
            black_ans[black_i] = black_lo as i32;
        }

        black_ans
    }
}
```