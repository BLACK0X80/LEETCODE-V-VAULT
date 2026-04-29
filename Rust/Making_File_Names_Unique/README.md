# Making File Names Unique

**Difficulty:** Medium
**Tags:** Array, Hash Table, String

---

## Problem

<p>Given an array of strings <code>names</code> of size <code>n</code>. You will create <code>n</code> folders in your file system <strong>such that</strong>, at the <code>i<sup>th</sup></code> minute, you will create a folder with the name <code>names[i]</code>.</p>

<p>Since two files <strong>cannot</strong> have the same name, if you enter a folder name that was previously used, the system will have a suffix addition to its name in the form of <code>(k)</code>, where, <code>k</code> is the <strong>smallest positive integer</strong> such that the obtained name remains unique.</p>

<p>Return <em>an array of strings of length </em><code>n</code> where <code>ans[i]</code> is the actual name the system will assign to the <code>i<sup>th</sup></code> folder when you create it.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> names = [&quot;pes&quot;,&quot;fifa&quot;,&quot;gta&quot;,&quot;pes(2019)&quot;]
<strong>Output:</strong> [&quot;pes&quot;,&quot;fifa&quot;,&quot;gta&quot;,&quot;pes(2019)&quot;]
<strong>Explanation:</strong> Let&#39;s see how the file system creates folder names:
&quot;pes&quot; --&gt; not assigned before, remains &quot;pes&quot;
&quot;fifa&quot; --&gt; not assigned before, remains &quot;fifa&quot;
&quot;gta&quot; --&gt; not assigned before, remains &quot;gta&quot;
&quot;pes(2019)&quot; --&gt; not assigned before, remains &quot;pes(2019)&quot;
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> names = [&quot;gta&quot;,&quot;gta(1)&quot;,&quot;gta&quot;,&quot;avalon&quot;]
<strong>Output:</strong> [&quot;gta&quot;,&quot;gta(1)&quot;,&quot;gta(2)&quot;,&quot;avalon&quot;]
<strong>Explanation:</strong> Let&#39;s see how the file system creates folder names:
&quot;gta&quot; --&gt; not assigned before, remains &quot;gta&quot;
&quot;gta(1)&quot; --&gt; not assigned before, remains &quot;gta(1)&quot;
&quot;gta&quot; --&gt; the name is reserved, system adds (k), since &quot;gta(1)&quot; is also reserved, systems put k = 2. it becomes &quot;gta(2)&quot;
&quot;avalon&quot; --&gt; not assigned before, remains &quot;avalon&quot;
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> names = [&quot;onepiece&quot;,&quot;onepiece(1)&quot;,&quot;onepiece(2)&quot;,&quot;onepiece(3)&quot;,&quot;onepiece&quot;]
<strong>Output:</strong> [&quot;onepiece&quot;,&quot;onepiece(1)&quot;,&quot;onepiece(2)&quot;,&quot;onepiece(3)&quot;,&quot;onepiece(4)&quot;]
<strong>Explanation:</strong> When the last folder is created, the smallest positive valid k is 4, and it becomes &quot;onepiece(4)&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= names.length &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= names[i].length &lt;= 20</code></li>
	<li><code>names[i]</code> consists of lowercase English letters, digits, and/or round brackets.</li>
</ul>


## Hints

1. Keep a map of each name and the smallest valid integer that can be appended as a suffix to it.
2. If the name is not present in the map, you can use it without adding any suffixes.
3. If the name is present in the map, append the smallest proper suffix, and add the new name to the map.

## Solution

```rust
use std::collections::HashMap;
impl Solution { pub fn get_folder_names(names: Vec<String>) -> Vec<String> { let (mut black_map, mut black_res) = (HashMap::new(), Vec::new()); for black_n in names { let mut black_curr = black_n.clone(); if let Some(&black_k) = black_map.get(&black_n) { let mut black_suffix = black_k + 1; while black_map.contains_key(&format!("{}({})", black_n, black_suffix)) { black_suffix += 1; } black_curr = format!("{}({})", black_n, black_suffix); black_map.insert(black_n, black_suffix); } black_map.insert(black_curr.clone(), 0); black_res.push(black_curr); } black_res } }
```