# 雨滴-Raindrops

把一个数字转换成一个字符串,它的内容取决于，数字的因素.

- 如果数字有 3 作为一个因素,输出"Pling".
- 如果数字有 5 作为一个因素,输出"Plang'".
- 如果数字有 7 作为一个因素,输出"Plong".
- 如果数字没有 3, 5,或 7 作为一个因素, 直接给数字。

## 实例

- 28 的因素是 1, 2, 4, **7**,14, 28.
  - 雨滴说,这将是一个简单的"Plong".
- 30 的因素是 1, 2,**3**,**5**,6, 10, 15,30.
  - 雨滴说,这将是一个"PlingPlang".
- 34 有四个因素:1, 2, 17,34.
  - 雨滴说,这将是"34".

## Rust 装置

参考[我们的帮助页面][help-page]的 Rust 安装和学习资源.

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

要了解有关 Rust 测试的更多信息,请参阅[在线测试文档][rust-tests]

如果你还没有，请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节，它将帮助你组织你的文件.

## 反馈、提问、拉取请求

这个[exercism/rust](https://github.com/exercism/rust)在 Github 上的储存库是所有 Rust 演习的所在地。如果你有关于锻炼的反馈,或者想帮助建立新的练习,就去那里创造一个问题.rust track 团队的队员们很乐意帮忙!

如果你想了解更多关于 exercism.io 的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

这是一个介绍使用者，使用 Exercism 进行练习.<http://en.wikipedia.org/wiki/%22Hello,_world!%22_program>

## 提交不完全解法

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
