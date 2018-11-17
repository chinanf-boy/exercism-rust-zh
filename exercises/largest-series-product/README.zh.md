# 最大系列产品

给定一个数字串,计算长度为 n 的连续子串的最大乘积.

例如,对于输入`'1027839564'`一系列 3 位数的最大产品是 270(9).*五*6),一系列 5 位数字的最大乘积为 7560(7).*八*三*九*5).

注意这些系列只需要占用.*相邻位置*在输入中,数字不需要*数值连续的*.

对于输入`'73167176531330624919225119674426574742355349194934'`一系列 6 位数的最大产品是 23520.

# 锈蚀最大系列产品

这些迭代器可能是有用的,取决于您的方法.

- [Windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
- [Product](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

工程欧拉问题 8 的一个变化<http://projecteuler.net/problem=8>
