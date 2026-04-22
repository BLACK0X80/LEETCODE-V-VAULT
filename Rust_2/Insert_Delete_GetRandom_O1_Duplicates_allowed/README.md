# Insert Delete GetRandom O(1) - Duplicates allowed

**Difficulty:** Hard
**Tags:** Array, Hash Table, Math, Design, Randomized

---

## Problem

<p><code>RandomizedCollection</code> is a data structure that contains a collection of numbers, possibly duplicates (i.e., a multiset). It should support inserting and removing specific elements and also reporting a random element.</p>

<p>Implement the <code>RandomizedCollection</code> class:</p>

<ul>
	<li><code>RandomizedCollection()</code> Initializes the empty <code>RandomizedCollection</code> object.</li>
	<li><code>bool insert(int val)</code> Inserts an item <code>val</code> into the multiset, even if the item is already present. Returns <code>true</code> if the item is not present, <code>false</code> otherwise.</li>
	<li><code>bool remove(int val)</code> Removes an item <code>val</code> from the multiset if present. Returns <code>true</code> if the item is present, <code>false</code> otherwise. Note that if <code>val</code> has multiple occurrences in the multiset, we only remove one of them.</li>
	<li><code>int getRandom()</code> Returns a random element from the current multiset of elements. The probability of each element being returned is <strong>linearly related</strong> to the number of the same values the multiset contains.</li>
</ul>

<p>You must implement the functions of the class such that each function works on <strong>average</strong> <code>O(1)</code> time complexity.</p>

<p><strong>Note:</strong> The test cases are generated such that <code>getRandom</code> will only be called if there is <strong>at least one</strong> item in the <code>RandomizedCollection</code>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;RandomizedCollection&quot;, &quot;insert&quot;, &quot;insert&quot;, &quot;insert&quot;, &quot;getRandom&quot;, &quot;remove&quot;, &quot;getRandom&quot;]
[[], [1], [1], [2], [], [1], []]
<strong>Output</strong>
[null, true, false, true, 2, true, 1]

<strong>Explanation</strong>
RandomizedCollection randomizedCollection = new RandomizedCollection();
randomizedCollection.insert(1);   // return true since the collection does not contain 1.
                                  // Inserts 1 into the collection.
randomizedCollection.insert(1);   // return false since the collection contains 1.
                                  // Inserts another 1 into the collection. Collection now contains [1,1].
randomizedCollection.insert(2);   // return true since the collection does not contain 2.
                                  // Inserts 2 into the collection. Collection now contains [1,1,2].
randomizedCollection.getRandom(); // getRandom should:
                                  // - return 1 with probability 2/3, or
                                  // - return 2 with probability 1/3.
randomizedCollection.remove(1);   // return true since the collection contains 1.
                                  // Removes 1 from the collection. Collection now contains [1,2].
randomizedCollection.getRandom(); // getRandom should return 1 or 2, both equally likely.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>-2<sup>31</sup> &lt;= val &lt;= 2<sup>31</sup> - 1</code></li>
	<li>At most <code>2 * 10<sup>5</sup></code> calls <strong>in total</strong> will be made to <code>insert</code>, <code>remove</code>, and <code>getRandom</code>.</li>
	<li>There will be <strong>at least one</strong> element in the data structure when <code>getRandom</code> is called.</li>
</ul>



## Solution

```rust
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    map: HashMap<i32, HashSet<usize>>,
    vals: Vec<i32>,
}

impl RandomizedCollection {
    fn new() -> Self { RandomizedCollection { map: HashMap::new(), vals: vec![] } }

    fn insert(&mut self, val: i32) -> bool {
        let idx = self.vals.len();
        self.vals.push(val);
        let e = self.map.entry(val).or_default();
        let fresh = e.is_empty();
        e.insert(idx);
        fresh
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) || self.map[&val].is_empty() { return false; }
        let idx = *self.map[&val].iter().next().unwrap();
        self.map.get_mut(&val).unwrap().remove(&idx);
        let last_idx = self.vals.len() - 1;
        if idx != last_idx {
            let last_val = self.vals[last_idx];
            self.vals[idx] = last_val;
            self.map.get_mut(&last_val).unwrap().remove(&last_idx);
            self.map.get_mut(&last_val).unwrap().insert(idx);
        }
        self.vals.pop();
        true
    }

    fn get_random(&self) -> i32 {
        self.vals[rand::random::<usize>() % self.vals.len()]
    }
}
```