# Forth

为 Forth 的一个非常简单的子集，实现一个执行程序.

[Forth](https://en.wikipedia.org/wiki/Forth_%28programming_language%29)是一种基于栈的编程语言。为 Forth 的一小部分子集，实现一个非常基本的执行器。

您的执行器必须支持以下关键字:

- `+`，`-`，`*`，`/`(整数运算)
- `DUP`，`DROP`，`SWAP`，`OVER`(栈操作)

您的执行器还必须支持使用惯用语法，定义新单词:`: word-name definition ;`.

为简单起见，您需要支持的唯一数据类型是，至少 16 位大小的有符号整数。

您应该对语法使用以下规则：一个 number 是一个或多个(ASCII)数字的序列，单词是一个或多个字母，数字，符号或标点符号的序列，而不是数字。(Forth 可能使用稍微不同的规则，但这已足够接近。)

单词不区分大小写。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
