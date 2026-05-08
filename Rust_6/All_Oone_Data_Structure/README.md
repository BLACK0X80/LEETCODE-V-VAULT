# All O`one Data Structure

**Difficulty:** Hard
**Tags:** Hash Table, Linked List, Design, Doubly-Linked List

---

## Problem

<p>Design a data structure to store the strings&#39; count with the ability to return the strings with minimum and maximum counts.</p>

<p>Implement the <code>AllOne</code> class:</p>

<ul>
	<li><code>AllOne()</code> Initializes the object of the data structure.</li>
	<li><code>inc(String key)</code> Increments the count of the string <code>key</code> by <code>1</code>. If <code>key</code> does not exist in the data structure, insert it with count <code>1</code>.</li>
	<li><code>dec(String key)</code> Decrements the count of the string <code>key</code> by <code>1</code>. If the count of <code>key</code> is <code>0</code> after the decrement, remove it from the data structure. It is guaranteed that <code>key</code> exists in the data structure before the decrement.</li>
	<li><code>getMaxKey()</code> Returns one of the keys with the maximal count. If no element exists, return an empty string <code>&quot;&quot;</code>.</li>
	<li><code>getMinKey()</code> Returns one of the keys with the minimum count. If no element exists, return an empty string <code>&quot;&quot;</code>.</li>
</ul>

<p><strong>Note</strong> that each function must run in <code>O(1)</code> average time complexity.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;AllOne&quot;, &quot;inc&quot;, &quot;inc&quot;, &quot;getMaxKey&quot;, &quot;getMinKey&quot;, &quot;inc&quot;, &quot;getMaxKey&quot;, &quot;getMinKey&quot;]
[[], [&quot;hello&quot;], [&quot;hello&quot;], [], [], [&quot;leet&quot;], [], []]
<strong>Output</strong>
[null, null, null, &quot;hello&quot;, &quot;hello&quot;, null, &quot;hello&quot;, &quot;leet&quot;]

<strong>Explanation</strong>
AllOne allOne = new AllOne();
allOne.inc(&quot;hello&quot;);
allOne.inc(&quot;hello&quot;);
allOne.getMaxKey(); // return &quot;hello&quot;
allOne.getMinKey(); // return &quot;hello&quot;
allOne.inc(&quot;leet&quot;);
allOne.getMaxKey(); // return &quot;hello&quot;
allOne.getMinKey(); // return &quot;leet&quot;
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= key.length &lt;= 10</code></li>
	<li><code>key</code> consists of lowercase English letters.</li>
	<li>It is guaranteed that for each call to <code>dec</code>, <code>key</code> is existing in the data structure.</li>
	<li>At most <code>5 * 10<sup>4</sup></code>&nbsp;calls will be made to <code>inc</code>, <code>dec</code>, <code>getMaxKey</code>, and <code>getMinKey</code>.</li>
</ul>



## Solution

```rust
use std::collections::{HashMap, BTreeMap, HashSet};

struct AllOne {
    key_cnt: HashMap<String, i32>,
    cnt_keys: BTreeMap<i32, HashSet<String>>,
}

impl AllOne {
    fn new() -> Self { AllOne { key_cnt: HashMap::new(), cnt_keys: BTreeMap::new() } }

    fn inc(&mut self, key: String) {
        let cnt = self.key_cnt.get(&key).copied().unwrap_or(0);
        if cnt > 0 {
            self.cnt_keys.get_mut(&cnt).unwrap().remove(&key);
            if self.cnt_keys[&cnt].is_empty() { self.cnt_keys.remove(&cnt); }
        }
        self.key_cnt.insert(key.clone(), cnt + 1);
        self.cnt_keys.entry(cnt + 1).or_default().insert(key);
    }

    fn dec(&mut self, key: String) {
        let cnt = *self.key_cnt.get(&key).unwrap();
        self.cnt_keys.get_mut(&cnt).unwrap().remove(&key);
        if self.cnt_keys[&cnt].is_empty() { self.cnt_keys.remove(&cnt); }
        if cnt == 1 { self.key_cnt.remove(&key); }
        else {
            self.key_cnt.insert(key.clone(), cnt - 1);
            self.cnt_keys.entry(cnt - 1).or_default().insert(key);
        }
    }

    fn get_max_key(&self) -> String {
        self.cnt_keys.iter().next_back().and_then(|(_,s)| s.iter().next()).cloned().unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.cnt_keys.iter().next().and_then(|(_,s)| s.iter().next()).cloned().unwrap_or_default()
    }
}
```