# 你的所有基本

将一个数字，表示为一个基数中的数字序列，并转为其他基本

实施通用基本转换。给出一个**a**参数，表示为数字序列，将其转换为基数**b**。

## 笔记

- 尝试自己实现转换。请勿使用其他内容为您执行转换.

## 关于[位置表示法](https://en.wikipedia.org/wiki/Positional_notation)

在位置表示法中，以数字表示**b**可以被理解为权力的线性组合**b**.

数字 42，_基本为 10_，意思是:

`(4 * 10^1) + (2 * 10^0)`

数字 101010，_基本为 2_，意思是:

`(1 * 2^5) + (0 * 2^4) + (1 * 2^3) + (0 * 2^2) + (1 * 2^1) + (0 * 2^0)`

号码 1120，_基本为 3_，意思是:

`(1 * 3^3) + (1 * 3^2) + (2 * 3^1) + (0 * 3^0)`

我想你明白了!

_是。上面这三个数字完全一样。恭喜!_

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
