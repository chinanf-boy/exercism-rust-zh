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

-   [Result](https://doc.rust-lang.org/std/result/index.html)

实现这一点可以采取多种形式.以下是一些可以帮助你的话题,这取决于你采取的方法.

-   [Enums](https://doc.rust-lang.org/book/2018-edition/ch06-00-enums.html)
-   [Traits](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html)
-   [BTreeSet](https://doc.rust-lang.org/std/collections/btree_set/struct.BTreeSet.html)

或者也许你会想出一种不用那些方法的方法!

## 非整数长度

基础练习测试三角形的边是所有整数的识别.然而,一些三角形不能用纯整数表示.一个简单的例子是一个直角三角形(等边三角形,其等边分开90度),其等边都有1的长度.它的斜边是2的平方根,这是一个无理数:没有简单的乘法可以将这个数表示为整数.

重写分析函数来处理整数和浮点情况将是乏味的,特别是对于所有潜在的整数和浮点类型来说都是乏味的:给定的比特宽度8, 16, 32、64和128的已签名和未签名的变体,这将是10个重新实现的.根本上相同的代码,甚至在考虑浮动!

还有更好的方法:[generics](https://doc.rust-lang.org/stable/book/2018-edition/ch10-00-generics.html). 把你的三角形重写为`Triangle<T>`您可以编写一次代码,并将生成所有这些专门化的工作交给编译器.注意,为了使用数学运算,您需要将泛型类型限制为支持使用特征的那些运算的类型.

您可以运行一些奖金测试,这些测试测试您在浮点数字上的实现.若要启用它们,请使用`generic`特征标记,像这样:

```bash
cargo test --features generic
```

## 锈蚀装置

参考[练习帮助页面][help-page]用于锈蚀安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了.在获得第一个测试通过后,打开位于`tests`目录和删除`#[ignore]`从下一个测试标志,并使测试再次通过.每个单独的测试都是一个函数.`#[test]`旗帜在上面.继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下运行所有测试,请使用:

```bash
$ cargo test -- --ignored
```

运行特定的测试,例如`some_test`,您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,则使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关锈蚀试验的更多信息,请参阅[在线测试文档][rust-tests]

请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)如果你还没有,它将帮助你组织你的文件.

## 反馈、问题、拉动请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有锈蚀演习的所在地.如果你有关于锻炼的反馈,或者想帮助实施新的锻炼,就去那里创造一个问题.铁锈队的队员们很乐意帮忙!

如果你想了解更多关于运动的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

红宝石-科恩三角项目,第1和2部分<http://rubykoans.com>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
