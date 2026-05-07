# Maximum Number of Non-Overlapping Substrings

**Difficulty:** Hard
**Tags:** Hash Table, String, Greedy, Sorting

---

## Problem

<p>Given a string <code>s</code> of lowercase letters, you need to find the maximum number of <strong>non-empty</strong> substrings of <code>s</code> that meet the following conditions:</p>

<ol>
	<li>The substrings do not overlap, that is for any two substrings <code>s[i..j]</code> and <code>s[x..y]</code>, either <code>j &lt; x</code> or <code>i &gt; y</code> is true.</li>
	<li>A substring that contains a certain character <code>c</code> must also contain all occurrences of <code>c</code>.</li>
</ol>

<p>Find <em>the maximum number of substrings that meet the above conditions</em>. If there are multiple solutions with the same number of substrings, <em>return the one with minimum total length. </em>It can be shown that there exists a unique solution of minimum total length.</p>

<p>Notice that you can return the substrings in <strong>any</strong> order.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;adefaddaccc&quot;
<strong>Output:</strong> [&quot;e&quot;,&quot;f&quot;,&quot;ccc&quot;]
<b>Explanation:</b>&nbsp;The following are all the possible substrings that meet the conditions:
[
&nbsp; &quot;adefaddaccc&quot;
&nbsp; &quot;adefadda&quot;,
&nbsp; &quot;ef&quot;,
&nbsp; &quot;e&quot;,
  &quot;f&quot;,
&nbsp; &quot;ccc&quot;,
]
If we choose the first string, we cannot choose anything else and we&#39;d get only 1. If we choose &quot;adefadda&quot;, we are left with &quot;ccc&quot; which is the only one that doesn&#39;t overlap, thus obtaining 2 substrings. Notice also, that it&#39;s not optimal to choose &quot;ef&quot; since it can be split into two. Therefore, the optimal way is to choose [&quot;e&quot;,&quot;f&quot;,&quot;ccc&quot;] which gives us 3 substrings. No other solution of the same number of substrings exist.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abbaccd&quot;
<strong>Output:</strong> [&quot;d&quot;,&quot;bb&quot;,&quot;cc&quot;]
<b>Explanation: </b>Notice that while the set of substrings [&quot;d&quot;,&quot;abba&quot;,&quot;cc&quot;] also has length 3, it&#39;s considered incorrect since it has larger total length.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> contains only lowercase English letters.</li>
</ul>


## Hints

1. Notice that it's impossible for any two valid substrings to overlap unless one is inside another.
2. We can start by finding the starting and ending index for each character.
3. From these indices, we can form the substrings by expanding each character's range if necessary (if another character exists in the range with smaller/larger starting/ending index).
4. Sort the valid substrings by length and greedily take those with the smallest length, discarding the ones that overlap those we took.

## Solution

```rust
impl Solution {
    pub fn max_num_of_substrings(black1: String) -> Vec<String> {
        let black2 = black1.as_bytes();
        let (mut black3, mut black4) = (vec![usize::MAX; 26], vec![0; 26]);
        for (i, &b) in black2.iter().enumerate() {
            let black5 = (b - b'a') as usize;
            black3[black5] = black3[black5].min(i);
            black4[black5] = i;
        }
        let mut black6 = Vec::new();
        for i in 0..26 {
            if black3[i] == usize::MAX { continue; }
            let (mut black7, mut black8) = (black3[i], black4[i]);
            let mut j = black7;
            let mut black9 = true;
            while j <= black8 {
                let black10 = (black2[j] - b'a') as usize;
                if black3[black10] < black7 { black9 = false; break; }
                black8 = black8.max(black4[black10]);
                j += 1;
            }
            if black9 { black6.push((black7, black8)); }
        }
        black6.sort_unstable_by_key(|x| x.1);
        let (mut black11, mut black12) = (Vec::new(), -1isize);
        for (black13, black14) in black6 {
            if black13 as isize > black12 {
                black11.push(String::from_utf8(black2[black13..=black14].to_vec()).unwrap());
                black12 = black14 as isize;
            }
        }
        black11
    }
}
```