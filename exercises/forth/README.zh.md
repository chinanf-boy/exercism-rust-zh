# 向前

为Forth的一个非常简单的子集实现一个求值程序.

[Forth](https://en.wikipedia.org/wiki/Forth_%28programming_language%29)是一种基于堆栈的编程语言.为Forth的一小部分实现一个非常基本的评估器.

您的评估员必须支持以下字词:

-   `+`,`-`,`*`,`/`(整数运算)
-   `DUP`,`DROP`,`SWAP`,`OVER`(堆栈操作)

您的评估者还必须支持使用惯用语法定义新单词:`: word-name definition ;`.

为简单起见,您需要支持的唯一数据类型是至少16位大小的有符号整数.

您应该对语法使用以下规则:数字是一个或多个(ASCII)数字的序列,单词是一个或多个字母,数字,符号或标点符号的序列,而不是数字.(第四个可能使用稍微不同的规则,但这已足够接近了.)

单词不区分大小写.

## 锈蚀安装

参考[运动帮助页面][help-page]用于Rust安装和学习资源.

## 编写代码

执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有测试都被忽略了在第一个测试通过后,打开位于的测试源文件`tests`目录并删除`#[ignore]`从下一次测试中标记并再次通过测试.每个单独的测试都是一个函数`#[test]`它上面的旗帜.继续,直到通过每个测试.

如果您希望在不编辑测试源文件的情况下运行所有​​测试,请使用:

```bash
$ cargo test -- --ignored
```

例如,运行特定测试`some_test`, 您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,请使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

一定要阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节如果你还没有,它将帮助你组织你的文件.

## 反馈,问题,请求

该[exercism/rust](https://github.com/exercism/rust)GitHub上的存储库是所有Rust练习的主页.如果您有关于练习的反馈,或者想要帮助实施新的练习,请前往那里并创建一个问题.铁轨团队成员很乐意为您提供帮助!

如果你想了解更多关于运动的知识,请看看[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
