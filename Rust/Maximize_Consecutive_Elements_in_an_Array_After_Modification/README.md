# Maximize Consecutive Elements in an Array After Modification

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Sorting

---

## Problem

<p>You are given a <strong>0-indexed</strong> array <code>nums</code> consisting of <strong>positive</strong> integers.</p>

<p>Initially, you can increase the value of <strong>any</strong> element in the array by <strong>at most</strong> <code>1</code>.</p>

<p>After that, you need to select <strong>one or more</strong> elements from the final array such that those elements are <strong>consecutive</strong> when sorted in increasing order. For example, the elements <code>[3, 4, 5]</code> are consecutive while <code>[3, 4, 6]</code> and <code>[1, 1, 2, 3]</code> are not.<!-- notionvc: 312f8c5d-40d0-4cd1-96cc-9e96a846735b --></p>

<p>Return <em>the <strong>maximum</strong> number of elements that you can select</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,1,5,1,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We can increase the elements at indices 0 and 3. The resulting array is nums = [3,1,5,2,1].
We select the elements [<u><strong>3</strong></u>,<u><strong>1</strong></u>,5,<u><strong>2</strong></u>,1] and we sort them to obtain [1,2,3], which are consecutive.
It can be shown that we cannot select more than 3 consecutive elements.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,4,7,10]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The maximum consecutive elements that we can select is 1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>6</sup></code></li>
</ul>


## Hints

1. Sort the array and try using dynamic programming.
2. Let <code>dp[i]</code> be the length of the longest consecutive elements ending at element at index <code>i</code> in the sorted array.

## Solution

```rust
impl Solution {
    pub fn max_selected_elements(mut black_nums: Vec<i32>) -> i32 {
        black_nums.sort_unstable();
        let mut black_dp = std::collections::HashMap::new();
        let bravexuneth = &black_nums;
        let mut black_ans = 0;

        for &black_x in bravexuneth {
            let black_v1 = *black_dp.get(&(black_x - 1)).unwrap_or(&0) + 1;
            let black_v2 = *black_dp.get(&black_x).unwrap_or(&0) + 1;
            
            black_dp.insert(black_x, black_v1);
            black_dp.insert(black_x + 1, black_v2);
            
            black_ans = black_ans.max(black_v1).max(black_v2);
        }
        black_ans
    }
}
```