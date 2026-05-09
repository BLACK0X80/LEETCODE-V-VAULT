# Sequentially Ordinal Rank Tracker

**Difficulty:** Hard
**Tags:** Design, Heap (Priority Queue), Data Stream, Ordered Set

---

## Problem

<p>A scenic location is represented by its <code>name</code> and attractiveness <code>score</code>, where <code>name</code> is a <strong>unique</strong> string among all locations and <code>score</code> is an integer. Locations can be ranked from the best to the worst. The <strong>higher</strong> the score, the better the location. If the scores of two locations are equal, then the location with the <strong>lexicographically smaller</strong> name is better.</p>

<p>You are building a system that tracks the ranking of locations with the system initially starting with no locations. It supports:</p>

<ul>
	<li><strong>Adding</strong> scenic locations, <strong>one at a time</strong>.</li>
	<li><strong>Querying</strong> the <code>i<sup>th</sup></code> <strong>best</strong> location of <strong>all locations already added</strong>, where <code>i</code> is the number of times the system has been queried (including the current query).
	<ul>
		<li>For example, when the system is queried for the <code>4<sup>th</sup></code> time, it returns the <code>4<sup>th</sup></code> best location of all locations already added.</li>
	</ul>
	</li>
</ul>

<p>Note that the test data are generated so that <strong>at any time</strong>, the number of queries <strong>does not exceed</strong> the number of locations added to the system.</p>

<p>Implement the <code>SORTracker</code> class:</p>

<ul>
	<li><code>SORTracker()</code> Initializes the tracker system.</li>
	<li><code>void add(string name, int score)</code> Adds a scenic location with <code>name</code> and <code>score</code> to the system.</li>
	<li><code>string get()</code> Queries and returns the <code>i<sup>th</sup></code> best location, where <code>i</code> is the number of times this method has been invoked (including this invocation).</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input</strong>
[&quot;SORTracker&quot;, &quot;add&quot;, &quot;add&quot;, &quot;get&quot;, &quot;add&quot;, &quot;get&quot;, &quot;add&quot;, &quot;get&quot;, &quot;add&quot;, &quot;get&quot;, &quot;add&quot;, &quot;get&quot;, &quot;get&quot;]
[[], [&quot;bradford&quot;, 2], [&quot;branford&quot;, 3], [], [&quot;alps&quot;, 2], [], [&quot;orland&quot;, 2], [], [&quot;orlando&quot;, 3], [], [&quot;alpine&quot;, 2], [], []]
<strong>Output</strong>
[null, null, null, &quot;branford&quot;, null, &quot;alps&quot;, null, &quot;bradford&quot;, null, &quot;bradford&quot;, null, &quot;bradford&quot;, &quot;orland&quot;]

<strong>Explanation</strong>
SORTracker tracker = new SORTracker(); // Initialize the tracker system.
tracker.add(&quot;bradford&quot;, 2); // Add location with name=&quot;bradford&quot; and score=2 to the system.
tracker.add(&quot;branford&quot;, 3); // Add location with name=&quot;branford&quot; and score=3 to the system.
tracker.get();              // The sorted locations, from best to worst, are: branford, bradford.
                            // Note that branford precedes bradford due to its <strong>higher score</strong> (3 &gt; 2).
                            // This is the 1<sup>st</sup> time get() is called, so return the best location: &quot;branford&quot;.
tracker.add(&quot;alps&quot;, 2);     // Add location with name=&quot;alps&quot; and score=2 to the system.
tracker.get();              // Sorted locations: branford, alps, bradford.
                            // Note that alps precedes bradford even though they have the same score (2).
                            // This is because &quot;alps&quot; is <strong>lexicographically smaller</strong> than &quot;bradford&quot;.
                            // Return the 2<sup>nd</sup> best location &quot;alps&quot;, as it is the 2<sup>nd</sup> time get() is called.
tracker.add(&quot;orland&quot;, 2);   // Add location with name=&quot;orland&quot; and score=2 to the system.
tracker.get();              // Sorted locations: branford, alps, bradford, orland.
                            // Return &quot;bradford&quot;, as it is the 3<sup>rd</sup> time get() is called.
tracker.add(&quot;orlando&quot;, 3);  // Add location with name=&quot;orlando&quot; and score=3 to the system.
tracker.get();              // Sorted locations: branford, orlando, alps, bradford, orland.
                            // Return &quot;bradford&quot;.
tracker.add(&quot;alpine&quot;, 2);   // Add location with name=&quot;alpine&quot; and score=2 to the system.
tracker.get();              // Sorted locations: branford, orlando, alpine, alps, bradford, orland.
                            // Return &quot;bradford&quot;.
tracker.get();              // Sorted locations: branford, orlando, alpine, alps, bradford, orland.
                            // Return &quot;orland&quot;.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>name</code> consists of lowercase English letters, and is unique among all locations.</li>
	<li><code>1 &lt;= name.length &lt;= 10</code></li>
	<li><code>1 &lt;= score &lt;= 10<sup>5</sup></code></li>
	<li>At any time, the number of calls to <code>get</code> does not exceed the number of calls to <code>add</code>.</li>
	<li>At most <code>4 * 10<sup>4</sup></code> calls <strong>in total</strong> will be made to <code>add</code> and <code>get</code>.</li>
</ul>


## Hints

1. If the problem were to find the median of a stream of scenery locations while they are being added, can you solve it?
2. We can use a similar approach as an optimization to avoid repeated sorting.
3. Employ two heaps: left heap and right heap. The left heap is a max-heap, and the right heap is a min-heap. The size of the left heap is k + 1 (best locations), where k is the number of times the get method was invoked. The other locations are maintained in the right heap.
4. Every time when add is being called, we add it to the left heap. If the size of the left heap exceeds k + 1, we move the head element to the right heap.
5. When the get method is invoked again (the k + 1 time it is invoked), we can return the head element of the left heap. But before returning it, if the right heap is not empty, we maintain the left heap to have the best k + 2 items by moving the best location from the right heap to the left heap.

## Solution

```rust
use std::collections::BTreeSet;

struct SORTracker {
    black_left: BTreeSet<(i32, String)>,
    black_right: BTreeSet<(i32, String)>,
    black_queries: i32,
}

impl SORTracker {
    fn new() -> Self {
        SORTracker {
            black_left: BTreeSet::new(),
            black_right: BTreeSet::new(),
            black_queries: 0,
        }
    }

    fn add(&mut self, name: String, score: i32) {
        let black_entry = (-score, name);
        if self.black_left.is_empty() || &black_entry <= self.black_left.iter().next_back().unwrap() {
            self.black_left.insert(black_entry);
            if self.black_left.len() > self.black_queries as usize {
                let black_max = self.black_left.iter().next_back().unwrap().clone();
                self.black_left.remove(&black_max);
                self.black_right.insert(black_max);
            }
        } else {
            self.black_right.insert(black_entry);
        }
    }

    fn get(&mut self) -> String {
        self.black_queries += 1;
        if self.black_right.is_empty() {
            return self.black_left.iter().next_back().unwrap().1.clone();
        }
        let black_min = self.black_right.iter().next().unwrap().clone();
        self.black_right.remove(&black_min);
        let res = black_min.1.clone();
        self.black_left.insert(black_min);
        res
    }
}
```