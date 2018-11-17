# 三角形

确定三角形是等边、等腰还是不等边三角形.

安*等边的*三角形三条边都有相同的长度.

安*等腰的*三角形至少有两个边相同的长度.(有时它被指定为两边长度完全相同,但是为了这个练习的目的,我们至少要说两边.)

一*不等边的*三角形的两边各有不同的长度.

## 注释

一个形状要成为三角形,所有的边都必须的长度>0,并且任何两边的长度之和必须大于或等于第三边的长度.见[Triangle Inequality](https://en.wikipedia.org/wiki/Triangle_inequality).

## 挖深

两边长度之和的情况*等于*第三人称为*退化的*三角形-它有零面积,看起来像一条直线.随意添加你自己的代码/测试来检查退化三角形.

# 鲁斯特三角

- [Result](https://doc.rust-lang.org/std/result/index.html)

实现这一点可以采取多种形式.以下是一些可以帮助你的话题,这取决于你采取的方法.

- [Enums](https://doc.rust-lang.org/book/2018-edition/ch06-00-enums.html)
- [Traits](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html)
- [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

或者也许你会想出一种不用那些方法的方法!

## 非整数长度

基础练习测试三角形的边是所有整数的识别.然而,一些三角形不能用纯整数表示.一个简单的例子是一个直角三角形(等边三角形,其等边分开 90 度),其等边都有 1 的长度.它的斜边是 2 的平方根,这是一个无理数:没有简单的乘法可以将这个数表示为整数.

重写分析函数来处理整数和浮点情况将是乏味的,特别是对于所有潜在的整数和浮点类型来说都是乏味的:给定的比特宽度 8, 16, 32、64 和 128 的已签名和未签名的变体,这将是 10 个重新实现的.根本上相同的代码,甚至在考虑浮动!

还有更好的方法:[generics](https://doc.rust-lang.org/stable/book/2018-edition/ch10-00-generics.html). 把你的三角形重写为`Triangle<T>`您可以编写一次代码,并将生成所有这些专门化的工作交给编译器.注意,为了使用数学运算,您需要将泛型类型限制为支持使用特征的那些运算的类型.

您可以运行一些奖金测试,这些测试测试您在浮点数字上的实现.若要启用它们,请使用`generic`特征标记,像这样:

cargo test --features generic

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

红宝石-科恩三角项目,第 1 和 2 部分<http://rubykoans.com>
