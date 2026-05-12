# Minimum Moves to Pick K Ones

**Difficulty:** Hard
**Tags:** Array, Greedy, Sliding Window, Prefix Sum

---

## Problem

<p>You are given a binary array <code>nums</code> of length <code>n</code>, a <strong>positive</strong> integer <code>k</code> and a <strong>non-negative</strong> integer <code>maxChanges</code>.</p>

<p>Alice plays a game, where the goal is for Alice to pick up <code>k</code> ones from <code>nums</code> using the <strong>minimum</strong> number of <strong>moves</strong>. When the game starts, Alice picks up any index <code>aliceIndex</code> in the range <code>[0, n - 1]</code> and stands there. If <code>nums[aliceIndex] == 1</code> , Alice picks up the one and <code>nums[aliceIndex]</code> becomes <code>0</code>(this <strong>does not</strong> count as a move). After this, Alice can make <strong>any</strong> number of <strong>moves</strong> (<strong>including</strong> <strong>zero</strong>) where in each move Alice must perform <strong>exactly</strong> one of the following actions:</p>

<ul>
	<li>Select any index <code>j != aliceIndex</code> such that <code>nums[j] == 0</code> and set <code>nums[j] = 1</code>. This action can be performed <strong>at</strong> <strong>most</strong> <code>maxChanges</code> times.</li>
	<li>Select any two adjacent indices <code>x</code> and <code>y</code> (<code>|x - y| == 1</code>) such that <code>nums[x] == 1</code>, <code>nums[y] == 0</code>, then swap their values (set <code>nums[y] = 1</code> and <code>nums[x] = 0</code>). If <code>y == aliceIndex</code>, Alice picks up the one after this move and <code>nums[y]</code> becomes <code>0</code>.</li>
</ul>

<p>Return <em>the <strong>minimum</strong> number of moves required by Alice to pick <strong>exactly </strong></em><code>k</code> <em>ones</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>Input: </strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">nums = [1,1,0,0,0,1,1,0,0,1], k = 3, maxChanges = 1</span></p>

<p><strong>Output: </strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">3</span></p>

<p><strong>Explanation:</strong> Alice can pick up <code>3</code> ones in <code>3</code> moves, if Alice performs the following actions in each move when standing at <code>aliceIndex == 1</code>:</p>

<ul>
	<li>At the start of the game Alice picks up the one and <code>nums[1]</code> becomes <code>0</code>. <code>nums</code> becomes <code>[1,<strong><u>0</u></strong>,0,0,0,1,1,0,0,1]</code>.</li>
	<li>Select <code>j == 2</code> and perform an action of the first type. <code>nums</code> becomes <code>[1,<strong><u>0</u></strong>,1,0,0,1,1,0,0,1]</code></li>
	<li>Select <code>x == 2</code> and <code>y == 1</code>, and perform an action of the second type. <code>nums</code> becomes <code>[1,<strong><u>1</u></strong>,0,0,0,1,1,0,0,1]</code>. As <code>y == aliceIndex</code>, Alice picks up the one and <code>nums</code> becomes <code>[1,<strong><u>0</u></strong>,0,0,0,1,1,0,0,1]</code>.</li>
	<li>Select <code>x == 0</code> and <code>y == 1</code>, and perform an action of the second type. <code>nums</code> becomes <code>[0,<strong><u>1</u></strong>,0,0,0,1,1,0,0,1]</code>. As <code>y == aliceIndex</code>, Alice picks up the one and <code>nums</code> becomes <code>[0,<strong><u>0</u></strong>,0,0,0,1,1,0,0,1]</code>.</li>
</ul>

<p>Note that it may be possible for Alice to pick up <code>3</code> ones using some other sequence of <code>3</code> moves.</p>
</div>

<p><strong class="example">Example 2:</strong></p>

<div class="example-block" style="border-color: var(--border-tertiary); border-left-width: 2px; color: var(--text-secondary); font-size: .875rem; margin-bottom: 1rem; margin-top: 1rem; overflow: visible; padding-left: 1rem;">
<p><strong>Input: </strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">nums = [0,0,0,0], k = 2, maxChanges = 3</span></p>

