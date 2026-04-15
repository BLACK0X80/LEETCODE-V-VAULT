# Check If Digits Are Equal in String After Operations II

**Difficulty:** Hard
**Tags:** Math, String, Combinatorics, Number Theory

---

## Problem

<p>You are given a string <code>s</code> consisting of digits. Perform the following operation repeatedly until the string has <strong>exactly</strong> two digits:</p>

<ul>
	<li>For each pair of consecutive digits in <code>s</code>, starting from the first digit, calculate a new digit as the sum of the two digits <strong>modulo</strong> 10.</li>
	<li>Replace <code>s</code> with the sequence of newly calculated digits, <em>maintaining the order</em> in which they are computed.</li>
</ul>

<p>Return <code>true</code> if the final two digits in <code>s</code> are the <strong>same</strong>; otherwise, return <code>false</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;3902&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">true</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Initially, <code>s = &quot;3902&quot;</code></li>
	<li>First operation:
	<ul>
		<li><code>(s[0] + s[1]) % 10 = (3 + 9) % 10 = 2</code></li>
		<li><code>(s[1] + s[2]) % 10 = (9 + 0) % 10 = 9</code></li>
		<li><code>(s[2] + s[3]) % 10 = (0 + 2) % 10 = 2</code></li>
		<li><code>s</code> becomes <code>&quot;292&quot;</code></li>
	</ul>
	</li>
	<li>Second operation:
	<ul>
		<li><code>(s[0] + s[1]) % 10 = (2 + 9) % 10 = 1</code></li>
		<li><code>(s[1] + s[2]) % 10 = (9 + 2) % 10 = 1</code></li>
		<li><code>s</code> becomes <code>&quot;11&quot;</code></li>
	</ul>
	</li>
	<li>Since the digits in <code>&quot;11&quot;</code> are the same, the output is <code>true</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">s = &quot;34789&quot;</span></p>

<p><strong>Output:</strong> <span class="example-io">false</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Initially, <code>s = &quot;34789&quot;</code>.</li>
	<li>After the first operation, <code>s = &quot;7157&quot;</code>.</li>
	<li>After the second operation, <code>s = &quot;862&quot;</code>.</li>
	<li>After the third operation, <code>s = &quot;48&quot;</code>.</li>
	<li>Since <code>&#39;4&#39; != &#39;8&#39;</code>, the output is <code>false</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= s.length &lt;= 10<sup>5</sup></code></li>
	<li><code>s</code> consists of only digits.</li>
</ul>


## Hints

1. Can we use <code>nCr</code> and use Pascal's triangle values here?
2. <code>nCr mod 10</code> can be uniquely determined from <code>nCr mod 2</code> and <code>nCr mod 5</code>.
3. Use Lucas's theorem.

## Solution

```rust
impl Solution {
    pub fn has_same_digits(black_s: String) -> bool {
        let black_n = black_s.len();
        let black_digits: Vec<i32> = black_s.bytes().map(|b| (b - b'0') as i32).collect();
        
        fn ncr_mod(n: usize, r: usize, m: i32) -> i32 {
            if r > n { return 0; }
            let mut num = n;
            let mut den = r;
            let mut count = 0;
            let mut res = 1;
            for i in 0..r {
                let mut curr_num = n - i;
                while curr_num > 0 && curr_num % m as usize == 0 {
                    count += 1;
                    curr_num /= m as usize;
                }
                res = (res * (curr_num % m as usize)) % m as usize;
                let mut curr_den = i + 1;
                while curr_den > 0 && curr_den % m as usize == 0 {
                    count -= 1;
                    curr_den /= m as usize;
                }
                let inv = match m {
                    2 => 1,
                    5 => [0, 1, 3, 2, 4][curr_den % 5],
                    _ => 0
                };
                res = (res * inv) % m as usize;
            }
            if count > 0 { 0 } else { res as i32 }
        }

        
        fn get_comb_vec(n: usize, m: i32) -> Vec<i32> {
            let mut res = vec![1; n + 1];
            let mut count = 0;
            let mut val = 1;
            for r in 1..=n {
                let mut num = n - r + 1;
                while num % m as usize == 0 { count += 1; num /= m as usize; }
                val = (val * (num % m as usize)) % m as usize;
                let mut den = r;
                while den % m as usize == 0 { count -= 1; den /= m as usize; }
                let inv = match m { 2 => 1, 5 => [0, 1, 3, 2, 4][den % 5], _ => 1 };
                val = (val * inv) % m as usize;
                res[r] = if count > 0 { 0 } else { val as i32 };
            }
            res
        }

        let black_c2 = get_comb_vec(black_n - 2, 2);
        let black_c5 = get_comb_vec(black_n - 2, 5);
        let mut black_c10 = vec![0; black_n - 1];
        for i in 0..black_n - 1 {
            let (a, b) = (black_c2[i], black_c5[i]);
            for v in 0..10 { if v % 2 == a && v % 5 == b { black_c10[i] = v; break; } }
        }

        let (mut black_v1, mut black_v2) = (0, 0);
        for i in 0..black_n - 1 {
            black_v1 = (black_v1 + black_digits[i] * black_c10[i]) % 10;
            black_v2 = (black_v2 + black_digits[i + 1] * black_c10[i]) % 10;
        }
        black_v1 == black_v2
    }
}
```