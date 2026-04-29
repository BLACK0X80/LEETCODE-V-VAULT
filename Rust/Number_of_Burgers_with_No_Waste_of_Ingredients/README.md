# Number of Burgers with No Waste of Ingredients

**Difficulty:** Medium
**Tags:** Math

---

## Problem

<p>Given two integers <code>tomatoSlices</code> and <code>cheeseSlices</code>. The ingredients of different burgers are as follows:</p>

<ul>
	<li><strong>Jumbo Burger:</strong> <code>4</code> tomato slices and <code>1</code> cheese slice.</li>
	<li><strong>Small Burger:</strong> <code>2</code> Tomato slices and <code>1</code> cheese slice.</li>
</ul>

<p>Return <code>[total_jumbo, total_small]</code> so that the number of remaining <code>tomatoSlices</code> equal to <code>0</code> and the number of remaining <code>cheeseSlices</code> equal to <code>0</code>. If it is not possible to make the remaining <code>tomatoSlices</code> and <code>cheeseSlices</code> equal to <code>0</code> return <code>[]</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> tomatoSlices = 16, cheeseSlices = 7
<strong>Output:</strong> [1,6]
<strong>Explantion:</strong> To make one jumbo burger and 6 small burgers we need 4*1 + 2*6 = 16 tomato and 1 + 6 = 7 cheese.
There will be no remaining ingredients.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> tomatoSlices = 17, cheeseSlices = 4
<strong>Output:</strong> []
<strong>Explantion:</strong> There will be no way to use all ingredients to make small and jumbo burgers.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> tomatoSlices = 4, cheeseSlices = 17
<strong>Output:</strong> []
<strong>Explantion:</strong> Making 1 jumbo burger there will be 16 cheese remaining and making 2 small burgers there will be 15 cheese remaining.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>0 &lt;= tomatoSlices, cheeseSlices &lt;= 10<sup>7</sup></code></li>
</ul>


## Hints

1. Can we have an answer if the number of tomatoes is odd ?
2. If we have answer will be there multiple answers or just one answer ?
3. Let us define number of jumbo burgers as X and number of small burgers as Y
We have to find an x and y in this equation
4. 1. 4X + 2Y = tomato
5. 2. X + Y = cheese

## Solution

```rust
impl Solution { pub fn num_of_burgers(black_t: i32, black_c: i32) -> Vec<i32> { let black_two_j = black_t - 2 * black_c; if black_two_j >= 0 && black_two_j % 2 == 0 && black_c - black_two_j / 2 >= 0 { vec![black_two_j / 2, black_c - black_two_j / 2] } else { vec![] } } }
```