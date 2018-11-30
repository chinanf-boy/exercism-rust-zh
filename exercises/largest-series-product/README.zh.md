# 最大系列乘积

给定一个数字串,计算长度为 n 的连续子串的最大乘积.

例如,对于输入`'1027839564'` ， 3 位数系列的最大乘积是 270 (`9 * 5 * 6`), 5 位数系列的最大乘积为 7560 (`7 * 8 * 3 * 9 * 5`).

注意这些系列数字字符，在输入中，只要求*相邻位置*，不需要*连续数值(123456..)*.

对于输入`'73167176531330624919225119674426574742355349194934'`一系列 6 位数的最大乘积是 23520.

# Rust 的最大系列乘积

这些迭代器可能是有用的,取决于您的方法.

- [Windows](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
- [Product](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

欧拉工程的问题 8 的一个变种<http://projecteuler.net/problem=8>
