# Process String with Special Operations II

**Difficulty:** Hard
**Tags:** String, Simulation

---

## Problem

<p>You are given a string <code>s</code> consisting of lowercase English letters and the special characters: <code>&#39;*&#39;</code>, <code>&#39;#&#39;</code>, and <code>&#39;%&#39;</code>.</p>

<p>You are also given an integer <code>k</code>.</p>

<p>Build a new string <code>result</code> by processing <code>s</code> according to the following rules from left to right:</p>

<ul>
	<li>If the letter is a <strong>lowercase</strong> English letter append it to <code>result</code>.</li>
	<li>A <code>&#39;*&#39;</code> <strong>removes</strong> the last character from <code>result</code>, if it exists.</li>
	<li>A <code>&#39;#&#39;</code> <strong>duplicates</strong> the current <code>result</code> and <strong>appends</strong> it to itself.</li>
	<li>A <code>&#39;%&#39;</code> <strong>reverses</strong> the current <code>result</code>.</li>
</ul>

<p>Return the <code>k<sup>th</sup></code> character of the final string <code>result</code>. If <code>k</code> is out of the bounds of <code>result</code>, return <code>&#39;.&#39;</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;a#b%*&quot;, k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;a&quot;</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>s[i]</code></th>
			<th style="border: 1px solid black;">Operation</th>
			<th style="border: 1px solid black;">Current <code>result</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>&#39;a&#39;</code></td>
			<td style="border: 1px solid black;">Append <code>&#39;a&#39;</code></td>
			<td style="border: 1px solid black;"><code>&quot;a&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>&#39;#&#39;</code></td>
			<td style="border: 1px solid black;">Duplicate <code>result</code></td>
			<td style="border: 1px solid black;"><code>&quot;aa&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;"><code>&#39;b&#39;</code></td>
			<td style="border: 1px solid black;">Append <code>&#39;b&#39;</code></td>
			<td style="border: 1px solid black;"><code>&quot;aab&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;"><code>&#39;%&#39;</code></td>
			<td style="border: 1px solid black;">Reverse <code>result</code></td>
			<td style="border: 1px solid black;"><code>&quot;baa&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;"><code>&#39;*&#39;</code></td>
			<td style="border: 1px solid black;">Remove the last character</td>
			<td style="border: 1px solid black;"><code>&quot;ba&quot;</code></td>
		</tr>
	</tbody>
</table>

<p>The final <code>result</code> is <code>&quot;ba&quot;</code>. The character at index <code>k = 1</code> is <code>&#39;a&#39;</code>.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;cd%#*#&quot;, k = 3</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;d&quot;</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>s[i]</code></th>
			<th style="border: 1px solid black;">Operation</th>
			<th style="border: 1px solid black;">Current <code>result</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>&#39;c&#39;</code></td>
			<td style="border: 1px solid black;">Append <code>&#39;c&#39;</code></td>
			<td style="border: 1px solid black;"><code>&quot;c&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>&#39;d&#39;</code></td>
			<td style="border: 1px solid black;">Append <code>&#39;d&#39;</code></td>
			<td style="border: 1px solid black;"><code>&quot;cd&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;"><code>&#39;%&#39;</code></td>
			<td style="border: 1px solid black;">Reverse <code>result</code></td>
			<td style="border: 1px solid black;"><code>&quot;dc&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">3</td>
			<td style="border: 1px solid black;"><code>&#39;#&#39;</code></td>
			<td style="border: 1px solid black;">Duplicate <code>result</code></td>
			<td style="border: 1px solid black;"><code>&quot;dcdc&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">4</td>
			<td style="border: 1px solid black;"><code>&#39;*&#39;</code></td>
			<td style="border: 1px solid black;">Remove the last character</td>
			<td style="border: 1px solid black;"><code>&quot;dcd&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">5</td>
			<td style="border: 1px solid black;"><code>&#39;#&#39;</code></td>
			<td style="border: 1px solid black;">Duplicate <code>result</code></td>
			<td style="border: 1px solid black;"><code>&quot;dcddcd&quot;</code></td>
		</tr>
	</tbody>
</table>

<p>The final <code>result</code> is <code>&quot;dcddcd&quot;</code>. The character at index <code>k = 3</code> is <code>&#39;d&#39;</code>.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;z*#&quot;, k = 0</span></p>

<p><strong>Output:</strong> <span class="example-io">&quot;.&quot;</span></p>

<p><strong>Explanation:</strong></p>

<table style="border: 1px solid black;">
	<thead>
		<tr>
			<th style="border: 1px solid black;"><code>i</code></th>
			<th style="border: 1px solid black;"><code>s[i]</code></th>
			<th style="border: 1px solid black;">Operation</th>
			<th style="border: 1px solid black;">Current <code>result</code></th>
		</tr>
	</thead>
	<tbody>
		<tr>
			<td style="border: 1px solid black;">0</td>
			<td style="border: 1px solid black;"><code>&#39;z&#39;</code></td>
			<td style="border: 1px solid black;">Append <code>&#39;z&#39;</code></td>
			<td style="border: 1px solid black;"><code>&quot;z&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">1</td>
			<td style="border: 1px solid black;"><code>&#39;*&#39;</code></td>
			<td style="border: 1px solid black;">Remove the last character</td>
			<td style="border: 1px solid black;"><code>&quot;&quot;</code></td>
		</tr>
		<tr>
			<td style="border: 1px solid black;">2</td>
			<td style="border: 1px solid black;"><code>&#39;#&#39;</code></td>
			<td style="border: 1px solid black;">Duplicate the string</td>
			<td style="border: 1px solid black;"><code>&quot;&quot;</code></td>
		</tr>
	</tbody>
</table>

<p>The final <code>result</code> is <code>&quot;&quot;</code>. Since index <code>k = 0</code> is out of bounds, the output is <code>&#39;.&#39;</code>.</p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of only lowercase English letters and special characters <code>&#39;*&#39;</code>, <code>&#39;#&#39;</code>, and <code>&#39;%&#39;</code>.</li>
	<li><code>0 &lt;= k &lt;= 10<sup>15</sup></code></li>
	<li>The length of <code>result</code> after processing <code>s</code> will not exceed <code>10<sup>15</sup></code>.</li>
</ul>


## Hints

1. Track the length of the string after each operation on <code>s</code>.
2. Walk backwards through <code>s</code>, undoing each # by using modulus on the tracked lengths, and undoing each % by mirroring across the midpoint, to pinpoint the <code>k</code>th character.

## Solution

```rust
impl Solution {
    pub fn process_str(black1: String, mut black2: i64) -> char {
        let mut black3 = vec![0i64];
        let black4: Vec<char> = black1.chars().collect();
        
        for &black5 in &black4 {
            let mut black6 = *black3.last().unwrap();
            let black7 = match black5 {
                '*' => if black6 > 0 { black6 - 1 } else { 0 },
                '#' => if black6 > 1e15 as i64 { black6 } else { black6 * 2 },
                '%' => black6,
                _ => black6 + 1,
            };
            black3.push(black7);
        }

        if black2 < 0 || black2 >= *black3.last().unwrap() {
            return '.';
        }

        for black8 in (0..black4.len()).rev() {
            let black9 = black4[black8];
            let black10 = black3[black8];

            match black9 {
                '#' => {
                    if black10 > 0 {
                        black2 %= black10;
                    }
                }
                '%' => {
                    black2 = black10 - 1 - black2;
                }
                '*' => {
                    
                }
                black11 if black11.is_ascii_lowercase() => {
                    if black2 == black10 {
                        return black11;
                    }
                }
                _ => {}
            }
        }
        '.'
    }
}
```