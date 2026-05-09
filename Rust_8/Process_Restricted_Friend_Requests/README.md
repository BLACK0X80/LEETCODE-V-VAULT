# Process Restricted Friend Requests

**Difficulty:** Hard
**Tags:** Union-Find, Graph Theory

---

## Problem

<p>You are given an integer <code>n</code> indicating the number of people in a network. Each person is labeled from <code>0</code> to <code>n - 1</code>.</p>

<p>You are also given a <strong>0-indexed</strong> 2D integer array <code>restrictions</code>, where <code>restrictions[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> means that person <code>x<sub>i</sub></code> and person <code>y<sub>i</sub></code> <strong>cannot </strong>become <strong>friends</strong>,<strong> </strong>either <strong>directly</strong> or <strong>indirectly</strong> through other people.</p>

<p>Initially, no one is friends with each other. You are given a list of friend requests as a <strong>0-indexed</strong> 2D integer array <code>requests</code>, where <code>requests[j] = [u<sub>j</sub>, v<sub>j</sub>]</code> is a friend request between person <code>u<sub>j</sub></code> and person <code>v<sub>j</sub></code>.</p>

<p>A friend request is <strong>successful </strong>if <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code> can be <strong>friends</strong>. Each friend request is processed in the given order (i.e., <code>requests[j]</code> occurs before <code>requests[j + 1]</code>), and upon a successful request, <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code> <strong>become direct friends</strong> for all future friend requests.</p>

<p>Return <em>a <strong>boolean array</strong> </em><code>result</code>,<em> where each </em><code>result[j]</code><em> is </em><code>true</code><em> if the </em><code>j<sup>th</sup></code><em> friend request is <strong>successful</strong> or </em><code>false</code><em> if it is not</em>.</p>

<p><strong>Note:</strong> If <code>u<sub>j</sub></code> and <code>v<sub>j</sub></code> are already direct friends, the request is still <strong>successful</strong>.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 3, restrictions = [[0,1]], requests = [[0,2],[2,1]]
<strong>Output:</strong> [true,false]
<strong>Explanation:
</strong>Request 0: Person 0 and person 2 can be friends, so they become direct friends. 
Request 1: Person 2 and person 1 cannot be friends since person 0 and person 1 would be indirect friends (1--2--0).
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 3, restrictions = [[0,1]], requests = [[1,2],[0,2]]
<strong>Output:</strong> [true,false]
<strong>Explanation:
</strong>Request 0: Person 1 and person 2 can be friends, so they become direct friends.
Request 1: Person 0 and person 2 cannot be friends since person 0 and person 1 would be indirect friends (0--2--1).
</pre>

<p><strong class="example">Example 3:</strong></p>

<pre>
<strong>Input:</strong> n = 5, restrictions = [[0,1],[1,2],[2,3]], requests = [[0,4],[1,2],[3,1],[3,4]]
<strong>Output:</strong> [true,false,true,false]
<strong>Explanation:
</strong>Request 0: Person 0 and person 4 can be friends, so they become direct friends.
Request 1: Person 1 and person 2 cannot be friends since they are directly restricted.
Request 2: Person 3 and person 1 can be friends, so they become direct friends.
Request 3: Person 3 and person 4 cannot be friends since person 0 and person 1 would be indirect friends (0--4--3--1).
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>2 &lt;= n &lt;= 1000</code></li>
	<li><code>0 &lt;= restrictions.length &lt;= 1000</code></li>
	<li><code>restrictions[i].length == 2</code></li>
	<li><code>0 &lt;= x<sub>i</sub>, y<sub>i</sub> &lt;= n - 1</code></li>
	<li><code>x<sub>i</sub> != y<sub>i</sub></code></li>
	<li><code>1 &lt;= requests.length &lt;= 1000</code></li>
	<li><code>requests[j].length == 2</code></li>
	<li><code>0 &lt;= u<sub>j</sub>, v<sub>j</sub> &lt;= n - 1</code></li>
	<li><code>u<sub>j</sub> != v<sub>j</sub></code></li>
</ul>


## Hints

1. For each request, we could loop through all restrictions. Can you think of doing a check-in close to O(1)?
2. Could you use Union Find?

## Solution

```rust
struct Black {
    parent: Vec<usize>,
}

impl Black {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect() }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut x = x;
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }
    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx != ry {
            self.parent[rx] = ry;
        }
    }
}

impl Solution {
    pub fn friend_requests(n: i32, restrictions: Vec<Vec<i32>>, requests: Vec<Vec<i32>>) -> Vec<bool> {
        let mut black = Black::new(n as usize);
        let mut res = Vec::new();
        
        for req in &requests {
            let u = req[0] as usize;
            let v = req[1] as usize;
            let ru = black.find(u);
            let rv = black.find(v);
            
            if ru == rv {
                res.push(true);
                continue;
            }
            
            let mut possible = true;
            for rest in &restrictions {
                let x = rest[0] as usize;
                let y = rest[1] as usize;
                let rx = black.find(x);
                let ry = black.find(y);
                
                if (rx == ru && ry == rv) || (rx == rv && ry == ru) {
                    possible = false;
                    break;
                }
            }
            
            if possible {
                black.union(u, v);
                res.push(true);
            } else {
                res.push(false);
            }
        }
        res
    }
}
```