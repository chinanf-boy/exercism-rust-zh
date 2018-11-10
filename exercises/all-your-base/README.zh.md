# 所有你的基地

将一个数字(表示为一个基数中的数字序列)转换为任何其他基数.

实施一般基础转换.鉴于基数**一个**,表示为数字序列,将其转换为基数**b**.

## 注意

-   尝试自己实现转换.请勿使用其他内容为您执行转换.

## 关于[Positional Notation](https://en.wikipedia.org/wiki/Positional_notation)

在位置表示法中,以数字表示**b**可以被理解为权力的线性组合**b**.

数字42,*在基地10*,意思是:

(4*10 ^ 1)+(2*10 ^ 0)

数字101010,*在基地2*,意思是:

(1*2 ^ 5)+(0*2 ^ 4)+(1*2 ^ 3)+(0*2 ^ 2)+(1*2 ^ 1)+(0*2 ^ 0)

号码1120,*在基地3*,意思是:

(1*3 ^ 3)+(1*3 ^ 2)+(2*3 ^ 1)+(0*3 ^ 0)

我想你明白了!

*是.上面这三个数字完全一样.恭喜!*

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

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
