# 宏

宏是 Rust 程序员工具箱中的一个强大工具，要想对它有个简单认知，可以看看[宏例子](https://doc.rust-lang.org/reference/macros-by-example.html)。让我们写一个!

## 问题陈述

你可以用`vec![]`内联宏，生产一个任意长度的`Vec`。然而,Rust 并没有生产[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)的内联宏`hashmap!()`。

例如，您的库的用户可能会编写`hashmap!('a' => 3, 'b' => 11, 'z' => 32)`。这应该扩展到以下代码:

```rust,no_run
{
   let mut hm = HashMap::new();
   hm.insert('a', 3);
   hm.insert('b', 11);
   hm.insert('z', 32);
   hm
}
```

注意[`maplit`箱子](https://crates.io/crates/maplit)时提供了一个很好地解决这个问题的宏。但请执行您自己的解决方案，而不是使用这个箱子，请在查看箱子源代码之前，自己尝试下。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

Peter Goodspeed-Niklaus
