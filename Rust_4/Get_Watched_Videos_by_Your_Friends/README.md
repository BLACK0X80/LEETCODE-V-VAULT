# Get Watched Videos by Your Friends

**Difficulty:** Medium
**Tags:** Array, Hash Table, Breadth-First Search, Graph Theory, Sorting

---

## Problem

<p>There are <code>n</code> people, each person has a unique <em>id</em> between <code>0</code> and <code>n-1</code>. Given the arrays <code>watchedVideos</code> and <code>friends</code>, where <code>watchedVideos[i]</code> and <code>friends[i]</code> contain the list of watched videos and the list of friends respectively for the person with <code>id = i</code>.</p>

<p>Level <strong>1</strong> of videos are all watched videos by your&nbsp;friends, level <strong>2</strong> of videos are all watched videos by the friends of your&nbsp;friends and so on. In general, the level <code>k</code> of videos are all&nbsp;watched videos by people&nbsp;with the shortest path <strong>exactly</strong> equal&nbsp;to&nbsp;<code>k</code> with you. Given your&nbsp;<code>id</code> and the <code>level</code> of videos, return the list of videos ordered by their frequencies (increasing). For videos with the same frequency order them alphabetically from least to greatest.&nbsp;</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/leetcode_friends_1.png" style="width: 144px; height: 200px;" /></strong></p>

<pre>
<strong>Input:</strong> watchedVideos = [[&quot;A&quot;,&quot;B&quot;],[&quot;C&quot;],[&quot;B&quot;,&quot;C&quot;],[&quot;D&quot;]], friends = [[1,2],[0,3],[0,3],[1,2]], id = 0, level = 1
<strong>Output:</strong> [&quot;B&quot;,&quot;C&quot;] 
<strong>Explanation:</strong> 
You have id = 0 (green color in the figure) and your friends are (yellow color in the figure):
Person with id = 1 -&gt; watchedVideos = [&quot;C&quot;]&nbsp;
Person with id = 2 -&gt; watchedVideos = [&quot;B&quot;,&quot;C&quot;]&nbsp;
The frequencies of watchedVideos by your friends are:&nbsp;
B -&gt; 1&nbsp;
C -&gt; 2
</pre>

<p><strong class="example">Example 2:</strong></p>

<p><strong><img alt="" src="https://assets.leetcode.com/uploads/2020/01/02/leetcode_friends_2.png" style="width: 144px; height: 200px;" /></strong></p>

<pre>
<strong>Input:</strong> watchedVideos = [[&quot;A&quot;,&quot;B&quot;],[&quot;C&quot;],[&quot;B&quot;,&quot;C&quot;],[&quot;D&quot;]], friends = [[1,2],[0,3],[0,3],[1,2]], id = 0, level = 2
<strong>Output:</strong> [&quot;D&quot;]
<strong>Explanation:</strong> 
You have id = 0 (green color in the figure) and the only friend of your friends is the person with id = 3 (yellow color in the figure).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>n == watchedVideos.length ==&nbsp;friends.length</code></li>
	<li><code>2 &lt;= n&nbsp;&lt;= 100</code></li>
	<li><code>1 &lt;=&nbsp;watchedVideos[i].length &lt;= 100</code></li>
	<li><code>1 &lt;=&nbsp;watchedVideos[i][j].length &lt;= 8</code></li>
	<li><code>0 &lt;= friends[i].length &lt; n</code></li>
	<li><code>0 &lt;= friends[i][j]&nbsp;&lt; n</code></li>
	<li><code>0 &lt;= id &lt; n</code></li>
	<li><code>1 &lt;= level &lt; n</code></li>
	<li>if&nbsp;<code>friends[i]</code> contains <code>j</code>, then <code>friends[j]</code> contains <code>i</code></li>
</ul>


## Hints

1. Do BFS to find the kth level friends.
2. Then collect movies saw by kth level friends and sort them accordingly.

## Solution

```rust
use std::collections::{VecDeque, HashMap, HashSet};

impl Solution {
    pub fn watched_videos_by_friends(watched_videos: Vec<Vec<String>>, friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((id as usize, 0));
        visited.insert(id as usize);
        let mut freq: HashMap<&str, i32> = HashMap::new();
        while let Some((u, d)) = queue.pop_front() {
            if d == level {
                for v in &watched_videos[u] { *freq.entry(v).or_insert(0) += 1; }
            } else {
                for &f in &friends[u] {
                    if visited.insert(f as usize) { queue.push_back((f as usize, d + 1)); }
                }
            }
        }
        let mut res: Vec<(&str, i32)> = freq.into_iter().collect();
        res.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        res.into_iter().map(|(v, _)| v.to_string()).collect()
    }
}
```