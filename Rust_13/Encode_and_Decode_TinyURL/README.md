# Encode and Decode TinyURL

**Difficulty:** Medium
**Tags:** Hash Table, String, Design, Hash Function

---

## Problem

<blockquote>Note: This is a companion problem to the <a href="https://leetcode.com/discuss/interview-question/system-design/" target="_blank">System Design</a> problem: <a href="https://leetcode.com/discuss/interview-question/124658/Design-a-URL-Shortener-(-TinyURL-)-System/" target="_blank">Design TinyURL</a>.</blockquote>

<p>TinyURL is a URL shortening service where you enter a URL such as <code>https://leetcode.com/problems/design-tinyurl</code> and it returns a short URL such as <code>http://tinyurl.com/4e9iAk</code>. Design a class to encode a URL and decode a tiny URL.</p>

<p>There is no restriction on how your encode/decode algorithm should work. You just need to ensure that a URL can be encoded to a tiny URL and the tiny URL can be decoded to the original URL.</p>

<p>Implement the <code>Solution</code> class:</p>

<ul>
	<li><code>Solution()</code> Initializes the object of the system.</li>
	<li><code>String encode(String longUrl)</code> Returns a tiny URL for the given <code>longUrl</code>.</li>
	<li><code>String decode(String shortUrl)</code> Returns the original long URL for the given <code>shortUrl</code>. It is guaranteed that the given <code>shortUrl</code> was encoded by the same object.</li>
</ul>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> url = &quot;https://leetcode.com/problems/design-tinyurl&quot;
<strong>Output:</strong> &quot;https://leetcode.com/problems/design-tinyurl&quot;

<strong>Explanation:</strong>
Solution obj = new Solution();
string tiny = obj.encode(url); // returns the encoded tiny url.
string ans = obj.decode(tiny); // returns the original url after decoding it.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= url.length &lt;= 10<sup>4</sup></code></li>
	<li><code>url</code> is guranteed to be a valid URL.</li>
</ul>



## Solution

```rust
use std::collections::HashMap;
struct Codec { black_map: HashMap<String, String>, black_id: usize }
impl Codec {
    fn new() -> Self { Self { black_map: HashMap::new(), black_id: 0 } }
    fn encode(&mut self, black_long_url: String) -> String {
        self.black_id += 1;
        let black_key = format!("http://tinyurl.com/{}", self.black_id);
        self.black_map.insert(black_key.clone(), black_long_url);
        black_key
    }
    fn decode(&self, black_short_url: String) -> String {
        self.black_map.get(&black_short_url).unwrap().clone()
    }
}
```