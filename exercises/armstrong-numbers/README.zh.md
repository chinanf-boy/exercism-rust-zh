# 阿姆斯特朗数字

一个[Armstrong number](https://en.wikipedia.org/wiki/Narcissistic_number)是一个数字,它是自己的数字之和,每个数字都是数字的幂.

例如:

-   9是阿姆斯特朗号,因为`9 = 9^1 = 9`
-   10是*不*一个阿姆斯特朗号,因为`10 != 1^2 + 0^2 = 1`
-   153是阿姆斯特朗号,因为:`153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153`
-   154是*不*一个阿姆斯特朗号,因为:`154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190`

写一些代码来确定一个数字是否是阿姆斯特朗数.

## 锈蚀安装

参考[运动帮助页面][help-page]用于Rust安装和学习资源.

## 编写代码

执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有测试都被忽略了在第一个测试通过后,打开位于的测试源文件`tests`目录并删除`#[ignore]`从下一次测试中标记并再次通过测试.每个单独的测试都是一个函数`#[test]`它上面的旗帜.继续,直到通过每个测试.

如果您希望在不编辑测试源文件的情况下运行所有​​测试,请使用:

```bash
$ cargo test -- --ignored
```

例如,运行特定测试`some_test`, 您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,请使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

一定要阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节如果你还没有,它将帮助你组织你的文件.

## 反馈,问题,请求

该[exercism/rust](https://github.com/exercism/rust)GitHub上的存储库是所有Rust练习的主页.如果您有关于练习的反馈,或者想要帮助实施新的练习,请前往那里并创建一个问题.铁轨团队成员很乐意为您提供帮助!

如果你想了解更多关于运动的知识,请看看[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Narcissistic_number>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
