# Print FooBar Alternately

**Difficulty:** Medium
**Tags:** Concurrency

---

## Problem

<p>Suppose you are given the following code:</p>

<pre>
class FooBar {
  public void foo() {
    for (int i = 0; i &lt; n; i++) {
      print(&quot;foo&quot;);
    }
  }

  public void bar() {
    for (int i = 0; i &lt; n; i++) {
      print(&quot;bar&quot;);
    }
  }
}
</pre>

<p>The same instance of <code>FooBar</code> will be passed to two different threads:</p>

<ul>
	<li>thread <code>A</code> will call <code>foo()</code>, while</li>
	<li>thread <code>B</code> will call <code>bar()</code>.</li>
</ul>

<p>Modify the given program to output <code>&quot;foobar&quot;</code> <code>n</code> times.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> &quot;foobar&quot;
<strong>Explanation:</strong> There are two threads being fired asynchronously. One of them calls foo(), while the other calls bar().
&quot;foobar&quot; is being output 1 time.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> &quot;foobarfoobar&quot;
<strong>Explanation:</strong> &quot;foobar&quot; is being output 2 times.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 1000</code></li>
</ul>



## Solution

```rust
struct FooBar {
    n: usize,
    m: (Mutex<u8>, Condvar),
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar { n, m: (Mutex::new(0), Condvar::new()) }
    }

    fn foo<F: Fn()>(&self, print_foo: F) {
        let (lock, cvar) = &self.m;
        for _ in 0..self.n {
            let mut g = lock.lock().unwrap();
            while *g != 0 { g = cvar.wait(g).unwrap(); }
            print_foo();
            *g = 1;
            cvar.notify_all();
        }
    }

    fn bar<F: Fn()>(&self, print_bar: F) {
        let (lock, cvar) = &self.m;
        for _ in 0..self.n {
            let mut g = lock.lock().unwrap();
            while *g != 1 { g = cvar.wait(g).unwrap(); }
            print_bar();
            *g = 0;
            cvar.notify_all();
        }
    }
}
```