# Print in Order

**Difficulty:** Easy
**Tags:** Concurrency

---

## Problem

<p>Suppose we have a class:</p>

<pre>
public class Foo {
  public void first() { print(&quot;first&quot;); }
  public void second() { print(&quot;second&quot;); }
  public void third() { print(&quot;third&quot;); }
}
</pre>

<p>The same instance of <code>Foo</code> will be passed to three different threads. Thread A will call <code>first()</code>, thread B will call <code>second()</code>, and thread C will call <code>third()</code>. Design a mechanism and modify the program to ensure that <code>second()</code> is executed after <code>first()</code>, and <code>third()</code> is executed after <code>second()</code>.</p>

<p><strong>Note:</strong></p>

<p>We do not know how the threads will be scheduled in the operating system, even though the numbers in the input seem to imply the ordering. The input format you see is mainly to ensure our tests&#39; comprehensiveness.</p>

<p>&nbsp;</p>
<p><strong class="example">Example 1:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> &quot;firstsecondthird&quot;
<strong>Explanation:</strong> There are three threads being fired asynchronously. The input [1,2,3] means thread A calls first(), thread B calls second(), and thread C calls third(). &quot;firstsecondthird&quot; is the correct output.
</pre>

<p><strong class="example">Example 2:</strong></p>

<pre>
<strong>Input:</strong> nums = [1,3,2]
<strong>Output:</strong> &quot;firstsecondthird&quot;
<strong>Explanation:</strong> The input [1,3,2] means thread A calls first(), thread B calls third(), and thread C calls second(). &quot;firstsecondthird&quot; is the correct output.
</pre>

<p>&nbsp;</p>
<p><strong>Constraints:</strong></p>

<ul>
	<li><code>nums</code> is a permutation of <code>[1, 2, 3]</code>.</li>
</ul>



## Solution

```rust
struct Foo {
    m: Arc<(Mutex<u8>, Condvar)>,
}

impl Foo {
    fn new() -> Self {
        Foo {
            m: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }

    fn first(&self, print_first: impl FnOnce()) {
        let (lock, cvar) = &*self.m;
        print_first();
        *lock.lock().unwrap() = 1;
        cvar.notify_all();
    }

    fn second(&self, print_second: impl FnOnce()) {
        let (lock, cvar) = &*self.m;
        let mut g = lock.lock().unwrap();
        while *g < 1 { g = cvar.wait(g).unwrap(); }
        print_second();
        *g = 2;
        cvar.notify_all();
    }

    fn third(&self, print_third: impl FnOnce()) {
        let (lock, cvar) = &*self.m;
        let mut g = lock.lock().unwrap();
        while *g < 2 { g = cvar.wait(g).unwrap(); }
        print_third();
    }
}
```