# 回文产品

在给定范围内检测回文产品.

回文数是指当数字倒数时保持不变的数.例如,`121`是回文数,但`112`不是.

给定一系列数字,找出最大和最小回文,这是该范围内数字的乘积.

您的解决方案应该返回最大和最小回文,以及范围内的每个因素.如果最大或最小回文在范围内有多于一对的因素,则返回所有的对.

## 例1

给定范围`[1, 9]`(两者兼而有之)

并给出了所有可能的产品在这个范围内的列表:`[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16, 18, 15, 21, 24, 27, 20, 28, 32, 36, 25, 30, 35, 40, 45, 42, 48, 54, 49, 56, 63, 64, 72, 81]`

回文产品都是单数数字(在这种情况下):`[1, 2, 3, 4, 5, 6, 7, 8, 9]`

最小回文产品是`1`. 其因素是`(1, 1)`. 最大回文产品是`9`. 其因素是`(1, 9)`和`(3, 3)`.

## 例2

给定范围`[10, 99]`(两者兼而有之)

最小回文产品是`121`. 其因素是`(11, 11)`. 最大回文产品是`9009`. 其因素是`(91, 99)`.

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

项目欧拉问题4<http://projecteuler.net/problem=4>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
