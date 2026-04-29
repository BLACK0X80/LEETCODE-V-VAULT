# Find the Count of Good Integers

**Difficulty:** Hard
**Tags:** Hash Table, Math, Combinatorics, Enumeration

---

## Problem

<p>You are given two <strong>positive</strong> integers <code>n</code> and <code>k</code>.</p>

<p>An integer <code>x</code> is called <strong>k-palindromic</strong> if:</p>

<ul>
	<li><code>x</code> is a <span data-keyword="palindrome-integer">palindrome</span>.</li>
	<li><code>x</code> is divisible by <code>k</code>.</li>
</ul>

<p>An integer is called <strong>good</strong> if its digits can be <em>rearranged</em> to form a <strong>k-palindromic</strong> integer. For example, for <code>k = 2</code>, 2020 can be rearranged to form the <em>k-palindromic</em> integer 2002, whereas 1010 cannot be rearranged to form a <em>k-palindromic</em> integer.</p>

<p>Return the count of <strong>good</strong> integers containing <code>n</code> digits.</p>

<p><strong>Note</strong> that <em>any</em> integer must <strong>not</strong> have leading zeros, <strong>neither</strong> before <strong>nor</strong> after rearrangement. For example, 1010 <em>cannot</em> be rearranged to form 101.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 3, k = 5</span></p>

<p><strong>Output:</strong> <span class="example-io">27</span></p>

<p><strong>Explanation:</strong></p>

<p><em>Some</em> of the good integers are:</p>

<ul>
	<li>551 because it can be rearranged to form 515.</li>
	<li>525 because it is already k-palindromic.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 1, k = 4</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<p>The two good integers are 4 and 8.</p>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">n = 5, k = 6</span></p>

<p><strong>Output:</strong> <span class="example-io">2468</span></p>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 10</code></li>
	<li><code>1 &lt;= k &lt;= 9</code></li>
</ul>


## Hints

1. How to generate all K-palindromic strings of length <code>n</code>? Do we need to go through all <code>n</code> digits?
2. Use permutations to calculate the number of possible rearrangements.

## Solution

```rust
use std::collections::HashSet;
impl Solution {
    pub fn count_good_integers(black1: i32, black2: i32) -> i64 {
        let mut black3 = HashSet::new();
        let black4 = (black1 + 1) / 2;
        let black5 = 10i64.pow(black4 as u32 - 1);
        let black6 = 10i64.pow(black4 as u32);

        for black7 in black5..black6 {
            let mut black8 = black7.to_string();
            let mut black9 = black8.chars().rev().collect::<String>();
            if black1 % 2 == 1 { black9.remove(0); }
            black8.push_str(&black9);
            let black10: i64 = black8.parse().unwrap();
            if black10 % black2 as i64 == 0 {
                let mut black11: Vec<char> = black8.chars().collect();
                black11.sort();
                black3.insert(black11);
            }
        }

        let mut black12 = 0;
        let black13 = Self::black_fact(black1 as i64);
        for black14 in black3 {
            let mut black15 = vec![0; 10];
            for black16 in black14 { black15[(black16 as u8 - b'0') as usize] += 1; }
            
            let mut black17 = black13;
            for black18 in 0..10 { black17 /= Self::black_fact(black15[black18]); }
            
            if black15[0] > 0 {
                let mut black19 = Self::black_fact(black1 as i64 - 1);
                black15[0] -= 1;
                for black20 in 0..10 { black19 /= Self::black_fact(black15[black20]); }
                black17 -= black19;
            }
            black12 += black17;
        }
        black12
    }

    fn black_fact(black21: i64) -> i64 {
        (1..=black21).product()
    }
}
```