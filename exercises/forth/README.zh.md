# 向前

为 Forth 的一个非常简单的子集实现一个求值程序.

[Forth](https://en.wikipedia.org/wiki/Forth_%28programming_language%29)是一种基于堆栈的编程语言.为 Forth 的一小部分实现一个非常基本的评估器.

您的评估员必须支持以下字词:

- `+`,`-`,`*`,`/`(整数运算)
- `DUP`,`DROP`,`SWAP`,`OVER`(堆栈操作)

您的评估者还必须支持使用惯用语法定义新单词:`: word-name definition ;`.

为简单起见,您需要支持的唯一数据类型是至少 16 位大小的有符号整数.

您应该对语法使用以下规则:数字是一个或多个(ASCII)数字的序列,单词是一个或多个字母,数字,符号或标点符号的序列,而不是数字.(第四个可能使用稍微不同的规则,但这已足够接近了.)

单词不区分大小写.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html
