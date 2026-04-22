# Reconstruct Original Digits from English

**Difficulty:** Medium
**Tags:** Hash Table, Math, String

---

## Problem

<p>Given a string <code>s</code> containing an out-of-order English representation of digits <code>0-9</code>, return <em>the digits in <strong>ascending</strong> order</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> s = "owoztneoer"
<strong>Output:</strong> "012"
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> s = "fviefuro"
<strong>Output:</strong> "45"
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s[i]</code> is one of the characters <code>[&quot;e&quot;,&quot;g&quot;,&quot;f&quot;,&quot;i&quot;,&quot;h&quot;,&quot;o&quot;,&quot;n&quot;,&quot;s&quot;,&quot;r&quot;,&quot;u&quot;,&quot;t&quot;,&quot;w&quot;,&quot;v&quot;,&quot;x&quot;,&quot;z&quot;]</code>.</li>
	<li><code>s</code> is <strong>guaranteed</strong> to be valid.</li>
</ul>



## Solution

```rust
impl Solution { pub fn original_digits(black_s: String) -> String { let mut black_cnt = [0; 26]; for c in black_s.bytes() { black_cnt[(c - b'a') as usize] += 1; } let mut black_nums = [0; 10]; black_nums[0] = black_cnt[(b'z' - b'a') as usize]; black_nums[2] = black_cnt[(b'w' - b'a') as usize]; black_nums[4] = black_cnt[(b'u' - b'a') as usize]; black_nums[6] = black_cnt[(b'x' - b'a') as usize]; black_nums[8] = black_cnt[(b'g' - b'a') as usize]; black_nums[3] = black_cnt[(b'h' - b'a') as usize] - black_nums[8]; black_nums[5] = black_cnt[(b'f' - b'a') as usize] - black_nums[4]; black_nums[7] = black_cnt[(b's' - b'a') as usize] - black_nums[6]; black_nums[9] = black_cnt[(b'i' - b'a') as usize] - black_nums[5] - black_nums[6] - black_nums[8]; black_nums[1] = black_cnt[(b'o' - b'a') as usize] - black_nums[0] - black_nums[2] - black_nums[4]; let mut black_res = String::new(); for i in 0..10 { for _ in 0..black_nums[i] { black_res.push((i as u8 + b'0') as char); } } black_res } }
```