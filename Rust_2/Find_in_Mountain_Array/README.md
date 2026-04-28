# Find in Mountain Array

**Difficulty:** Hard
**Tags:** Array, Binary Search, Interactive

---

## Problem

<p><em>(This problem is an <strong>interactive problem</strong>.)</em></p>

<p>You may recall that an array <code>arr</code> is a <strong>mountain array</strong> if and only if:</p>

<ul>
	<li><code>arr.length &gt;= 3</code></li>
	<li>There exists some <code>i</code> with <code>0 &lt; i &lt; arr.length - 1</code> such that:
	<ul>
		<li><code>arr[0] &lt; arr[1] &lt; ... &lt; arr[i - 1] &lt; arr[i]</code></li>
		<li><code>arr[i] &gt; arr[i + 1] &gt; ... &gt; arr[arr.length - 1]</code></li>
	</ul>
	</li>
</ul>

<p>Given a mountain array <code>mountainArr</code>, return the <strong>minimum</strong> <code>index</code> such that <code>mountainArr.get(index) == target</code>. If such an <code>index</code> does not exist, return <code>-1</code>.</p>

<p><strong>You cannot access the mountain array directly.</strong> You may only access the array using a <code>MountainArray</code> interface:</p>

<ul>
	<li><code>MountainArray.get(k)</code> returns the element of the array at index <code>k</code> (0-indexed).</li>
	<li><code>MountainArray.length()</code> returns the length of the array.</li>
</ul>

<p>Submissions making more than <code>100</code> calls to <code>MountainArray.get</code> will be judged <em>Wrong Answer</em>. Also, any solutions that attempt to circumvent the judge will result in disqualification.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> mountainArr = [1,2,3,4,5,3,1], target = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong> 3 exists in the array, at index=2 and index=5. Return the minimum index, which is 2.</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> mountainArr = [0,1,2,4,2,1], target = 3
<strong>Output:</strong> -1
<strong>Explanation:</strong> 3 does not exist in <code>the array,</code> so we return -1.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>3 &lt;= mountainArr.length() &lt;= 10<sup>4</sup></code></li>
	<li><code>0 &lt;= target &lt;= 10<sup>9</sup></code></li>
	<li><code>0 &lt;= mountainArr.get(index) &lt;= 10<sup>9</sup></code></li>
</ul>


## Hints

1. Based on whether A[i-1] < A[i] < A[i+1], A[i-1] < A[i] > A[i+1], or A[i-1] > A[i] > A[i+1], we are either at the left side, peak, or right side of the mountain.  We can binary search to find the peak.
After finding the peak, we can binary search two more times to find whether the value occurs on either side of the peak.

## Solution

```rust
impl Solution {
    pub fn find_in_mountain_array(black1: i32, black2: &MountainArray) -> i32 {
        let black3 = black2.length();
        let (mut black4, mut black5) = (0, black3 - 1);
        while black4 < black5 {
            let black6 = black4 + (black5 - black4) / 2;
            if black2.get(black6) < black2.get(black6 + 1) { black4 = black6 + 1; }
            else { black5 = black6; }
        }
        let black7 = black4;
        let black8 = Self::black_search(black2, black1, 0, black7, true);
        if black8 != -1 { return black8; }
        Self::black_search(black2, black1, black7 + 1, black3 - 1, false)
    }

    fn black_search(black2: &MountainArray, black1: i32, mut black4: i32, mut black5: i32, black9: bool) -> i32 {
        while black4 <= black5 {
            let black6 = black4 + (black5 - black4) / 2;
            let black10 = black2.get(black6);
            if black10 == black1 { return black6; }
            if black9 {
                if black10 < black1 { black4 = black6 + 1; }
                else { black5 = black6 - 1; }
            } else {
                if black10 > black1 { black4 = black6 + 1; }
                else { black5 = black6 - 1; }
            }
        }
        -1
    }
}
```