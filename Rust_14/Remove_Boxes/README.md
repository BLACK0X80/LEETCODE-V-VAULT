# Remove Boxes

**Difficulty:** Hard
**Tags:** Array, Dynamic Programming, Memoization

---

## Problem

<p>You are given several <code>boxes</code> with different colors represented by different positive numbers.</p>

<p>You may experience several rounds to remove boxes until there is no box left. Each time you can choose some continuous boxes with the same color (i.e., composed of <code>k</code> boxes, <code>k &gt;= 1</code>), remove them and get <code>k * k</code> points.</p>

<p>Return <em>the maximum points you can get</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> boxes = [1,3,2,2,2,3,4,3,1]
<strong>Output:</strong> 23
<strong>Explanation:</strong>
[1, 3, 2, 2, 2, 3, 4, 3, 1] 
----&gt; [1, 3, 3, 4, 3, 1] (3*3=9 points) 
----&gt; [1, 3, 3, 3, 1] (1*1=1 points) 
----&gt; [1, 1] (3*3=9 points) 
----&gt; [] (2*2=4 points)
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> boxes = [1,1,1]
<strong>Output:</strong> 9
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> boxes = [1]
<strong>Output:</strong> 1
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= boxes.length &lt;= 100</code></li>
	<li><code>1 &lt;= boxes[i]&nbsp;&lt;= 100</code></li>
</ul>



## Solution

```rust
impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut memo = vec![vec![vec![-1i32; n]; n]; n];
        Self::dp(&boxes, 0, n as i32 - 1, 0, &mut memo)
    }

    fn dp(boxes: &[i32], l: i32, r: i32, k: i32, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if l > r { return 0; }
        let (l, r, k) = (l as usize, r as usize, k as usize);
        if memo[l][r][k] != -1 { return memo[l][r][k]; }

        let mut res = Self::dp(boxes, l as i32, r as i32 - 1, 0, memo) + (k + 1) as i32 * (k + 1) as i32;

        for i in l..r {
            if boxes[i] == boxes[r] {
                res = res.max(
                    Self::dp(boxes, l as i32, i as i32, k as i32 + 1, memo) +
                    Self::dp(boxes, i as i32 + 1, r as i32 - 1, 0, memo)
                );
            }
        }

        memo[l][r][k] = res;
        res
    }
}
```