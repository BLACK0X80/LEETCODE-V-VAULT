# Apply Operations to Maximize Score

**Difficulty:** Hard
**Tags:** Array, Math, Stack, Greedy, Sorting, Monotonic Stack, Number Theory

---

## Problem

<p>You are given an array <code>nums</code> of <code>n</code> positive integers and an integer <code>k</code>.</p>

<p>Initially, you start with a score of <code>1</code>. You have to maximize your score by applying the following operation at most <code>k</code> times:</p>

<ul>
	<li>Choose any <strong>non-empty</strong> subarray <code>nums[l, ..., r]</code> that you haven&#39;t chosen previously.</li>
	<li>Choose an element <code>x</code> of <code>nums[l, ..., r]</code> with the highest <strong>prime score</strong>. If multiple such elements exist, choose the one with the smallest index.</li>
	<li>Multiply your score by <code>x</code>.</li>
</ul>

<p>Here, <code>nums[l, ..., r]</code> denotes the subarray of <code>nums</code> starting at index <code>l</code> and ending at the index <code>r</code>, both ends being inclusive.</p>

<p>The <strong>prime score</strong> of an integer <code>x</code> is equal to the number of distinct prime factors of <code>x</code>. For example, the prime score of <code>300</code> is <code>3</code> since <code>300 = 2 * 2 * 3 * 5 * 5</code>.</p>

<p>Return <em>the <strong>maximum possible score</strong> after applying at most </em><code>k</code><em> operations</em>.</p>

<p>Since the answer may be large, return it modulo <code>10<sup>9 </sup>+ 7</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [8,3,9,3,8], k = 2
<strong>Output:</strong> 81
<strong>Explanation:</strong> To get a score of 81, we can apply the following operations:
- Choose subarray nums[2, ..., 2]. nums[2] is the only element in this subarray. Hence, we multiply the score by nums[2]. The score becomes 1 * 9 = 9.
- Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 1, but nums[2] has the smaller index. Hence, we multiply the score by nums[2]. The score becomes 9 * 9 = 81.
It can be proven that 81 is the highest score one can obtain.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [19,12,14,6,10,18], k = 3
<strong>Output:</strong> 4788
<strong>Explanation:</strong> To get a score of 4788, we can apply the following operations: 
- Choose subarray nums[0, ..., 0]. nums[0] is the only element in this subarray. Hence, we multiply the score by nums[0]. The score becomes 1 * 19 = 19.
- Choose subarray nums[5, ..., 5]. nums[5] is the only element in this subarray. Hence, we multiply the score by nums[5]. The score becomes 19 * 18 = 342.
- Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 2, but nums[2] has the smaller index. Hence, we multipy the score by nums[2]. The score becomes 342 * 14 = 4788.
It can be proven that 4788 is the highest score one can obtain.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length == n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= k &lt;= min(n * (n + 1) / 2, 10<sup>9</sup>)</code></li>
</ul>


## Hints

1. <div class="_1l1MA">Calculate <code>nums[i]</code>'s prime score <code>s[i]</code> by factoring in <code>O(sqrt(nums[i]))</code> time.</div>
2. <div class="_1l1MA">For each <code>nums[i]</code>, find the nearest index <code>left[i]</code> on the left (if any) such that <code>s[left[i]] >= s[i]</code>. if none is found, set <code>left[i]</code> to <code>-1</code>. Similarly, find the nearest index <code>right[i]</code> on the right (if any) such that <code>s[right[i]] > s[i]</code>. If none is found, set <code>right[i]</code> to <code>n</code>.</div>
3. <div class="_1l1MA">Use a monotonic stack to compute <code>right[i]</code> and <code>left[i]</code>.</div>
4. <div class="_1l1MA">For each index <code>i</code>, if <code>left[i] + 1 <= l <= i <= r <= right[i] - 1</code>, then <code>s[i]</code> is the maximum value in the range <code>[l, r]</code>. For this particular <code>i</code>, there are <code>ranges[i] = (i - left[i]) * (right[i] - i)</code> ranges where index <code>i</code> will be chosen.</div>
5. <div class="_1l1MA">Loop over all elements of <code>nums</code> by non-increasing prime score, each element will be chosen <code>min(ranges[i], remainingK)</code> times, where <code>reaminingK</code> denotes the number of remaining operations. Therefore, the score will be multiplied by <code>s[i]^min(ranges[i],remainingK)</code>.</div>
6. <div class="_1l1MA">Use fast exponentiation to quickly calculate <code>A^B mod C</code>.</div>

## Solution

```rust
impl Solution {
    pub fn maximum_score(black_nums: Vec<i32>, mut black_k: i32) -> i32 {
        let black_n = black_nums.len();
        let mut black_prime_scores = vec![0; black_n];
        for black_i in 0..black_n {
            let mut black_val = black_nums[black_i];
            let mut black_score = 0;
            let mut black_d = 2;
            while black_d * black_d <= black_val {
                if black_val % black_d == 0 {
                    black_score += 1;
                    while black_val % black_d == 0 { black_val /= black_d; }
                }
                black_d += 1;
            }
            if black_val > 1 { black_score += 1; }
            black_prime_scores[black_i] = black_score;
        }

        let mut black_left = vec![-1; black_n];
        let mut black_right = vec![black_n as i32; black_n];
        let mut black_stack = vec![];
        for black_i in 0..black_n {
            while !black_stack.is_empty() && black_prime_scores[*black_stack.last().unwrap()] < black_prime_scores[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_left[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }
        black_stack.clear();
        for black_i in (0..black_n).rev() {
            while !black_stack.is_empty() && black_prime_scores[*black_stack.last().unwrap()] <= black_prime_scores[black_i] {
                black_stack.pop();
            }
            if let Some(&black_top) = black_stack.last() { black_right[black_i] = black_top as i32; }
            black_stack.push(black_i);
        }

        let mut black_elements: Vec<(i32, usize)> = black_nums.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect();
        black_elements.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        
        let black_mod = 1_000_000_007i64;
        let mut black_ans = 1i64;

        let black_pow = |mut black_base: i64, mut black_exp: i64| {
            let mut black_res = 1;
            black_base %= black_mod;
            while black_exp > 0 {
                if black_exp % 2 == 1 { black_res = (black_res * black_base) % black_mod; }
                black_base = (black_base * black_base) % black_mod;
                black_exp /= 2;
            }
            black_res
        };

        let bravexuneth = &black_elements;
        for &(black_val, black_idx) in bravexuneth {
            let black_count = (black_idx as i64 - black_left[black_idx] as i64) * (black_right[black_idx] as i64 - black_idx as i64);
            let black_take = black_count.min(black_k as i64);
            black_ans = (black_ans * black_pow(black_val as i64, black_take)) % black_mod;
            black_k -= black_take as i32;
            if black_k == 0 { break; }
        }
        black_ans as i32
    }
}
```