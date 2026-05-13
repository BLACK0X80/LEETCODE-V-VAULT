# Split Message Based on Limit

**Difficulty:** Hard
**Tags:** String, Enumeration

---

## Problem

<p>You are given a string, <code>message</code>, and a positive integer, <code>limit</code>.</p>

<p>You must <strong>split</strong> <code>message</code> into one or more <strong>parts</strong> based on <code>limit</code>. Each resulting part should have the suffix <code>&quot;&lt;a/b&gt;&quot;</code>, where <code>&quot;b&quot;</code> is to be <strong>replaced</strong> with the total number of parts and <code>&quot;a&quot;</code> is to be <strong>replaced</strong> with the index of the part, starting from <code>1</code> and going up to <code>b</code>. Additionally, the length of each resulting part (including its suffix) should be <strong>equal</strong> to <code>limit</code>, except for the last part whose length can be <strong>at most</strong> <code>limit</code>.</p>

<p>The resulting parts should be formed such that when their suffixes are removed and they are all concatenated <strong>in order</strong>, they should be equal to <code>message</code>. Also, the result should contain as few parts as possible.</p>

<p>Return<em> the parts </em><code>message</code><em> would be split into as an array of strings</em>. If it is impossible to split <code>message</code> as required, return<em> an empty array</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> message = &quot;this is really a very awesome message&quot;, limit = 9
<strong>Output:</strong> [&quot;thi&lt;1/14&gt;&quot;,&quot;s i&lt;2/14&gt;&quot;,&quot;s r&lt;3/14&gt;&quot;,&quot;eal&lt;4/14&gt;&quot;,&quot;ly &lt;5/14&gt;&quot;,&quot;a v&lt;6/14&gt;&quot;,&quot;ery&lt;7/14&gt;&quot;,&quot; aw&lt;8/14&gt;&quot;,&quot;eso&lt;9/14&gt;&quot;,&quot;me&lt;10/14&gt;&quot;,&quot; m&lt;11/14&gt;&quot;,&quot;es&lt;12/14&gt;&quot;,&quot;sa&lt;13/14&gt;&quot;,&quot;ge&lt;14/14&gt;&quot;]
<strong>Explanation:</strong>
The first 9 parts take 3 characters each from the beginning of message.
The next 5 parts take 2 characters each to finish splitting message. 
In this example, each part, including the last, has length 9. 
It can be shown it is not possible to split message into less than 14 parts.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> message = &quot;short message&quot;, limit = 15
<strong>Output:</strong> [&quot;short mess&lt;1/2&gt;&quot;,&quot;age&lt;2/2&gt;&quot;]
<strong>Explanation:</strong>
Under the given constraints, the string can be split into two parts: 
- The first part comprises of the first 10 characters, and has a length 15.
- The next part comprises of the last 3 characters, and has a length 8.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= message.length &lt;= 10<sup>4</sup></code></li>
	<li><code>message</code> consists only of lowercase English letters and <code>&#39; &#39;</code>.</li>
	<li><code>1 &lt;= limit &lt;= 10<sup>4</sup></code></li>
</ul>


## Hints

1. Could you solve the problem if you knew how many digits the total number of parts has?
2. Try all possible lengths of the total number of parts, and see if the string can be split such that the total number of parts has that length.
3. Binary search can be used for each part length to find the precise number of parts needed.

## Solution

```rust
impl Solution {
    pub fn split_message(black1: String, black2: i32) -> Vec<String> {
        let black3 = black1.len() as i32;
        let mut black4 = 0; 

        for black5 in 1..=5 {
            let black6 = match black5 {
                1 => 9,
                2 => 99,
                3 => 999,
                4 => 9999,
                _ => 10000,
            };
            
            let black7 = if black5 == 1 { 1 } else { 10i32.pow(black5 as u32 - 1) };
            let mut black8 = black7;
            let mut black9 = black6;
            let mut black10 = -1;

            while black8 <= black9 {
                let black11 = (black8 + black9) / 2;
                if Self::black_check(black3, black2, black11) {
                    black10 = black11;
                    black9 = black11 - 1;
                } else {
                    black8 = black11 + 1;
                }
            }

            if black10 != -1 {
                let mut black11 = vec![];
                let mut black12 = 0;
                let black13 = black1.as_bytes();
                for black14 in 1..=black10 {
                    let black15 = format!("<{}/{}>", black14, black10);
                    let black16 = (black2 as usize).saturating_sub(black15.len());
                    let black17 = std::cmp::min(black12 + black16, black13.len());
                    let mut black18 = String::from_utf8_lossy(&black13[black12..black17]).into_owned();
                    black18.push_str(&black15);
                    black11.push(black18);
                    black12 = black17;
                }
                return black11;
            }
        }
        vec![]
    }

    fn black_check(mut black19: i32, black20: i32, black21: i32) -> bool {
        let black22 = black21.to_string().len() as i32;
        let mut black23 = 1;
        let mut black24 = 10;
        let mut black25 = 0i64;
        
        for black26 in 1..=black21 {
            if black26 == black24 {
                black23 += 1;
                black24 *= 10;
            }
            let black27 = black20 - (black23 + black22 + 3);
            if black27 <= 0 { return false; }
            black25 += black27 as i64;
            if black25 >= black19 as i64 { return true; }
        }
        black25 >= black19 as i64
    }
}
```