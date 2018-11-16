# 鲁恩性状

# Luhn:使用自定义特征

在做这个练习之前,你应该做原始的LuHn练习和它的继任者,"Luhn:使用来自特征".

要获得原始的LuHn练习,运行

```shell
exercism download --exercise=luhn --track=rust
```

为了获得"Lunn:使用来自特征"的练习,运行

```shell
exercism download --exercise=luhn-from --track=rust
```

在原始的LuHn练习中,您只验证字符串,但LuHN算法也可以应用于整数.

在"Lurn:使用from特性"中,你实现了一个从特性,这也需要你创建一个LuHn结构.

而不是创建一个结构来执行验证,如果您自己验证了原语(即,字符串,U8等)呢?

在本练习中,您将创建并实现自定义[trait](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html)执行验证.

注:它是[not idiomatic Rust to implement traits on on primitives](https://doc.rust-lang.org/book/2018-edition/ch10-02-traits.html#implementing-a-trait-on-a-type). 在这个练习中,我们展示了一些东西*可以*做,而不是你*应该*做.如果你发现自己在图元上实现了特性,也许你有一个例子.[Primitive Obsession](http://wiki.c2.com/?PrimitiveObsession).

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

基于原始Luhn运动的锈迹保持器

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
