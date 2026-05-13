# Count Prefix and Suffix Pairs II

**Difficulty:** Hard
**Tags:** Array, String, Trie, Rolling Hash, String Matching, Hash Function

---

## Problem

<p>You are given a <strong>0-indexed</strong> string array <code>words</code>.</p>

<p>Let&#39;s define a <strong>boolean</strong> function <code>isPrefixAndSuffix</code> that takes two strings, <code>str1</code> and <code>str2</code>:</p>

<ul>
	<li><code>isPrefixAndSuffix(str1, str2)</code> returns <code>true</code> if <code>str1</code> is <strong>both</strong> a <span data-keyword="string-prefix">prefix</span> and a <span data-keyword="string-suffix">suffix</span> of <code>str2</code>, and <code>false</code> otherwise.</li>
</ul>

<p>For example, <code>isPrefixAndSuffix(&quot;aba&quot;, &quot;ababa&quot;)</code> is <code>true</code> because <code>&quot;aba&quot;</code> is a prefix of <code>&quot;ababa&quot;</code> and also a suffix, but <code>isPrefixAndSuffix(&quot;abc&quot;, &quot;abcd&quot;)</code> is <code>false</code>.</p>

<p>Return <em>an integer denoting the <strong>number</strong> of index pairs </em><code>(i<em>, </em>j)</code><em> such that </em><code>i &lt; j</code><em>, and </em><code>isPrefixAndSuffix(words[i], words[j])</code><em> is </em><code>true</code><em>.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;a&quot;,&quot;aba&quot;,&quot;ababa&quot;,&quot;aa&quot;]
<strong>Output:</strong> 4
<strong>Explanation:</strong> In this example, the counted index pairs are:
i = 0 and j = 1 because isPrefixAndSuffix(&quot;a&quot;, &quot;aba&quot;) is true.
i = 0 and j = 2 because isPrefixAndSuffix(&quot;a&quot;, &quot;ababa&quot;) is true.
i = 0 and j = 3 because isPrefixAndSuffix(&quot;a&quot;, &quot;aa&quot;) is true.
i = 1 and j = 2 because isPrefixAndSuffix(&quot;aba&quot;, &quot;ababa&quot;) is true.
Therefore, the answer is 4.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;pa&quot;,&quot;papa&quot;,&quot;ma&quot;,&quot;mama&quot;]
<strong>Output:</strong> 2
<strong>Explanation:</strong> In this example, the counted index pairs are:
i = 0 and j = 1 because isPrefixAndSuffix(&quot;pa&quot;, &quot;papa&quot;) is true.
i = 2 and j = 3 because isPrefixAndSuffix(&quot;ma&quot;, &quot;mama&quot;) is true.
Therefore, the answer is 2.  </pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> words = [&quot;abab&quot;,&quot;ab&quot;]
<strong>Output:</strong> 0
<strong>Explanation: </strong>In this example, the only valid index pair is i = 0 and j = 1, and isPrefixAndSuffix(&quot;abab&quot;, &quot;ab&quot;) is false.
Therefore, the answer is 0.</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= words.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= words[i].length &lt;= 10<sup>5</sup></code></li>
	<li><code>words[i]</code> consists only of lowercase English letters.</li>
	<li>The sum of the lengths of all <code>words[i]</code> does not exceed <code>5 * 10<sup>5</sup></code>.</li>
</ul>


## Hints

1. We can use a trie to solve it.
2. Process all <code>words[i]</code> from left to right. The trie stores the pair <code>(words[i][j], words[i][words[i].length - j - 1])</code> as a single character; we process all the words in this way.
3. During insertion, keep a counter in each trie node, as in a normal trie. If the current node is the end of a word (namely, the pair on that node is <code>(words[i][words[i].length - 1], words[i][0])</code>), increase the node's counter by <code>1</code>.
4. From left to right, insert each word into the trie, and increase our final result by each node's counter when going down the trie during insertion. This means there was at least one word that is both a prefix and a suffix of the current word before.

## Solution

```rust
use std::collections::HashMap;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut children: Vec<HashMap<(u8,u8), usize>> = vec![HashMap::new()];
        let mut cnt: Vec<i64> = vec![0];
        let mut ans = 0i64;

        for word in &words {
            let w = word.as_bytes();
            let m = w.len();
            let mut node = 0usize;
            ans += cnt[node];
            for i in 0..m {
                let key = (w[i], w[m-1-i]);
                let next = if let Some(&n) = children[node].get(&key) {
                    n
                } else {
                    let n = children.len();
                    children.push(HashMap::new());
                    cnt.push(0);
                    children[node].insert(key, n);
                    n
                };
                node = next;
                ans += cnt[node];
            }
            cnt[node] += 1;
        }
        ans
    }
}
```