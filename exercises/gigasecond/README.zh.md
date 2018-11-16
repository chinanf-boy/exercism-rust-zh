# Gigasecond

计算某个开始时刻，计算10^9秒后的时刻.

一个 `千兆秒-gigasecond` 是10^9(1,000,000,000)秒.

如果您不确定`DateTime<Utc>`可以执行哪些操作，看看[chrono crate](https://docs.rs/chrono/0.4.0/chrono/) - 它在`Cargo.toml`，被列为本练习的一个依赖项.

## Rust装置

参考[练习帮助页面][help-page]用于Rust安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了。在获得第一个测试通过后,打开位于`tests`目录和，在下一个测试标志删除`#[ignore]`,并使测试再次通过。每个单独的测试都是一个函数.`#[test]`标志在上面。继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下，运行所有测试,请使用:

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

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

如果你还没有，请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节，它将帮助你组织你的文件.

## 反馈、提问、拉取请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有Rust演习的所在地。如果你有关于锻炼的反馈,或者想帮助建立新的练习,就去那里创造一个问题.rust track 团队的队员们很乐意帮忙!

如果你想了解更多关于exercism.io的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

这是一个介绍使用者，使用Exercism进行练习.<http://en.wikipedia.org/wiki/%22Hello,_world!%22_program>

## 提交不完全解法

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
