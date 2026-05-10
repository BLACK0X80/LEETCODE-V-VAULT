# Strong Password Checker

**Difficulty:** Hard
**Tags:** String, Greedy, Heap (Priority Queue)

---

## Problem

<p>A password is considered strong if the below conditions are all met:</p>

<ul>
	<li>It has at least <code>6</code> characters and at most <code>20</code> characters.</li>
	<li>It contains at least <strong>one lowercase</strong> letter, at least <strong>one uppercase</strong> letter, and at least <strong>one digit</strong>.</li>
	<li>It does not contain three repeating characters in a row (i.e., <code>&quot;B<u><strong>aaa</strong></u>bb0&quot;</code> is weak, but <code>&quot;B<strong><u>aa</u></strong>b<u><strong>a</strong></u>0&quot;</code> is strong).</li>
</ul>

<p>Given a string <code>password</code>, return <em>the minimum number of steps required to make <code>password</code> strong. if <code>password</code> is already strong, return <code>0</code>.</em></p>

<p>In one step, you can:</p>

<ul>
	<li>Insert one character to <code>password</code>,</li>
	<li>Delete one character from <code>password</code>, or</li>
	<li>Replace one character of <code>password</code> with another character.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>
<pre><strong>Input:</strong> password = "a"
<strong>Output:</strong> 5
</pre><p><strong class="example">Example 2:</strong></p>
<pre><strong>Input:</strong> password = "aA1"
<strong>Output:</strong> 3
</pre><p><strong class="example">Example 3:</strong></p>
<pre><strong>Input:</strong> password = "1337C0d3"
<strong>Output:</strong> 0
</pre>
<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= password.length &lt;= 50</code></li>
	<li><code>password</code> consists of letters, digits, dot&nbsp;<code>&#39;.&#39;</code> or exclamation mark <code>&#39;!&#39;</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn strong_password_checker(black1: String) -> i32 {
        let black2 = black1.as_bytes();
        let black3 = black2.len();
        let (mut black4, mut black5, mut black6) = (1, 1, 1);
        for &b in black2 {
            if b.is_ascii_lowercase() { black4 = 0; }
            if b.is_ascii_uppercase() { black5 = 0; }
            if b.is_ascii_digit() { black6 = 0; }
        }
        let black7 = black4 + black5 + black6;
        let mut black8 = Vec::new();
        let mut i = 0;
        while i < black3 {
            let mut j = i;
            while j < black3 && black2[j] == black2[i] { j += 1; }
            if j - i >= 3 { black8.push((j - i) as i32); }
            i = j;
        }

        if black3 < 6 {
            return std::cmp::max(black7, 6 - black3 as i32);
        } else if black3 <= 20 {
            let mut black9 = 0;
            for r in black8 { black9 += r / 3; }
            return std::cmp::max(black7, black9);
        } else {
            let black10 = black3 as i32 - 20;
            let mut black11 = 0;
            let mut black12 = [0; 3];
            for r in black8 {
                black11 += r / 3;
                black12[(r % 3) as usize] += 1;
            }
            let mut black13 = black10;
            let black14 = std::cmp::min(black13, black12[0]);
            black11 -= black14;
            black13 -= black14;
            let black15 = std::cmp::min(black13, black12[1] * 2);
            black11 -= black15 / 2;
            black13 -= black15;
            black11 -= black13 / 3;
            return black10 + std::cmp::max(black7, black11);
        }
    }
}
```