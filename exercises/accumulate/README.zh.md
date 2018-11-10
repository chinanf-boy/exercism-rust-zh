# 积累

实施`accumulate`操作,给定对集合的每个元素执行的集合和操作,返回包含将该操作应用于输入集合的每个元素的结果的新集合.

鉴于数字的集合:

-   1,2,3,4,5

操作:

-   方数(`x => x * x`)

您的代码应该能够生成正方形集合:

-   1,4,9,16,25

查看测试套件以查看预期的功能签名.

## 限制

请关闭标准库提供的collect / map / fmap / whatchamacallit功能!使用其他基本工具自己解决这个问题.

## 提示

看看Fn可能会有所帮助\*性状:[Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html),[FnMut](https://doc.rust-lang.org/std/ops/trait.Fn.html)和[FnOnce](https://doc.rust-lang.org/std/ops/trait.Fn.html).

有关将闭包传递给函数的帮助可以在["closures as input parameters" section](https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html)的[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/).

如果您的函数签名不适合它们,即使它们没有运行,此练习的测试也会导致编译时错误.您可能希望对某些测试进行评论,并逐个推广您的解决方案.

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

与詹姆斯爱德华格雷二世的对话<https://twitter.com/jeg2>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
