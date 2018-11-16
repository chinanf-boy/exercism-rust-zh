# RNA转录

给定DNA链,返回其RNA补体(每个RNA转录).

DNA和RNA链都是核苷酸序列.

DNA中发现的四个核苷酸是腺嘌呤(**一个**),胞嘧啶(**C**),鸟嘌呤(**G**)和胸腺嘧啶(**Ť**).

RNA中发现的四个核苷酸是腺嘌呤(**一个**),胞嘧啶(**C**),鸟嘌呤(**G**)和尿嘧啶(**ü**).

给定DNA链,其转录的RNA链通过用其互补物替换每个核苷酸而形成:

-   `G`- >`C`
-   `C`- >`G`
-   `T`- >`A`
-   `A`- >`U`

## 关于Rust实现的注释

通过在公共结构中使用私有字段`new`函数返回`Option`要么`Result`(在这里`DNA::new`&`RNA::new`),我们可以保证内部的代表性`DNA`是正确的.因为每个有效的DNA字符串都有一个有效的RNA字符串,所以我们不需要返回一个`Result`/`Option`从`to_rna`.

这解释了您将在测试中看到的类型签名.

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

Hyperphysics<http://hyperphysics.phy-astr.gsu.edu/hbase/Organic/transcription.html>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
