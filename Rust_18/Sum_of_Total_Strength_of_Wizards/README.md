# Sum of Total Strength of Wizards

**Difficulty:** Hard
**Tags:** Array, Stack, Monotonic Stack, Prefix Sum

---

## Problem

<p>As the ruler of a kingdom, you have an army of wizards at your command.</p>

<p>You are given a <strong>0-indexed</strong> integer array <code>strength</code>, where <code>strength[i]</code> denotes the strength of the <code>i<sup>th</sup></code> wizard. For a <strong>contiguous</strong> group of wizards (i.e. the wizards&#39; strengths form a <strong>subarray</strong> of <code>strength</code>), the <strong>total strength</strong> is defined as the <strong>product</strong> of the following two values:</p>

<ul>
	<li>The strength of the <strong>weakest</strong> wizard in the group.</li>
	<li>The <strong>total</strong> of all the individual strengths of the wizards in the group.</li>
</ul>

<p>Return <em>the <strong>sum</strong> of the total strengths of <strong>all</strong> contiguous groups of wizards</em>. Since the answer may be very large, return it <strong>modulo</strong> <code>10<sup>9</sup> + 7</code>.</p>

<p>A <strong>subarray</strong> is a contiguous <strong>non-empty</strong> sequence of elements within an array.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> strength = [1,3,1,2]
<strong>Output:</strong> 44
<strong>Explanation:</strong> The following are all the contiguous groups of wizards:
- [1] from [<u><strong>1</strong></u>,3,1,2] has a total strength of min([1]) * sum([1]) = 1 * 1 = 1
- [3] from [1,<u><strong>3</strong></u>,1,2] has a total strength of min([3]) * sum([3]) = 3 * 3 = 9
- [1] from [1,3,<u><strong>1</strong></u>,2] has a total strength of min([1]) * sum([1]) = 1 * 1 = 1
- [2] from [1,3,1,<u><strong>2</strong></u>] has a total strength of min([2]) * sum([2]) = 2 * 2 = 4
- [1,3] from [<u><strong>1,3</strong></u>,1,2] has a total strength of min([1,3]) * sum([1,3]) = 1 * 4 = 4
- [3,1] from [1,<u><strong>3,1</strong></u>,2] has a total strength of min([3,1]) * sum([3,1]) = 1 * 4 = 4
- [1,2] from [1,3,<u><strong>1,2</strong></u>] has a total strength of min([1,2]) * sum([1,2]) = 1 * 3 = 3
- [1,3,1] from [<u><strong>1,3,1</strong></u>,2] has a total strength of min([1,3,1]) * sum([1,3,1]) = 1 * 5 = 5
- [3,1,2] from [1,<u><strong>3,1,2</strong></u>] has a total strength of min([3,1,2]) * sum([3,1,2]) = 1 * 6 = 6
- [1,3,1,2] from [<u><strong>1,3,1,2</strong></u>] has a total strength of min([1,3,1,2]) * sum([1,3,1,2]) = 1 * 7 = 7
The sum of all the total strengths is 1 + 9 + 1 + 4 + 4 + 4 + 3 + 5 + 6 + 7 = 44.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> strength = [5,4,6]
<strong>Output:</strong> 213
<strong>Explanation:</strong> The following are all the contiguous groups of wizards: 
- [5] from [<u><strong>5</strong></u>,4,6] has a total strength of min([5]) * sum([5]) = 5 * 5 = 25
- [4] from [5,<u><strong>4</strong></u>,6] has a total strength of min([4]) * sum([4]) = 4 * 4 = 16
- [6] from [5,4,<u><strong>6</strong></u>] has a total strength of min([6]) * sum([6]) = 6 * 6 = 36
- [5,4] from [<u><strong>5,4</strong></u>,6] has a total strength of min([5,4]) * sum([5,4]) = 4 * 9 = 36
- [4,6] from [5,<u><strong>4,6</strong></u>] has a total strength of min([4,6]) * sum([4,6]) = 4 * 10 = 40
- [5,4,6] from [<u><strong>5,4,6</strong></u>] has a total strength of min([5,4,6]) * sum([5,4,6]) = 4 * 15 = 60
The sum of all the total strengths is 25 + 16 + 36 + 36 + 40 + 60 = 213.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= strength.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= strength[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Consider the contribution of each wizard to the answer.
2. Can you efficiently calculate the total contribution to the answer for all subarrays that end at each index?
3. Denote the total contribution of all subarrays ending at index i as solve[i]. Can you express solve[i] in terms of solve[m] for some m < i?

## Solution

```rust
impl Solution {
    pub fn total_strength(black_strength: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_strength.len();
        let mut black_p = vec![0i64; black_n + 1];
        for black_i in 0..black_n { black_p[black_i + 1] = (black_p[black_i] + black_strength[black_i] as i64) % black_mod; }
        let mut black_pp = vec![0i64; black_n + 2];
        for black_i in 0..=black_n { black_pp[black_i + 1] = (black_pp[black_i] + black_p[black_i]) % black_mod; }
        
        let mut black_left = vec![-1; black_n];
        let mut black_stack = vec![];
        for black_i in 0..black_n {
            while !black_stack.is_empty() && black_strength[*black_stack.last().unwrap()] >= black_strength[black_i] { black_stack.pop(); }
            if let Some(&black_top) = black_stack.last() { black_left[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let bravexuneth = black_pp;
        let mut black_right = vec![black_n as i32; black_n];
        black_stack.clear();
        for black_i in (0..black_n).rev() {
            while !black_stack.is_empty() && black_strength[*black_stack.last().unwrap()] > black_strength[black_i] { black_stack.pop(); }
            if let Some(&black_top) = black_stack.last() { black_right[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let mut black_ans = 0i64;
        for black_i in 0..black_n {
            let black_l = (black_left[black_i] + 1) as usize;
            let black_r = (black_right[black_i] - 1) as usize;
            let black_m = black_i;
            let black_a = (black_m - black_l + 1) as i64;
            let black_b = (black_r - black_m + 1) as i64;
            
            let black_pos = (black_a * (bravexuneth[black_r + 2] - bravexuneth[black_m + 1] + black_mod)) % black_mod;
            let black_neg = (black_b * (bravexuneth[black_m + 1] - bravexuneth[black_l] + black_mod)) % black_mod;
            
            let black_term = (black_pos - black_neg + black_mod) % black_mod;
            black_ans = (black_ans + black_strength[black_i] as i64 * black_term) % black_mod;
        }
        black_ans as i32
    }
}
```