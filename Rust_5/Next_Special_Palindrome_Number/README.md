# Next Special Palindrome Number

**Difficulty:** Hard
**Tags:** Backtracking, Bit Manipulation

---

## Problem

<p>You are given an integer <code>n</code>.</p>

<p>A number is called <strong>special</strong> if:</p>

<ul>
	<li>It is a <strong><span data-keyword="palindrome-integer">palindrome</span></strong>.</li>
	<li>Every digit <code>k</code> in the number appears <strong>exactly</strong> <code>k</code> times.</li>
</ul>

<p>Return the <strong>smallest</strong> special number <strong>strictly </strong>greater than <code>n</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">22</span></p>

<p><strong>Explanation:</strong></p>

<p>22 is the smallest special number greater than 2, as it is a palindrome and the digit 2 appears exactly 2 times.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 33</span></p>

<p><strong>Output:</strong> <span class="example-io">212</span></p>

<p><strong>Explanation:</strong></p>

<p>212 is the smallest special number greater than 33, as it is a palindrome and the digits 1 and 2 appear exactly 1 and 2 times respectively.<br />
 </p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= n &lt;= 10<sup>15</sup></code></li>
</ul>


## Hints

1. There are only a few special numbers; preprocess them.
2. Use bitmasking to brute‑force all valid selections of digits <code>k</code> for the number.
3. Generate all permutations of the first half of the digits, then mirror them to form the full palindrome.
4. For each <code>n</code>, use binary search to find the smallest special number strictly greater than <code>n</code>.

## Solution

```rust
impl Solution {
    pub fn special_palindrome(n: i64) -> i64 {
        let mut black_results = Vec::new();
        let black_digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 1..(1 << 9) {
            let mut black_set = Vec::new();
            let (mut black_sum, mut black_odds) = (0, 0);
            for j in 0..9 {
                if (i >> j) & 1 == 1 {
                    black_set.push(black_digits[j]);
                    black_sum += black_digits[j];
                    if black_digits[j] % 2 != 0 { black_odds += 1; }
                }
            }
            
            if black_odds <= 1 && black_sum <= 16 {
                let mut black_half = Vec::new();
                let mut black_mid = None;
                for &d in &black_set {
                    if d % 2 != 0 {
                        black_mid = Some(d);
                        for _ in 0..(d / 2) { black_half.push(d); }
                    } else {
                        for _ in 0..(d / 2) { black_half.push(d); }
                    }
                }
                
                Self::black_permute(&mut black_half, black_mid, &mut black_results);
            }
        }
        black_results.sort_unstable();
        black_results.dedup();
        *black_results.iter().find(|&&x| x > n).unwrap_or(&-1)
    }

    fn black_permute(black_h: &mut Vec<u8>, black_m: Option<u8>, black_res: &mut Vec<i64>) {
        black_h.sort_unstable();
        loop {
            if !black_h.is_empty() && black_h[0] == 0 { if !Self::black_next_p(black_h) { break; } continue; }
            let mut black_f = black_h.clone();
            if let Some(m) = black_m { black_f.push(m); }
            for i in (0..black_h.len()).rev() { black_f.push(black_h[i]); }
            black_res.push(black_f.iter().fold(0i64, |a, &b| a * 10 + b as i64));
            if !Self::black_next_p(black_h) { break; }
        }
    }

    fn black_next_p(black_p: &mut Vec<u8>) -> bool {
        if black_p.len() < 2 { return false; }
        let mut i = black_p.len() - 1;
        while i > 0 && black_p[i-1] >= black_p[i] { i -= 1; }
        if i == 0 { return false; }
        let mut j = black_p.len() - 1;
        while black_p[j] <= black_p[i-1] { j -= 1; }
        black_p.swap(i-1, j);
        black_p[i..].reverse();
        true
    }
}
```