# Count Almost Equal Pairs II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Sorting, Counting, Enumeration

---

## Problem

<p><strong>Attention</strong>: In this version, the number of operations that can be performed, has been increased to <strong>twice</strong>.<!-- notionvc: 278e7cb2-3b05-42fa-8ae9-65f5fd6f7585 --></p>

<p>You are given an array <code>nums</code> consisting of positive integers.</p>

<p>We call two integers <code>x</code> and <code>y</code> <strong>almost equal</strong> if both integers can become equal after performing the following operation <strong>at most <u>twice</u></strong>:</p>

<ul>
	<li>Choose <strong>either</strong> <code>x</code> or <code>y</code> and swap any two digits within the chosen number.</li>
</ul>

<p>Return the number of indices <code>i</code> and <code>j</code> in <code>nums</code> where <code>i &lt; j</code> such that <code>nums[i]</code> and <code>nums[j]</code> are <strong>almost equal</strong>.</p>

<p><strong>Note</strong> that it is allowed for an integer to have leading zeros after performing an operation.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1023,2310,2130,213]</span></p>

<p><strong>Output:</strong> <span class="example-io">4</span></p>

<p><strong>Explanation:</strong></p>

<p>The almost equal pairs of elements are:</p>

<ul>
	<li>1023 and 2310. By swapping the digits 1 and 2, and then the digits 0 and 3 in 1023, you get 2310.</li>
	<li>1023 and 213. By swapping the digits 1 and 0, and then the digits 1 and 2 in 1023, you get 0213, which is 213.</li>
	<li>2310 and 213. By swapping the digits 2 and 0, and then the digits 3 and 2 in 2310, you get 0213, which is 213.</li>
	<li>2310 and 2130. By swapping the digits 3 and 1 in 2310, you get 2130.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [1,10,100]</span></p>

<p><strong>Output:</strong> <span class="example-io">3</span></p>

<p><strong>Explanation:</strong></p>

<p>The almost equal pairs of elements are:</p>

<ul>
	<li>1 and 10. By swapping the digits 1 and 0 in 10, you get 01 which is 1.</li>
	<li>1 and 100. By swapping the second 0 with the digit 1 in 100, you get 001, which is 1.</li>
	<li>10 and 100. By swapping the first 0 with the digit 1 in 100, you get 010, which is 10.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= nums.length &lt;= 5000</code></li>
	<li><code>1 &lt;= nums[i] &lt; 10<sup>7</sup></code></li>
</ul>


## Hints

1. For each element, find all possible integers we can get by applying the operations.
2. Store the frequencies of all the integers in a hashmap.

## Solution

```rust
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn count_pairs(black1: Vec<i32>) -> i32 {
        let mut black2 = 0;
        let mut black3 = HashMap::new();
        let mut black4 = black1.clone();
        black4.sort();

        for black5 in black4 {
            let mut black6 = HashSet::new();
            black6.insert(black5);
            let mut black7 = black5.to_string().into_bytes();
            while black7.len() < 7 { black7.insert(0, b'0'); }

            for black8 in 0..7 {
                for black9 in black8 + 1..7 {
                    black7.swap(black8, black9);
                    black6.insert(Self::black_to_int(&black7));
                    for black10 in 0..7 {
                        for black11 in black10 + 1..7 {
                            black7.swap(black10, black11);
                            black6.insert(Self::black_to_int(&black7));
                            black7.swap(black10, black11);
                        }
                    }
                    black7.swap(black8, black9);
                }
            }
            for black12 in black6 {
                if let Some(&black13) = black3.get(&black12) { black2 += black13; }
            }
            *black3.entry(black5).or_insert(0) += 1;
        }
        black2
    }

    fn black_to_int(black14: &[u8]) -> i32 {
        black14.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as i32)
    }
}
```