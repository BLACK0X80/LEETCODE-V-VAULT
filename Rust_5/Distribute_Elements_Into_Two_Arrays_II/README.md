# Distribute Elements Into Two Arrays II

**Difficulty:** Hard
**Tags:** Array, Binary Indexed Tree, Segment Tree, Simulation

---

## Problem

<p>You are given a <strong>1-indexed</strong> array of integers <code>nums</code> of length <code>n</code>.</p>

<p>We define a function <code>greaterCount</code> such that <code>greaterCount(arr, val)</code> returns the number of elements in <code>arr</code> that are <strong>strictly greater</strong> than <code>val</code>.</p>

<p>You need to distribute all the elements of <code>nums</code> between two arrays <code>arr1</code> and <code>arr2</code> using <code>n</code> operations. In the first operation, append <code>nums[1]</code> to <code>arr1</code>. In the second operation, append <code>nums[2]</code> to <code>arr2</code>. Afterwards, in the <code>i<sup>th</sup></code> operation:</p>

<ul>
	<li>If <code>greaterCount(arr1, nums[i]) &gt; greaterCount(arr2, nums[i])</code>, append <code>nums[i]</code> to <code>arr1</code>.</li>
	<li>If <code>greaterCount(arr1, nums[i]) &lt; greaterCount(arr2, nums[i])</code>, append <code>nums[i]</code> to <code>arr2</code>.</li>
	<li>If <code>greaterCount(arr1, nums[i]) == greaterCount(arr2, nums[i])</code>, append <code>nums[i]</code> to the array with a <strong>lesser</strong> number of elements.</li>
	<li>If there is still a tie, append <code>nums[i]</code> to <code>arr1</code>.</li>
</ul>

<p>The array <code>result</code> is formed by concatenating the arrays <code>arr1</code> and <code>arr2</code>. For example, if <code>arr1 == [1,2,3]</code> and <code>arr2 == [4,5,6]</code>, then <code>result = [1,2,3,4,5,6]</code>.</p>

<p>Return <em>the integer array</em> <code>result</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [2,1,3,3]
<strong>Output:</strong> [2,3,1,3]
<strong>Explanation:</strong> After the first 2 operations, arr1 = [2] and arr2 = [1].
In the 3<sup>rd</sup> operation, the number of elements greater than 3 is zero in both arrays. Also, the lengths are equal, hence, append nums[3] to arr1.
In the 4<sup>th</sup> operation, the number of elements greater than 3 is zero in both arrays. As the length of arr2 is lesser, hence, append nums[4] to arr2.
After 4 operations, arr1 = [2,3] and arr2 = [1,3].
Hence, the array result formed by concatenation is [2,3,1,3].
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [5,14,3,1,2]
<strong>Output:</strong> [5,3,1,2,14]
<strong>Explanation:</strong> After the first 2 operations, arr1 = [5] and arr2 = [14].
In the 3<sup>rd</sup> operation, the number of elements greater than 3 is one in both arrays. Also, the lengths are equal, hence, append nums[3] to arr1.
In the 4<sup>th</sup> operation, the number of elements greater than 1 is greater in arr1 than arr2 (2 &gt; 1). Hence, append nums[4] to arr1.
In the 5<sup>th</sup> operation, the number of elements greater than 2 is greater in arr1 than arr2 (2 &gt; 1). Hence, append nums[5] to arr1.
After 5 operations, arr1 = [5,3,1,2] and arr2 = [14].
Hence, the array result formed by concatenation is [5,3,1,2,14].
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> nums = [3,3,3,3]
<strong>Output:</strong> [3,3,3,3]
<strong>Explanation:</strong> At the end of 4 operations, arr1 = [3,3] and arr2 = [3,3].
Hence, the array result formed by concatenation is [3,3,3,3].
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= n &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. We need a data structure that counts the number of integers greater than a given value <code>x</code> and supports insertion.
2. Use Segment Tree or Binary Indexed Tree by compressing the numbers to the range <code>[1,n]</code>.

## Solution

```rust
impl Solution {
    pub fn result_array(black1: Vec<i32>) -> Vec<i32> {
        let mut black2 = black1.clone();
        black2.sort();
        black2.dedup();
        let black3 = |black4: i32| black2.binary_search(&black4).unwrap() as i32 + 1;
        let mut black5 = vec![0; black2.len() + 1];
        let mut black6 = vec![0; black2.len() + 1];
        let black7 = |black8: &mut Vec<i32>, black9: i32| {
            let mut black10 = black9;
            while black10 < black8.len() as i32 { black8[black10 as usize] += 1; black10 += black10 & -black10; }
        };
        let black11 = |black12: &Vec<i32>, black13: i32| {
            let mut black14 = 0; let mut black15 = black13;
            while black15 > 0 { black14 += black12[black15 as usize]; black15 -= black15 & -black15; }
            black14
        };
        let (mut black16, mut black17) = (vec![black1[0]], vec![black1[1]]);
        black7(&mut black5, black3(black1[0]));
        black7(&mut black6, black3(black1[1]));
        for &black18 in &black1[2..] {
            let black19 = black3(black18);
            let black20 = black16.len() as i32 - black11(&black5, black19);
            let black21 = black17.len() as i32 - black11(&black6, black19);
            if black20 > black21 || (black20 == black21 && black16.len() <= black17.len()) {
                black16.push(black18); black7(&mut black5, black19);
            } else {
                black17.push(black18); black7(&mut black6, black19);
            }
        }
        black16.extend(black17); black16
    }
}
```