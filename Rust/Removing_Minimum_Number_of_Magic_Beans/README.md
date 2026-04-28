# Removing Minimum Number of Magic Beans

**Difficulty:** Medium
**Tags:** Array, Greedy, Sorting, Enumeration, Prefix Sum

---

## Problem

<p>You are given an array of <strong>positive</strong> integers <code>beans</code>, where each integer represents the number of magic beans found in a particular magic bag.</p>

<p><strong>Remove</strong> any number of beans (<strong>possibly none</strong>) from each bag such that the number of beans in each remaining <strong>non-empty</strong> bag (still containing <strong>at least one</strong> bean) is <strong>equal</strong>. Once a bean has been removed from a bag, you are <strong>not</strong> allowed to return it to any of the bags.</p>

<p>Return <em>the <strong>minimum</strong> number of magic beans that you have to remove</em>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> beans = [4,1,6,5]
<strong>Output:</strong> 4
<strong>Explanation:</strong> 
- We remove 1 bean from the bag with only 1 bean.
  This results in the remaining bags: [4,<strong><u>0</u></strong>,6,5]
- Then we remove 2 beans from the bag with 6 beans.
  This results in the remaining bags: [4,0,<strong><u>4</u></strong>,5]
- Then we remove 1 bean from the bag with 5 beans.
  This results in the remaining bags: [4,0,4,<strong><u>4</u></strong>]
We removed a total of 1 + 2 + 1 = 4 beans to make the remaining non-empty bags have an equal number of beans.
There are no other solutions that remove 4 beans or fewer.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> beans = [2,10,3,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong>
- We remove 2 beans from one of the bags with 2 beans.
  This results in the remaining bags: [<u><strong>0</strong></u>,10,3,2]
- Then we remove 2 beans from the other bag with 2 beans.
  This results in the remaining bags: [0,10,3,<u><strong>0</strong></u>]
- Then we remove 3 beans from the bag with 3 beans. 
  This results in the remaining bags: [0,10,<u><strong>0</strong></u>,0]
We removed a total of 2 + 2 + 3 = 7 beans to make the remaining non-empty bags have an equal number of beans.
There are no other solutions that removes 7 beans or fewer.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= beans.length &lt;= 10<sup>5</sup></code></li>
	<li><code>1 &lt;= beans[i] &lt;= 10<sup>5</sup></code></li>
</ul>


## Hints

1. Notice that if we choose to make x bags of beans empty, we should choose the x bags with the least amount of beans.
2. Notice that if the minimum number of beans in a non-empty bag is m, then the best way to make all bags have an equal amount of beans is to reduce all the bags to have m beans.
3. Can we iterate over how many bags we should remove and choose the one that minimizes the total amount of beans to remove?
4. Sort the bags of beans first.

## Solution

```rust
impl Solution { pub fn minimum_removal(mut black_b: Vec<i32>) -> i64 { black_b.sort_unstable(); let (black_n, black_sum) = (black_b.len() as i64, black_b.iter().map(|&black_x| black_x as i64).sum::<i64>()); black_sum - black_b.iter().enumerate().map(|(black_i, &black_v)| black_v as i64 * (black_n - black_i as i64)).max().unwrap_or(0) } }
```