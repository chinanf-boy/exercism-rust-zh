# 筛

使用 Eratosthenes 的 Sieve 查找从 2 到给定数字的所有素数.

Eratosthenes 的 Sieve 是一种简单,古老的算法,用于查找任何给定限制的所有素数.它通过迭代地将每个素数的倍数标记为复合(即非素数),从 2 的倍数开始.它不使用任何除法或余数运算.创建您的范围,从 2 开始并持续到包括给定限制.

(即 2,限制[,]该算法包括反复重复以下内容:

在列表中取下一个可用的未标记数字(它是素数)

- 标记该数字的所有倍数(它们不是素数)
- 重复,直到处理了范围内的每个数字.

当算法终止时,列表中尚未标记的所有数字都是素数.

维基百科文章有一个有用的图解解释算法:

请注意,这是一个非常具体的算法,并且测试不会检查您是否实现了算法,只是您已经提出了正确的素数列表.<https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes>

一个好的第一个测试是检查你不使用除法或余数运算(div,/,mod 或%,具体取决于语言).锈蚀安装

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科的 Eratosthenes 筛<http://en.wikipedia.org/wiki/Sieve_of_Eratosthenes>
