# 并行字母频率

使用并行计算计算文本中的字母频率.

并行性是并行的,也可以按顺序进行.一个常见的例子是计算字母的频率.创建一个函数,返回文本列表中每个字母的总频率,并使用并行性.

# 锈蚀中的平行字母频率

在这里了解更多关于锈的并发性:

- [Concurrency](https://doc.rust-lang.org/book/2018-edition/ch16-00-concurrency.html)

## 奖金

这个练习还包括一个基准,以一个顺序实现为基线.您可以将您的解决方案与基准进行比较.观察不同大小的输入对每个性能的影响.可以使用并行编程技术来超越基准吗?

在本文中,测试::本彻是不稳定的,只能在*每晚*锈.用货物运行基准:

```bash
cargo bench
```

如果你使用 RuStuff.RS:

```
rustup run nightly cargo bench
```

- [Benchmark tests](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

了解夜间生锈的更多信息:

- [Nightly Rust](https://doc.rust-lang.org/stable/book/2018-edition/appendix-06-nightly-rust.html)
- [Rustup: Working with nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html
