# 多米诺骨牌

制作多米诺骨牌.

计算订购给定多米诺骨牌的方法,使它们形成一个正确的多米诺骨牌链(石头的一半上的点与相邻石头的一半上的点相匹配),并且石头的一半上的点与没有邻居(第一块石头和最后一块石头)的点匹配另一方面.

比如石头`[2|1]`,`[2|3]`和`[1|3]`你应该计算一些类似的东西`[1|2] [2|3] [3|1]`或`[3|2] [2|1] [1|3]`或`[1|3] [3|2] [2|1]`等,其中第一个和最后一个数字是相同的.

石头`[1|2]`,`[4|1]`和`[2|3]`结果链无效:`[4|1] [1|2] [2|3]`第一个和最后一个数字不一样.4!= 3

一些测试用例可以在链式解决方案中使用重复的石头,假设使用了多个Domino集.

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

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
