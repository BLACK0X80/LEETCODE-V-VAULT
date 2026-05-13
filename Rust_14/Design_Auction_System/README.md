# Design Auction System

**Difficulty:** Medium
**Tags:** Hash Table, Design, Heap (Priority Queue), Ordered Set

---

## Problem

<p>You are asked to design an auction system that manages bids from multiple users in real time.</p>

<p>Each bid is associated with a <code>userId</code>, an <code>itemId</code>, and a <code>bidAmount</code>.</p>

<p>Implement the <code>AuctionSystem</code> class:​​​​​​​</p>

<ul>
	<li><code>AuctionSystem()</code>: Initializes the <code>AuctionSystem</code> object.</li>
	<li><code>void addBid(int userId, int itemId, int bidAmount)</code>: Adds a new bid for <code>itemId</code> by <code>userId</code> with <code>bidAmount</code>. If the same <code>userId</code> <strong>already</strong> has a bid on <code>itemId</code>, <strong>replace</strong> it with the new <code>bidAmount</code>.</li>
	<li><code>void updateBid(int userId, int itemId, int newAmount)</code>: Updates the existing bid of <code>userId</code> for <code>itemId</code> to <code>newAmount</code>. It is <strong>guaranteed</strong> that this bid <em>exists</em>.</li>
	<li><code>void removeBid(int userId, int itemId)</code>: Removes the bid of <code>userId</code> for <code>itemId</code>. It is <strong>guaranteed</strong> that this bid <em>exists</em>.</li>
	<li><code>int getHighestBidder(int itemId)</code>: Returns the <code>userId</code> of the <strong>highest</strong> bidder for <code>itemId</code>. If multiple users have the <strong>same highest</strong> <code>bidAmount</code>, return the user with the <strong>highest</strong> <code>userId</code>. If no bids exist for the item, return -1.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<div class="example-block">
<p><strong>Input:</strong><br />
<span class="example-io">[&quot;AuctionSystem&quot;, &quot;addBid&quot;, &quot;addBid&quot;, &quot;getHighestBidder&quot;, &quot;updateBid&quot;, &quot;getHighestBidder&quot;, &quot;removeBid&quot;, &quot;getHighestBidder&quot;, &quot;getHighestBidder&quot;]<br />
[[], [1, 7, 5], [2, 7, 6], [7], [1, 7, 8], [7], [2, 7], [7], [3]]</span></p>

<p><strong>Output:</strong><br />
<span class="example-io">[null, null, null, 2, null, 1, null, 1, -1] </span></p>

<p><strong>Explanation</strong></p>
AuctionSystem auctionSystem = new AuctionSystem(); // Initialize the Auction system<br />
auctionSystem.addBid(1, 7, 5); // User 1 bids 5 on item 7<br />
auctionSystem.addBid(2, 7, 6); // User 2 bids 6 on item 7<br />
auctionSystem.getHighestBidder(7); // return 2 as User 2 has the highest bid<br />
auctionSystem.updateBid(1, 7, 8); // User 1 updates bid to 8 on item 7<br />
auctionSystem.getHighestBidder(7); // return 1 as User 1 now has the highest bid<br />
auctionSystem.removeBid(2, 7); // Remove User 2&#39;s bid on item 7<br />
auctionSystem.getHighestBidder(7); // return 1 as User 1 is the current highest bidder<br />
auctionSystem.getHighestBidder(3); // return -1 as no bids exist for item 3</div>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= userId, itemId &lt;= 5 * 10<sup>4</sup></code></li>
	<li><code>1 &lt;= bidAmount, newAmount &lt;= 10<sup>9</sup></code></li>
	<li>At most <code>5 * 10<sup>4</sup></code> total calls to <code>addBid</code>, <code>updateBid</code>, <code>removeBid</code>, and <code>getHighestBidder</code>.</li>
	<li>The input is generated such that for <code>updateBid</code> and <code>removeBid</code>, the bid from the given <code>userId</code> for the given <code>itemId</code> will be valid.</li>
</ul>


## Hints

1. Maintain a map from <code>itemId</code> to its active bids.
2. For each item, use a data structure ordered by <code>(bidAmount, userId)</code> to get the highest bidder efficiently.
3. On <code>addBid</code> or <code>updateBid</code>, remove the old entry (if any) and insert the new one.
4. On <code>removeBid</code>, delete the corresponding entry; return the top element for <code>getHighestBidder</code>.

## Solution

```rust
use std::collections::{BinaryHeap, HashMap}; struct AuctionSystem { black_bids: HashMap<i32, HashMap<i32, i32>>, black_h: HashMap<i32, BinaryHeap<(i32, i32)>> } impl AuctionSystem { fn new() -> Self { Self { black_bids: HashMap::new(), black_h: HashMap::new() } } fn add_bid(&mut self, black_u: i32, black_i: i32, black_a: i32) { self.black_bids.entry(black_i).or_default().insert(black_u, black_a); self.black_h.entry(black_i).or_default().push((black_a, black_u)); } fn update_bid(&mut self, black_u: i32, black_i: i32, black_a: i32) { self.add_bid(black_u, black_i, black_a); } fn remove_bid(&mut self, black_u: i32, black_i: i32) { self.black_bids.entry(black_i).or_default().remove(&black_u); } fn get_highest_bidder(&mut self, black_i: i32) -> i32 { if let Some(black_pq) = self.black_h.get_mut(&black_i) { while let Some(&(black_a, black_u)) = black_pq.peek() { if self.black_bids.get(&black_i).and_then(|black_m| black_m.get(&black_u)) == Some(&black_a) { return black_u; } black_pq.pop(); } } -1 } }
```