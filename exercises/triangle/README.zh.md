# 三角形

确定三角形是等边、等腰还是不等边三角形.

一个*等边的*三角形，三条边都有相同的长度。

一个*等腰的*三角形，至少两边相同的长度。(有时它被指定为两边长度完全相同,但是为了这个练习的目的，我们的说法是，至少两边。)

一*不等边的*三角形的两边各有不同的长度。

## 笔记

为了能形成三角形，所有的边都必须的长度>0，并且任何两边的长度之和必须大于或等于第三边的长度。见[不等三角形](https://en.wikipedia.org/wiki/Triangle_inequality).

## 挖深点

两边长度之和，是*等于*第三边，就叫做*退化*三角形 - 因它有零面积，看起来像一条直线。随意添加你自己的代码/测试来检查退化三角形.

# Rust 中 三角形帮助

- [Result](https://doc.rust-lang.org/std/result/index.html)

实现这一点可以采取多种形式。以下是一些可以帮助你的话题，这取决于你采取的方法.

- [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

或者也许你会想出一种不用那些方法的方法!

## 非整数长度

基础练习测试三角形的边，都是整数的识别。然而，一些三角形不能用纯整数表示。一个简单的例子是一个直角三角形(也是等边三角形，两等边分开 90 度)，其等边都有 1 的长度。它的斜边是 2 的平方根，这是一个无理数: 没有简单的乘法，可以将这个数表示为整数。

重写分析函数来处理整数和浮点情况，老乏味啦，特别是对于所有潜在的整数和浮点类型的情况来说：如给定的 8, 16, 32、64 和 128 位宽度的已签名和未签名的变体，甚至再考虑浮点数，就多出了 10 个根本上相同的代码,!

这有个更好的方法:[generics](https://doc.rust-lang.org/stable/book/2018-edition/ch10-00-generics.html)。 把你的三角形重写为`Triangle<T>`，这样您就可以编写一次代码，并将生成所有这些专门化的工作交给编译器。注意，为了使用数学运算，您需要将泛型类型限制为，支持使用特征的那些运算的类型。

有些加分测试，这些测试，能测试您在浮点数字上的实现。若要启用它们,请使用`generic`特征标记，像这样:

```bash
cargo test --features generic
```

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

Ruby-科恩三角形项目,第 1 和 2 部分<http://rubykoans.com>
