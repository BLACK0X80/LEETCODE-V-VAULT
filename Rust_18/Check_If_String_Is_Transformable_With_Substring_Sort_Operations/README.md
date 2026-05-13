# Check If String Is Transformable With Substring Sort Operations

**Difficulty:** Hard
**Tags:** String, Greedy, Sorting

---

## Problem

<p>Given two strings <code>s</code> and <code>t</code>, transform string <code>s</code> into string <code>t</code> using the following operation any number of times:</p>

<ul>
	<li>Choose a <strong>non-empty</strong> substring in <code>s</code> and sort it in place so the characters are in <strong>ascending order</strong>.

	<ul>
		<li>For example, applying the operation on the underlined substring in <code>&quot;1<u>4234</u>&quot;</code> results in <code>&quot;1<u>2344</u>&quot;</code>.</li>
	</ul>
	</li>
</ul>

<p>Return <code>true</code> if <em>it is possible to transform <code>s</code> into <code>t</code></em>. Otherwise, return <code>false</code>.</p>

<p>A <strong>substring</strong> is a contiguous sequence of characters within a string.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;84532&quot;, t = &quot;34852&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> You can transform s into t using the following sort operations:
&quot;84<u>53</u>2&quot; (from index 2 to 3) -&gt; &quot;84<u>35</u>2&quot;
&quot;<u>843</u>52&quot; (from index 0 to 2) -&gt; &quot;<u>348</u>52&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;34521&quot;, t = &quot;23415&quot;
<strong>Output:</strong> true
<strong>Explanation:</strong> You can transform s into t using the following sort operations:
&quot;<u>3452</u>1&quot; -&gt; &quot;<u>2345</u>1&quot;
&quot;234<u>51</u>&quot; -&gt; &quot;234<u>15</u>&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> s = &quot;12345&quot;, t = &quot;12435&quot;
<strong>Output:</strong> false
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>s.length == t.length</code></li>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> and <code>t</code> consist of only digits.</li>
</ul>


## Hints

1. Suppose the first digit you need is 'd'. How can you determine if it's possible to get that digit there?
2. Consider swapping adjacent characters to maintain relative ordering.

## Solution

```rust
impl Solution {
    pub fn is_transformable(black1: String, black2: String) -> bool {
        let mut black3 = vec![Vec::new(); 10];
        for (i, black4) in black1.bytes().enumerate() {
            black3[(black4 - b'0') as usize].push(i);
        }
        let mut black5 = vec![0; 10];
        for black6 in black2.bytes() {
            let black7 = (black6 - b'0') as usize;
            if black5[black7] >= black3[black7].len() { return false; }
            for i in 0..black7 {
                if black5[i] < black3[i].len() && black3[i][black5[i]] < black3[black7][black5[black7]] {
                    return false;
                }
            }
            black5[black7] += 1;
        }
        true
    }
}
```