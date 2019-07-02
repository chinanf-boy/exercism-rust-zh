# 十进制

实现任意精度的`Decimal`类。

浮点数是计算中非整数实数的最常见表示，它们是由[IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)标准定义。它们非常灵活且通用，但它们确实有一些局限性。众所周知，在浮点运算中，[`0.1 + 0.2 != 0.3`](http://0.30000000000000004.com/)。

解决这一问题的方法是，寻找另一种无损的方法来模拟任意精度的非整数 实数。这可能在内存或处理速度方面，不如浮点数有效；但目标是提供准确的结果。

尽管`Decimal`作为一种自定义类型，我们仍然应该能够将它们视为数字: 而`==`，`<`，`>`，`+`，`-`和`*`操作符都应该按小数进行工作。只是权宜之计，你不需要执行除法，因为任意的精确除法很快就会失控。(如何表示任意精度`1/3`?)

在 Rust 中，将这些操作用于自定义类型的方法是，实现自定义对象的相关 trait。特别是，您至少需要实现.`PartialEq`，`PartialOrd`，`Add`，`Sub`和`Mul`。 严格地说，由于十进制数构成一个总排序，你也应该实现`Eq`和`Ord`，尽管这些 trait 并没有被这些测试所检验.

# 笔记

使用这种.[bigdecimal](https://crates.io/crates/bigdecimal)箱子方法很容易实现这个练习。但不要那样做，你自己来实现。

# 提示

- 不要从头开始执行任意精确的算术，而是考虑在[num_bigint](https://crates.io/crates/num-bigint)箱子之上构建自己的类。
- 你也许能[derive](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)一些需要的 trait。
- `Decimal`假设为符号类型。你不必创建一个单独的无符号类型，尽管你可以这样做，如果你选择这么样，能作为一个实现细节。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

Peter Goodspeed-Niklaus
