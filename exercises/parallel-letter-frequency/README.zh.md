# 并行字母频率

使用并行计算计算文本中的字母频率.

并行性是并行的,也可以按顺序进行.一个常见的例子是计算字母的频率.创建一个函数,返回文本列表中每个字母的总频率,并使用并行性.

# 锈蚀中的平行字母频率

在这里了解更多关于锈的并发性:

-   [Concurrency](https://doc.rust-lang.org/book/2018-edition/ch16-00-concurrency.html)

## 奖金

这个练习还包括一个基准,以一个顺序实现为基线.您可以将您的解决方案与基准进行比较.观察不同大小的输入对每个性能的影响.可以使用并行编程技术来超越基准吗?

在本文中,测试::本彻是不稳定的,只能在*每晚*锈.用货物运行基准:

```
cargo bench
```

如果你使用RuStuff.RS:

```
rustup run nightly cargo bench
```

-   [Benchmark tests](https://doc.rust-lang.org/stable/unstable-book/library-features/test.html)

了解夜间生锈的更多信息:

-   [Nightly Rust](https://doc.rust-lang.org/stable/book/2018-edition/appendix-06-nightly-rust.html)
-   [Rustup: Working with nightly](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust)

## 锈蚀装置

参考[练习帮助页面][help-page]用于锈蚀安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了.在获得第一个测试通过后,打开位于`tests`目录和删除`#[ignore]`从下一个测试标志,并使测试再次通过.每个单独的测试都是一个函数.`#[test]`旗帜在上面.继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下运行所有测试,请使用:

```bash
$ cargo test -- --ignored
```

运行特定的测试,例如`some_test`,您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,则使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关锈蚀试验的更多信息,请参阅[在线测试文档][rust-tests]

请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)如果你还没有,它将帮助你组织你的文件.

## 反馈、问题、拉动请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有锈蚀演习的所在地.如果你有关于锻炼的反馈,或者想帮助实施新的锻炼,就去那里创造一个问题.铁锈队的队员们很乐意帮忙!

如果你想了解更多关于运动的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
