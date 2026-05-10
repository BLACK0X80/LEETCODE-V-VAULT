# Integer to English Words

**Difficulty:** Hard
**Tags:** Math, String, Recursion

---

## Problem

<p>Convert a non-negative integer <code>num</code> to its English words representation.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> num = 123
<strong>Output:</strong> &quot;One Hundred Twenty Three&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> num = 12345
<strong>Output:</strong> &quot;Twelve Thousand Three Hundred Forty Five&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> num = 1234567
<strong>Output:</strong> &quot;One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= num &lt;= 2<sup>31</sup> - 1</code></li>
</ul>


## Hints

1. Did you see a pattern in dividing the number into chunk of words? For example, 123 and 123000.
2. Group the number by thousands (3 digits). You can write a helper function that takes a number less than 1000 and convert just that chunk to words.
3. There are many edge cases. What are some good test cases? Does your code work with input such as 0? Or 1000010? (middle chunk is zero and should not be printed out)

## Solution

```rust
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        const ONES: &[&str] = &[
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven",
            "Eight", "Nine", "Ten", "Eleven", "Twelve", "Thirteen",
            "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        ];
        const TENS: &[&str] = &[
            "", "", "Twenty", "Thirty", "Forty", "Fifty",
            "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        const THOUSANDS: &[&str] = &["", "Thousand", "Million", "Billion"];

        fn helper(n: i32, ones: &[&str], tens: &[&str]) -> String {
            if n == 0 {
                return String::new();
            } else if n < 20 {
                return ones[n as usize].to_string();
            } else if n < 100 {
                let t = tens[(n / 10) as usize].to_string();
                let o = ones[(n % 10) as usize];
                return if o.is_empty() { t } else { format!("{} {}", t, o) };
            } else {
                let h = format!("{} Hundred", ones[(n / 100) as usize]);
                let rest = helper(n % 100, ones, tens);
                return if rest.is_empty() { h } else { format!("{} {}", h, rest) };
            }
        }

        let mut num = num;
        let mut parts: Vec<String> = Vec::new();
        let mut i = 0;

        while num > 0 {
            let chunk = num % 1000;
            if chunk != 0 {
                let chunk_str = helper(chunk, ONES, TENS);
                if THOUSANDS[i].is_empty() {
                    parts.push(chunk_str);
                } else {
                    parts.push(format!("{} {}", chunk_str, THOUSANDS[i]));
                }
            }
            num /= 1000;
            i += 1;
        }

        parts.reverse();
        parts.join(" ")
    }
}
```