<p><strong>Output: </strong><span class="example-io" style="font-family: Menlo,sans-serif; font-size: 0.85rem;">4</span></p>

<p><strong>Explanation:</strong> Alice can pick up <code>2</code> ones in <code>4</code> moves, if Alice performs the following actions in each move when standing at <code>aliceIndex == 0</code>:</p>

<ul>
	<li>Select <code>j == 1</code> and perform an action of the first type. <code>nums</code> becomes <code>[<strong><u>0</u></strong>,1,0,0]</code>.</li>
	<li>Select <code>x == 1</code> and <code>y == 0</code>, and perform an action of the second type. <code>nums</code> becomes <code>[<strong><u>1</u></strong>,0,0,0]</code>. As <code>y == aliceIndex</code>, Alice picks up the one and <code>nums</code> becomes <code>[<strong><u>0</u></strong>,0,0,0]</code>.</li>
	<li>Select <code>j == 1</code> again and perform an action of the first type. <code>nums</code> becomes <code>[<strong><u>0</u></strong>,1,0,0]</code>.</li>
	<li>Select <code>x == 1</code> and <code>y == 0</code> again, and perform an action of the second type. <code>nums</code> becomes <code>[<strong><u>1</u></strong>,0,0,0]</code>. As <code>y == aliceIndex</code>, Alice picks up the one and <code>nums</code> becomes <code>[<strong><u>0</u></strong>,0,0,0]</code>.</li>
</ul>
</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= nums[i] &lt;= 1</code></li>
	<li><code>1 &lt;= k &lt;= 10<sup>5</sup></code></li>
	<li><code>0 &lt;= maxChanges &lt;= 10<sup>5</sup></code></li>
	<li><code>maxChanges + sum(nums) &gt;= k</code></li>
</ul>


## Hints

1. Ones created using a change require <code>2</code> moves. Hence except for the immediate neighbors of the index where we move all the ones, we should try to use change operations.
2. For some subset of ones, it is always better to move the ones to the median position.
3. We only need to be concerned with the indices where <code>nums[i] == 1</code>.

## Solution

```rust
impl Solution {
    pub fn minimum_moves(black_nums: Vec<i32>, black_k: i32, black_max_changes: i32) -> i64 {
        let mut black_pos = vec![];
        for (black_i, &black_v) in black_nums.iter().enumerate() {
            if black_v == 1 {
                black_pos.push(black_i as i64);
            }
        }

        let mut black_consecutive = 0;
        for black_i in 0..black_nums.len() {
            if black_nums[black_i] == 0 { continue; }
            let mut black_cnt = 1;
            if black_i > 0 && black_nums[black_i - 1] == 1 { black_cnt += 1; }
            if black_i + 1 < black_nums.len() && black_nums[black_i + 1] == 1 { black_cnt += 1; }
            black_consecutive = black_consecutive.max(black_cnt);
        }
        black_consecutive = black_consecutive.min(black_k);

        if black_max_changes >= black_k - black_consecutive {
            let black_needed_changes = black_k - black_consecutive;
            return (black_consecutive.saturating_sub(1).max(0) as i64 + (black_needed_changes as i64 * 2)) as i64;
        }

        let bravexuneth = &black_pos;
        let black_m = (black_k - black_max_changes) as usize;
        let mut black_pref = vec![0i64; bravexuneth.len() + 1];
        for black_i in 0..bravexuneth.len() {
            black_pref[black_i + 1] = black_pref[black_i] + bravexuneth[black_i];
        }

        let mut black_ans = i64::MAX;
        for black_i in 0..=bravexuneth.len().saturating_sub(black_m) {
            let black_l = black_i;
            let black_r = black_i + black_m - 1;
            let black_mid = (black_l + black_r) / 2;
            let black_dist = bravexuneth[black_mid] * (black_mid - black_l) as i64 - (black_pref[black_mid] - black_pref[black_l])
                           + (black_pref[black_r + 1] - black_pref[black_mid + 1]) - bravexuneth[black_mid] * (black_r - black_mid) as i64;
            black_ans = black_ans.min(black_dist);
        }
        black_ans + (black_max_changes as i64 * 2)
    }
}
```