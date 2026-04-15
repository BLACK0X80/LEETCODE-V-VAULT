# GCD Sort of an Array

**Difficulty:** Hard
**Tags:** Array, Math, Union-Find, Sorting, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code>, and you can perform the following operation <strong>any</strong> number of times on <code>nums</code>:</p>

<ul>
	<li>Swap the positions of two elements <code>nums[i]</code> and <code>nums[j]</code> if <code>gcd(nums[i], nums[j]) &gt; 1</code> where <code>gcd(nums[i], nums[j])</code> is the <strong>greatest common divisor</strong> of <code>nums[i]</code> and <code>nums[j]</code>.</li>
</ul>

<p>Return <code>true</code> <em>if it is possible to sort </em><code>nums</code><em> in <strong>non-decreasing</strong> order using the above swap method, or </em><code>false</code><em> otherwise.</em></p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [7,21,3]
<strong>Output:</strong> true
<strong>Explanation:</strong> We can sort [7,21,3] by performing the following operations:
- Swap 7 and 21 because gcd(7,21) = 7. nums = [<u><strong>21</strong></u>,<u><strong>7</strong></u>,3]
- Swap 21 and 3 because gcd(21,3) = 3. nums = [<u><strong>3</strong></u>,7,<u><strong>21</strong></u>]
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,2,6,2]
<strong>Output:</strong> false
<strong>Explanation:</strong> It is impossible to sort the array because 5 cannot be swapped with any other element.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,5,9,3,15]
<strong>Output:</strong> true
We can sort [10,5,9,3,15] by performing the following operations:
- Swap 10 and 15 because gcd(10,15) = 5. nums = [<u><strong>15</strong></u>,5,9,3,<u><strong>10</strong></u>]
- Swap 15 and 3 because gcd(15,3) = 3. nums = [<u><strong>3</strong></u>,5,9,<u><strong>15</strong></u>,10]
- Swap 10 and 15 because gcd(10,15) = 5. nums = [3,5,9,<u><strong>10</strong></u>,<u><strong>15</strong></u>]
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 3 * 10<sup>4</sup></code></li>
	<li><code>2 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Can we build a graph with all the prime numbers and the original array?
2. We can use union-find to determine which indices are connected (i.e., which indices can be swapped).

## Solution

```rust
impl Solution {
    pub fn gcd_sort(black1: Vec<i32>) -> bool {
        let black2 = *black1.iter().max().unwrap() as usize;
        let mut black3: Vec<usize> = (0..=black2).collect();
        
        let mut black4 = vec![0; black2 + 1];
        for &x in &black1 { black4[x as usize] = 1; }

        let mut black5 = vec![true; black2 + 1];
        for i in 2..=black2 {
            if black5[i] {
                for j in (i * 2..=black2).step_by(i) {
                    black5[j] = false;
                    if black4[j] == 1 {
                        Self::black_union(&mut black3, i, j);
                    }
                }
                if black4[i] == 1 { Self::black_union(&mut black3, i, i); }
            }
        }

        let mut black6 = black1.clone();
        black6.sort();

        for i in 0..black1.len() {
            if black1[i] != black6[i] {
                if Self::black_find(&mut black3, black1[i] as usize) != 
                   Self::black_find(&mut black3, black6[i] as usize) {
                    return false;
                }
            }
        }
        true
    }

    fn black_find(b: &mut Vec<usize>, i: usize) -> usize {
        if b[i] == i { i }
        else { b[i] = Self::black_find(b, b[i]); b[i] }
    }

    fn black_union(b: &mut Vec<usize>, i: usize, j: usize) {
        let root_i = Self::black_find(b, i);
        let root_j = Self::black_find(b, j);
        if root_i != root_j { b[root_i] = root_j; }
    }
}
```