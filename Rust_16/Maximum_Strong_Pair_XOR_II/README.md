# Maximum Strong Pair XOR II

**Difficulty:** Hard
**Tags:** Array, Hash Table, Bit Manipulation, Trie, Sliding Window

---

## Problem

<p>You are given a <strong>0-indexed</strong> integer array <code>nums</code>. A pair of integers <code>x</code> and <code>y</code> is called a <strong>strong</strong> pair if it satisfies the condition:</p>

<ul>
	<li><code>|x - y| &lt;= min(x, y)</code></li>
</ul>

<p>You need to select two integers from <code>nums</code> such that they form a strong pair and their bitwise <code>XOR</code> is the <strong>maximum</strong> among all strong pairs in the array.</p>

<p>Return <em>the <strong>maximum</strong> </em><code>XOR</code><em> value out of all possible strong pairs in the array</em> <code>nums</code>.</p>

<p><strong>Note</strong> that you can pick the same integer twice to form a pair.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> 7
<strong>Explanation:</strong> There are 11 strong pairs in the array <code>nums</code>: (1, 1), (1, 2), (2, 2), (2, 3), (2, 4), (3, 3), (3, 4), (3, 5), (4, 4), (4, 5) and (5, 5).
The maximum XOR possible from these pairs is 3 XOR 4 = 7.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [10,100]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are 2 strong pairs in the array nums: (10, 10) and (100, 100).
The maximum XOR possible from these pairs is 10 XOR 10 = 0 since the pair (100, 100) also gives 100 XOR 100 = 0.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [500,520,2500,3000]
<strong>Output:</strong> 1020
<strong>Explanation:</strong> There are 6 strong pairs in the array nums: (500, 500), (500, 520), (520, 520), (2500, 2500), (2500, 3000) and (3000, 3000).
The maximum XOR possible from these pairs is 500 XOR 520 = 1020 since the only other non-zero XOR value is 2500 XOR 3000 = 636.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 2<sup>20</sup> - 1</code></li>
</ul>


## Hints

1. Sort the array, now let <code>x <= y</code> which means <code>|x - y| <= min(x, y)</code> can now be written as <code>y - x <= x</code> or in other words, <code>y <= 2 * x</code>.
2. If <code>x</code> and <code>y</code> have the same number of bits, try making<code>y</code>’s bits different from x if possible for each bit starting from the second most significant bit.
3. If <code>y</code> has 1 more bit than <code>x</code> and <code>y <= 2 * x</code> use the idea about Digit DP to make <code>y</code>’s prefix smaller than <code>2 * x + 1</code> as well as trying to make each bit different from <code>x</code> using a Hashmap.
4. Alternatively, use Trie data structure to find the pair with maximum <code>XOR</code>.

## Solution

```rust
impl Solution {
    pub fn maximum_strong_pair_xor(mut black_nums: Vec<i32>) -> i32 {
        black_nums.sort_unstable();
        let mut black_t = vec![[0, 0, 0]];
        let (mut black_l, mut black_ans) = (0, 0);

        for &black_r_val in &black_nums {
            Self::black_upd(&mut black_t, black_r_val, 1);
            while black_nums[black_l] * 2 < black_r_val {
                Self::black_upd(&mut black_t, black_nums[black_l], -1);
                black_l += 1;
            }
            black_ans = black_ans.max(Self::black_get(&black_t, black_r_val));
        }
        black_ans
    }

    fn black_upd(t: &mut Vec<[i32; 3]>, v: i32, d: i32) {
        let mut curr = 0;
        for i in (0..20).rev() {
            let b = ((v >> i) & 1) as usize;
            if t[curr][b] == 0 {
                t[curr][b] = t.len() as i32;
                t.push([0, 0, 0]);
            }
            curr = t[curr][b] as usize;
            t[curr][2] += d;
        }
    }

    fn black_get(t: &Vec<[i32; 3]>, v: i32) -> i32 {
        let (mut curr, mut res) = (0, 0);
        for i in (0..20).rev() {
            let b = ((v >> i) & 1) as usize;
            let target = 1 - b;
            let next = t[curr][target] as usize;
            if next != 0 && t[next][2] > 0 {
                res |= 1 << i;
                curr = next;
            } else {
                curr = t[curr][b] as usize;
            }
        }
        res
    }
}
```