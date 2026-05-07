# Three Equal Parts

**Difficulty:** Hard
**Tags:** Array, Math

---

## Problem

<p>You are given an array <code>arr</code> which consists of only zeros and ones, divide the array into <strong>three non-empty parts</strong> such that all of these parts represent the same binary value.</p>

<p>If it is possible, return any <code>[i, j]</code> with <code>i + 1 &lt; j</code>, such that:</p>

<ul>
	<li><code>arr[0], arr[1], ..., arr[i]</code> is the first part,</li>
	<li><code>arr[i + 1], arr[i + 2], ..., arr[j - 1]</code> is the second part, and</li>
	<li><code>arr[j], arr[j + 1], ..., arr[arr.length - 1]</code> is the third part.</li>
	<li>All three parts have equal binary values.</li>
</ul>

<p>If it is not possible, return <code>[-1, -1]</code>.</p>

<p>Note that the entire part is used when considering what binary value it represents. For example, <code>[1,1,0]</code> represents <code>6</code> in decimal, not <code>3</code>. Also, leading zeros <strong>are allowed</strong>, so <code>[0,1,1]</code> and <code>[1,1]</code> represent the same value.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> arr = [1,0,1,0,1]
<strong>Output:</strong> [0,3]
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> arr = [1,1,0,1,1]
<strong>Output:</strong> [-1,-1]
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> arr = [1,1,0,0,1]
<strong>Output:</strong> [0,2]
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= arr.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>arr[i]</code> is <code>0</code> or <code>1</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let black_total_ones = arr.iter().filter(|&&x| x == 1).count();
        if black_total_ones == 0 { return vec![0, 2]; }
        if black_total_ones % 3 != 0 { return vec![-1, -1]; }
        
        let black_ones_per_part = black_total_ones / 3;
        let (mut black_first, mut black_second, mut black_third) = (0, 0, 0);
        let mut black_count = 0;
        
        for i in 0..arr.len() {
            if arr[i] == 1 {
                black_count += 1;
                if black_count == 1 { black_first = i; }
                else if black_count == black_ones_per_part + 1 { black_second = i; }
                else if black_count == 2 * black_ones_per_part + 1 { black_third = i; }
            }
        }
        
        let black_len = arr.len() - black_third;
        if black_first + black_len <= black_second && black_second + black_len <= black_third {
            for i in 0..black_len {
                if arr[black_first + i] != arr[black_second + i] || arr[black_first + i] != arr[black_third + i] {
                    return vec![-1, -1];
                }
            }
            return vec![(black_first + black_len - 1) as i32, (black_second + black_len) as i32];
        }
        
        vec![-1, -1]
    }
}
```