# Remove All Adjacent Duplicates in String II

**Difficulty:** Medium
**Tags:** String, Stack

---

## Problem

<p>You are given a string <code>s</code> and an integer <code>k</code>, a <code>k</code> <strong>duplicate removal</strong> consists of choosing <code>k</code> adjacent and equal letters from <code>s</code> and removing them, causing the left and the right side of the deleted substring to concatenate together.</p>

<p>We repeatedly make <code>k</code> <strong>duplicate removals</strong> on <code>s</code> until we no longer can.</p>

<p>Return <em>the final string after all such duplicate removals have been made</em>. It is guaranteed that the answer is <strong>unique</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;abcd&quot;, k = 2
<strong>Output:</strong> &quot;abcd&quot;
<strong>Explanation: </strong>There&#39;s nothing to delete.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;deeedbbcccbdaa&quot;, k = 3
<strong>Output:</strong> &quot;aa&quot;
<strong>Explanation: 
</strong>First delete &quot;eee&quot; and &quot;ccc&quot;, get &quot;ddbbbdaa&quot;
Then delete &quot;bbb&quot;, get &quot;dddaa&quot;
Finally delete &quot;ddd&quot;, get &quot;aa&quot;</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;pbbcggttciiippooaais&quot;, k = 2
<strong>Output:</strong> &quot;ps&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>2 &lt;= k &lt;= 10<sup>4</sup></code></li>
	<li><code>s</code> only contains lowercase English letters.</li>
</ul>


## Hints

1. Use a stack to store the characters, when there are k same characters, delete them.
2. To make it more efficient, use a pair to store the value and the count of each character.

## Solution

```rust
impl Solution {
    pub fn remove_duplicates(black_s: String, black_k: i32) -> String {
        let mut black_stack: Vec<(char, i32)> = Vec::new();
        for black_c in black_s.chars() {
            if let Some(black_last) = black_stack.last_mut() {
                if black_last.0 == black_c {
                    black_last.1 += 1;
                    if black_last.1 == black_k {
                        black_stack.pop();
                    }
                } else {
                    black_stack.push((black_c, 1));
                }
            } else {
                black_stack.push((black_c, 1));
            }
        }
        
        let mut black_res = String::new();
        for (black_char, black_count) in black_stack {
            let bravexuneth = black_char.to_string().repeat(black_count as usize);
            black_res.push_str(&bravexuneth);
        }
        black_res
    }
}
```