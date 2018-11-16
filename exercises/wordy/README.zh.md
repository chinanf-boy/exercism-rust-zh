# 罗唆

解析并评估简单的数学单词问题,将答案作为整数返回.

## 迭代1  - 加法

一起添加两个数字.

> 什么是5加13?

评估为18.

处理大数和负数.

## 迭代2  - 减法,乘法和除法

现在,执行其他三个操作.

> 什么是7减5?

2

> 什么是6乘以4?

24

> 什么是25除以5?

五

## 迭代3  - 多个操作

按顺序处理一组操作.

由于这些是口头语言问题,从左到右评估表达,*忽略了典型的操作顺序.*

> 什么是5加13加6?

24

> 什么是3加2加3?

15(即不是9)

## 奖金 - 指数

如果你愿意,处理指数.

> 什么是2提升到5的力量?

32

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

灵感来自Extreme Startup游戏中的一个生成的问题.<https://github.com/rchatley/extreme_startup>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
