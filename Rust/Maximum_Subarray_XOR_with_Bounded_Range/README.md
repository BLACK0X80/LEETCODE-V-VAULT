# Maximum Subarray XOR with Bounded Range

**Difficulty:** Hard
**Tags:** Array, Bit Manipulation, Trie, Queue, Sliding Window, Prefix Sum, Monotonic Queue

---

## Problem

<p>You are given a non-negative integer array <code>nums</code> and an integer <code>k</code>.</p>

<p>You must select a <strong><span data-keyword="subarray-nonempty">subarray</span></strong> of <code>nums</code> such that the <strong>difference</strong> between its <strong>maximum</strong> and <strong>minimum</strong> elements is at most <code>k</code>. The <strong>value</strong> of this subarray is the bitwise XOR of all elements in the subarray.</p>

<p>Return an integer denoting the <strong>maximum</strong> possible <strong>value</strong> of the selected subarray.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,4,5,6], k = 2</span></p>

<p><strong>Output:</strong> <span class="example-io">7</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Select the subarray <code>[5, <u><strong>4, 5, 6</strong></u>]</code>.</li>
	<li>The difference between its maximum and minimum elements is <code>6 - 4 = 2 &lt;= k</code>.</li>
	<li>The value is <code>4 XOR 5 XOR 6 = 7</code>.</li>
</ul>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block">
<p><strong>Input:</strong> <span class="example-io">nums = [5,4,5,6], k = 1</span></p>

<p><strong>Output:</strong> <span class="example-io">6</span></p>

<p><strong>Explanation:</strong></p>

<ul>
	<li>Select the subarray <code>[5, 4, 5, <u><strong>6</strong></u>]</code>.</li>
	<li>The difference between its maximum and minimum elements is <code>6 - 6 = 0 &lt;= k</code>.</li>
	<li>The value is 6.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 4 * 10<sup>4</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt; 2<sup>15</sup></code></li>
	<li><code>0 &lt;= k &lt; 2<sup>15</sup></code></li>
</ul>


## Hints

1. Maintain an active window such that the difference between its maximum and minimum is at most <code>k</code>
2. For all valid subarray-start indices <code>i</code>, insert their prefix xors <code>pref[i]</code> into a trie (use <code>pref[0] = 0</code>, <code>pref[i + 1] = pref[i] ^ nums[i]</code>); keep counts per node to support deletions as <code>L</code> moves
3. For each right index <code>r</code>, query the trie with <code>pref[r + 1]</code> to get the maximum <code>pref[r + 1] ^ pref[l]</code> for <code>l in [L, r]</code>

## Solution

```rust
use std::collections::VecDeque;

struct BlackTrie {
    black_nodes: Vec<[i32; 2]>,
    black_cnt: Vec<i32>,
}

impl BlackTrie {
    fn new() -> Self { Self { black_nodes: vec![[-1; 2]], black_cnt: vec![0] } }
    
    fn black_update(&mut self, black_v: i32, black_delta: i32) {
        let mut u = 0;
        for i in (0..15).rev() {
            let b = ((black_v >> i) & 1) as usize;
            if self.black_nodes[u][b] == -1 {
                self.black_nodes[u][b] = self.black_nodes.len() as i32;
                self.black_nodes.push([-1; 2]);
                self.black_cnt.push(0);
            }
            u = self.black_nodes[u][b] as usize;
            self.black_cnt[u] += black_delta;
        }
    }

    fn black_query(&self, black_v: i32) -> i32 {
        let (mut u, mut black_res) = (0, 0);
        for i in (0..15).rev() {
            let b = ((black_v >> i) & 1) as usize;
            let black_target = 1 - b;
            let black_nxt = self.black_nodes[u][black_target];
            if black_nxt != -1 && self.black_cnt[black_nxt as usize] > 0 {
                black_res |= 1 << i;
                u = black_nxt as usize;
            } else {
                u = self.black_nodes[u][b] as usize;
            }
        }
        black_res
    }
}

impl Solution {
    pub fn max_xor(nums: Vec<i32>, k: i32) -> i32 {
        let black_n = nums.len();
        let mut black_pre = vec![0; black_n + 1];
        for i in 0..black_n { black_pre[i+1] = black_pre[i] ^ nums[i]; }

        let (mut black_ans, mut black_l, mut black_trie) = (0, 0, BlackTrie::new());
        let (mut black_min_q, mut black_max_q) = (VecDeque::new(), VecDeque::new());

        for black_r in 0..black_n {
            // تحديث القيم الصغرى والعظمى في النافذة
            while !black_min_q.is_empty() && nums[*black_min_q.back().unwrap()] >= nums[black_r] { black_min_q.pop_back(); }
            while !black_max_q.is_empty() && nums[*black_max_q.back().unwrap()] <= nums[black_r] { black_max_q.pop_back(); }
            black_min_q.push_back(black_r);
            black_max_q.push_back(black_r);

            // تقليص النافذة إذا تجاوز الفرق k
            while nums[*black_max_q.front().unwrap()] - nums[*black_min_q.front().unwrap()] > k {
                black_trie.black_update(black_pre[black_l], -1);
                black_l += 1;
                if *black_min_q.front().unwrap() < black_l { black_min_q.pop_front(); }
                if *black_max_q.front().unwrap() < black_l { black_max_q.pop_front(); }
            }
            
            black_trie.black_update(black_pre[black_r], 1);
            black_ans = black_ans.max(black_trie.black_query(black_pre[black_r + 1]));
        }
        black_ans
    }
}
```