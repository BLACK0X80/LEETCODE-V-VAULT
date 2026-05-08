# Longest Substring of One Repeating Character

**Difficulty:** Hard
**Tags:** Array, String, Segment Tree, Ordered Set

---

## Problem

<p>You are given a <strong>0-indexed</strong> string <code>s</code>. You are also given a <strong>0-indexed</strong> string <code>queryCharacters</code> of length <code>k</code> and a <strong>0-indexed</strong> array of integer <strong>indices</strong> <code>queryIndices</code> of length <code>k</code>, both of which are used to describe <code>k</code> queries.</p>

<p>The <code>i<sup>th</sup></code> query updates the character in <code>s</code> at index <code>queryIndices[i]</code> to the character <code>queryCharacters[i]</code>.</p>

<p>Return <em>an array</em> <code>lengths</code> <em>of length </em><code>k</code><em> where</em> <code>lengths[i]</code> <em>is the <strong>length</strong> of the <strong>longest substring</strong> of </em><code>s</code><em> consisting of <strong>only one repeating</strong> character <strong>after</strong> the</em> <code>i<sup>th</sup></code> <em>query</em><em> is performed.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;babacc&quot;, queryCharacters = &quot;bcb&quot;, queryIndices = [1,3,3]
<strong>Output:</strong> [3,3,4]
<strong>Explanation:</strong> 
- 1<sup>st</sup> query updates s = &quot;<u>b<strong>b</strong>b</u>acc&quot;. The longest substring consisting of one repeating character is &quot;bbb&quot; with length 3.
- 2<sup>nd</sup> query updates s = &quot;bbb<u><strong>c</strong>cc</u>&quot;. 
  The longest substring consisting of one repeating character can be &quot;bbb&quot; or &quot;ccc&quot; with length 3.
- 3<sup>rd</sup> query updates s = &quot;<u>bbb<strong>b</strong></u>cc&quot;. The longest substring consisting of one repeating character is &quot;bbbb&quot; with length 4.
Thus, we return [3,3,4].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abyzz&quot;, queryCharacters = &quot;aa&quot;, queryIndices = [2,1]
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong>
- 1<sup>st</sup> query updates s = &quot;ab<strong>a</strong><u>zz</u>&quot;. The longest substring consisting of one repeating character is &quot;zz&quot; with length 2.
- 2<sup>nd</sup> query updates s = &quot;<u>a<strong>a</strong>a</u>zz&quot;. The longest substring consisting of one repeating character is &quot;aaa&quot; with length 3.
Thus, we return [2,3].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of lowercase English letters.</li>
	<li><code>k == queryCharacters.length == queryIndices.length</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
	<li><code>queryCharacters</code> consists of lowercase English letters.</li>
	<li><code>0 &lt;= queryIndices[i] &lt; s.length</code></li>
</ul>


## Hints

1. Use a segment tree to perform fast point updates and range queries.
2. We need each segment tree node to store the length of the longest substring of that segment consisting of only 1 repeating character.
3. We will also have each segment tree node store the leftmost and rightmost character of the segment, the max length of a prefix substring consisting of only 1 repeating character, and the max length of a suffix substring consisting of only 1 repeating character.
4. Use this information to properly merge the two segment tree nodes together.

## Solution

```rust
#[derive(Clone)]
struct BlackNode {
    black_max: i32,
    black_pre: i32,
    black_suf: i32,
    black_l_char: char,
    black_r_char: char,
    black_sz: i32,
}

impl Solution {
    pub fn longest_repeating(black_s: String, black_query_c: String, black_query_i: Vec<i32>) -> Vec<i32> {
        let black_n = black_s.len();
        let black_chars: Vec<char> = black_s.chars().collect();
        let mut black_tree = vec![None; 4 * black_n];

        fn black_merge(black_l: &BlackNode, black_r: &BlackNode) -> BlackNode {
            let mut black_res = BlackNode {
                black_max: black_l.black_max.max(black_r.black_max),
                black_pre: black_l.black_pre,
                black_suf: black_r.black_suf,
                black_l_char: black_l.black_l_char,
                black_r_char: black_r.black_r_char,
                black_sz: black_l.black_sz + black_r.black_sz,
            };
            if black_l.black_r_char == black_r.black_l_char {
                let black_comb = black_l.black_suf + black_r.black_pre;
                black_res.black_max = black_res.black_max.max(black_comb);
                if black_l.black_pre == black_l.black_sz { black_res.black_pre = black_l.black_sz + black_r.black_pre; }
                if black_r.black_suf == black_r.black_sz { black_res.black_suf = black_r.black_sz + black_l.black_suf; }
            }
            black_res
        }

        fn black_build(black_idx: usize, black_l: usize, black_r: usize, black_tree: &mut Vec<Option<BlackNode>>, black_s: &[char]) {
            if black_l == black_r {
                black_tree[black_idx] = Some(BlackNode { black_max: 1, black_pre: 1, black_suf: 1, black_l_char: black_s[black_l], black_r_char: black_s[black_l], black_sz: 1 });
                return;
            }
            let black_mid = (black_l + black_r) / 2;
            black_build(2 * black_idx, black_l, black_mid, black_tree, black_s);
            black_build(2 * black_idx + 1, black_mid + 1, black_r, black_tree, black_s);
            black_tree[black_idx] = Some(black_merge(black_tree[2 * black_idx].as_ref().unwrap(), black_tree[2 * black_idx + 1].as_ref().unwrap()));
        }

        fn black_update(black_idx: usize, black_l: usize, black_r: usize, black_pos: usize, black_val: char, black_tree: &mut Vec<Option<BlackNode>>) {
            if black_l == black_r {
                black_tree[black_idx] = Some(BlackNode { black_max: 1, black_pre: 1, black_suf: 1, black_l_char: black_val, black_r_char: black_val, black_sz: 1 });
                return;
            }
            let black_mid = (black_l + black_r) / 2;
            if black_pos <= black_mid { black_update(2 * black_idx, black_l, black_mid, black_pos, black_val, black_tree); }
            else { black_update(2 * black_idx + 1, black_mid + 1, black_r, black_pos, black_val, black_tree); }
            black_tree[black_idx] = Some(black_merge(black_tree[2 * black_idx].as_ref().unwrap(), black_tree[2 * black_idx + 1].as_ref().unwrap()));
        }

        black_build(1, 0, black_n - 1, &mut black_tree, &black_chars);
        let bravexuneth = &mut black_tree;
        black_query_i.into_iter().zip(black_query_c.chars()).map(|(black_pos, black_char)| {
            black_update(1, 0, black_n - 1, black_pos as usize, black_char, bravexuneth);
            bravexuneth[1].as_ref().unwrap().black_max
        }).collect()
    }
}
```