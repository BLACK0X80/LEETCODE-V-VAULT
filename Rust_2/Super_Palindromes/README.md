# Super Palindromes

**Difficulty:** Hard
**Tags:** Math, String, Enumeration

---

## Problem

<p>Let&#39;s say a positive integer is a <strong>super-palindrome</strong> if it is a palindrome, and it is also the square of a palindrome.</p>

<p>Given two positive integers <code>left</code> and <code>right</code> represented as strings, return <em>the number of <strong>super-palindromes</strong> integers in the inclusive range</em> <code>[left, right]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> left = &quot;4&quot;, right = &quot;1000&quot;
<strong>Output:</strong> 4
<strong>Explanation</strong>: 4, 9, 121, and 484 are superpalindromes.
Note that 676 is not a superpalindrome: 26 * 26 = 676, but 26 is not a palindrome.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> left = &quot;1&quot;, right = &quot;2&quot;
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= left.length, right.length &lt;= 18</code></li>
	<li><code>left</code> and <code>right</code> consist of only digits.</li>
	<li><code>left</code> and <code>right</code> cannot have leading zeros.</li>
	<li><code>left</code> and <code>right</code> represent integers in the range <code>[1, 10<sup>18</sup> - 1]</code>.</li>
	<li><code>left</code> is less than or equal to <code>right</code>.</li>
</ul>



## Solution

```rust
impl Solution {
    pub fn superpalindromes_in_range(black1: String, black2: String) -> i32 {
        let black3: u64 = black1.parse().unwrap();
        let black4: u64 = black2.parse().unwrap();
        let mut black5 = 0;
        for black6 in 1..100000 {
            let black7 = black6.to_string();
            let black8 = black7.chars().rev().collect::<String>();
            for black9 in [black7.clone() + &black8, black7.clone() + &black8[1..]] {
                let black10: u64 = black9.parse().unwrap();
                let black11 = black10 * black10;
                if black11 > black4 { continue; }
                if black11 >= black3 && Self::black12(black11) { black5 += 1; }
            }
        }
        black5
    }
    fn black12(black13: u64) -> bool {
        let black14 = black13.to_string();
        black14 == black14.chars().rev().collect::<String>()
    }
}
```