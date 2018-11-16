# 太空时代

给定年龄(以秒为单位),计算某人的年龄:

-   地球:轨道周期365.25地球日,或31557600秒
-   汞:轨道周期0.2408467地球年
-   金星:轨道周期0.61519726地球年
-   火星:轨道时期1.8808158地球年
-   木星:轨道期11.862615地球年
-   土星:轨道周期29.447498地球年
-   天王星:轨道周期84.016846地球年
-   海王星:轨道周期164.79132地球年

因此,如果你被告知某人的年龄为1,000,000,000秒,你应该可以说它们的年龄为31.69岁.

如果你想知道为什么冥王星没有削减,请去观看[this
youtube video](http://www.youtube.com/watch?v=Z_2gbGXzFbs).

# 话题

在解决此问题时您可能想要阅读的一些Rust主题:

-   特征,从特性和实现自己的特征
-   特征的默认方法实现

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

部分灵感来自Chris Pine在线学习编程教程的第1章.<http://pine.fm/LearnToProgram/?Chapter=01>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
