# 谷物

计算棋盘上的小麦粒数,假设每个方格的数量增加一倍.

曾经有一位睿智的仆人拯救了王子的生命.国王承诺支付仆人梦寐以求的一切.知道国王喜欢国际象棋,仆人告诉国王他想吃小麦粒.在棋盘的第一个正方形上的一粒.接下来是两粒.四分之三,依此类推.

棋盘上有64个方格.

编写显示的代码:

-   每个广场上有多少谷物,和
-   谷物总数

## 奖励积分

你是否通过了测试并且代码干净了?如果您愿意,可以尝试以下一些额外的事情:

-   优化速度.
-   优化可读性.

那么请在提交的评论中分享您的想法.这个实验是否使代码更好?更差?你从中学到了什么吗?

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

JavaRanch牛驱动,练习6<http://www.javaranch.com/grains.jsp>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
