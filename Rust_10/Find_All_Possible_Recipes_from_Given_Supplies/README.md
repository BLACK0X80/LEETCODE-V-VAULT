# Find All Possible Recipes from Given Supplies

**Difficulty:** Medium
**Tags:** Array, Hash Table, String, Graph Theory, Topological Sort

---

## Problem

<p>You have information about <code>n</code> different recipes. You are given a string array <code>recipes</code> and a 2D string array <code>ingredients</code>. The <code>i<sup>th</sup></code> recipe has the name <code>recipes[i]</code>, and you can <strong>create</strong> it if you have <strong>all</strong> the needed ingredients from <code>ingredients[i]</code>. A recipe can also be an ingredient for <strong>other </strong>recipes, i.e., <code>ingredients[i]</code> may contain a string that is in <code>recipes</code>.</p>

<p>You are also given a string array <code>supplies</code> containing all the ingredients that you initially have, and you have an infinite supply of all of them.</p>

<p>Return <em>a list of all the recipes that you can create. </em>You may return the answer in <strong>any order</strong>.</p>

<p>Note that two recipes may contain each other in their ingredients.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> recipes = [&quot;bread&quot;], ingredients = [[&quot;yeast&quot;,&quot;flour&quot;]], supplies = [&quot;yeast&quot;,&quot;flour&quot;,&quot;corn&quot;]
<strong>Output:</strong> [&quot;bread&quot;]
<strong>Explanation:</strong>
We can create &quot;bread&quot; since we have the ingredients &quot;yeast&quot; and &quot;flour&quot;.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> recipes = [&quot;bread&quot;,&quot;sandwich&quot;], ingredients = [[&quot;yeast&quot;,&quot;flour&quot;],[&quot;bread&quot;,&quot;meat&quot;]], supplies = [&quot;yeast&quot;,&quot;flour&quot;,&quot;meat&quot;]
<strong>Output:</strong> [&quot;bread&quot;,&quot;sandwich&quot;]
<strong>Explanation:</strong>
We can create &quot;bread&quot; since we have the ingredients &quot;yeast&quot; and &quot;flour&quot;.
We can create &quot;sandwich&quot; since we have the ingredient &quot;meat&quot; and can create the ingredient &quot;bread&quot;.
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> recipes = [&quot;bread&quot;,&quot;sandwich&quot;,&quot;burger&quot;], ingredients = [[&quot;yeast&quot;,&quot;flour&quot;],[&quot;bread&quot;,&quot;meat&quot;],[&quot;sandwich&quot;,&quot;meat&quot;,&quot;bread&quot;]], supplies = [&quot;yeast&quot;,&quot;flour&quot;,&quot;meat&quot;]
<strong>Output:</strong> [&quot;bread&quot;,&quot;sandwich&quot;,&quot;burger&quot;]
<strong>Explanation:</strong>
We can create &quot;bread&quot; since we have the ingredients &quot;yeast&quot; and &quot;flour&quot;.
We can create &quot;sandwich&quot; since we have the ingredient &quot;meat&quot; and can create the ingredient &quot;bread&quot;.
We can create &quot;burger&quot; since we have the ingredient &quot;meat&quot; and can create the ingredients &quot;bread&quot; and &quot;sandwich&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == recipes.length == ingredients.length</code></li>
	<li><code>1 &lt;= n &lt;= 100</code></li>
	<li><code>1 &lt;= ingredients[i].length, supplies.length &lt;= 100</code></li>
	<li><code>1 &lt;= recipes[i].length, ingredients[i][j].length, supplies[k].length &lt;= 10</code></li>
	<li><code>recipes[i], ingredients[i][j]</code>, and <code>supplies[k]</code> consist only of lowercase English letters.</li>
	<li>All the values of <code>recipes</code> and <code>supplies</code>&nbsp;combined are unique.</li>
	<li>Each <code>ingredients[i]</code> does not contain any duplicate values.</li>
</ul>


## Hints

1. Can we use a data structure to quickly query whether we have a certain ingredient?
2. Once we verify that we can make a recipe, we can add it to our ingredient data structure. We can then check if we can make more recipes as a result of this.

## Solution

```rust
impl Solution { pub fn find_all_recipes(recipes: Vec<String>, ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> { let (mut black_adj, mut black_deg, mut black_q) = (std::collections::HashMap::new(), std::collections::HashMap::new(), supplies.into_iter().collect::<std::collections::VecDeque<_>>()); for (r, ing) in recipes.iter().zip(ingredients.iter()) { black_deg.insert(r.clone(), ing.len()); for i in ing { black_adj.entry(i.clone()).or_insert(vec![]).push(r.clone()); } } let mut black_res = vec![]; while let Some(u) = black_q.pop_front() { if let Some(neighbors) = black_adj.get(&u) { for v in neighbors { if let Some(d) = black_deg.get_mut(v) { *d -= 1; if *d == 0 { black_q.push_back(v.clone()); black_res.push(v.clone()); } } } } } black_res } }
```