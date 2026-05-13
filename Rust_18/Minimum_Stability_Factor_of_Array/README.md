# Minimum Stability Factor of Array

**Difficulty:** Hard
**Tags:** Array, Math, Binary Search, Greedy, Segment Tree, Number Theory

---

## Problem

<p>You are given an integer array <code>nums</code> and an integer <code>maxC</code>.</p>

<p>A <strong><span data-keyword="subarray">subarray</span></strong> is called <strong>stable</strong> if the <em>highest common factor (HCF)</em> of all its elements is <strong>greater than or equal to</strong> 2.</p>

<p>The <strong>stability factor</strong> of an array is defined as the length of its <strong>longest</strong> stable subarray.</p>

<p>You may modify <strong>at most</strong> <code>maxC</code> elements of the array to any integer.</p>

<p>Return the <strong>minimum</strong> possible stability factor of the array after at most <code>maxC</code> modifications. If no stable subarray remains, return 0.</p>

<p><strong>Note:</strong></p>

<ul>
	<li>The <strong>highest common factor (HCF)</strong> of an array is the largest integer that evenly divides all the array elements.</li>
	<li>A <strong>subarray</strong> of length 1 is stable if its only element is greater than or equal to 2, since <code>HCF([x]) = x</code>.</li>
</ul>

<div class="notranslate" style="all: initial;"> </div>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [3,5,10], maxC = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The stable subarray <code>[5, 10]</code> has <code>HCF = 5</code>, which has a stability factor of 2.</li>
	<li>Since <code>maxC = 1</code>, one optimal strategy is to change <code>nums[1]</code> to <code>7</code>, resulting in <code>nums = [3, 7, 10]</code>.</li>
	<li>Now, no subarray of length greater than 1 has <code>HCF &gt;= 2</code>. Thus, the minimum possible stability factor is 1.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,6,8], maxC = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">1</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The subarray <code>[2, 6, 8]</code> has <code>HCF = 2</code>, which has a stability factor of 3.</li>
	<li>Since <code>maxC = 2</code>, one optimal strategy is to change <code>nums[1]</code> to 3 and <code>nums[2]</code> to 5, resulting in <code>nums = [2, 3, 5]</code>.</li>
	<li>Now, no subarray of length greater than 1 has <code>HCF &gt;= 2</code>. Thus, the minimum possible stability factor is 1.</li>
</ul>
</div>

<p><strong class="example">Example 3:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [2,4,9,6], maxC = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">2</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>The stable subarrays are:
	<ul>
		<li><code>[2, 4]</code> with <code>HCF = 2</code> and stability factor of 2.</li>
		<li><code>[9, 6]</code> with <code>HCF = 3</code> and stability factor of 2.</li>
	</ul>
	</li>
	<li>Since <code>maxC = 1</code>, the stability factor of 2 cannot be reduced due to two separate stable subarrays. Thus, the minimum possible stability factor is 2.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n == nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= maxC &lt;= n</code></li>
</ul>


## Hints

1. Binary‐search the target length <code>k</code>
2. For each <code>k</code>, use fast range‐GCD queries
3. Greedily "hit" every window of size <code>k+1</code> with an edit if its <code>GCD > 1</code>

## Solution

```rust
impl Solution {
    pub fn min_stable(black_nums: Vec<i32>, black_max_c: i32) -> i32 {
        let black_n = black_nums.len();
        let black_log_n = (black_n as f64).log2() as usize + 1;
        let mut black_st = vec![vec![0; black_n]; black_log_n];

        for black_j in 0..black_n {
            black_st[0][black_j] = black_nums[black_j];
        }

        for black_i in 1..black_log_n {
            for black_j in 0..=(black_n - (1 << black_i)) {
                black_st[black_i][black_j] = Self::black_gcd(
                    black_st[black_i - 1][black_j],
                    black_st[black_i - 1][black_j + (1 << (black_i - 1))],
                );
            }
        }

        let black_query = |black_l: usize, black_r: usize, black_st: &Vec<Vec<i32>>| -> i32 {
            let black_dist = black_r - black_l + 1;
            let black_i = (black_dist as f64).log2() as usize;
            Self::black_gcd(black_st[black_i][black_l], black_st[black_i][black_r - (1 << black_i) + 1])
        };

        let mut black_left_stable = vec![0; black_n];
        let mut black_j_ptr = 0;
        for black_i in 0..black_n {
            while black_j_ptr < black_i && black_query(black_j_ptr, black_i, &black_st) == 1 {
                black_j_ptr += 1;
            }
            black_left_stable[black_i] = black_j_ptr;
        }

        let mut black_l_bs = 0;
        let mut black_r_bs = black_n as i32;
        while black_l_bs < black_r_bs {
            let black_mid = (black_l_bs + black_r_bs) / 2;
            let mut black_cnt = 0;
            let mut black_curr_j = 0;
            let mut black_curr_g = 0;
            let mut black_idx = 0;

            while black_idx < black_n {
                black_curr_g = if black_curr_g == 0 { black_nums[black_idx] } else { Self::black_gcd(black_curr_g, black_nums[black_idx]) };
                
                if black_curr_g == 1 {
                    black_curr_j = black_curr_j.max(black_left_stable[black_idx]);
                    black_curr_g = black_query(black_curr_j, black_idx, &black_st);
                }

                if (black_idx - black_curr_j) as i32 >= black_mid {
                    if black_curr_g > 1 {
                        black_cnt += 1;
                    }
                    black_curr_g = 0;
                    black_curr_j = black_idx + 1;
                }
                black_idx += 1;
            }

            if black_cnt <= black_max_c {
                black_r_bs = black_mid;
            } else {
                black_l_bs = black_mid + 1;
            }
        }

        black_l_bs
    }

    fn black_gcd(mut black_a: i32, mut black_b: i32) -> i32 {
        while black_b != 0 {
            black_a %= black_b;
            std::mem::swap(&mut black_a, &mut black_b);
        }
        black_a
    }
}
```