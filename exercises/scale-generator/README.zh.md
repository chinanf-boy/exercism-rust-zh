# 比例生成器

给定音调,或者开始音符以及一组间隔,从音调开始并遵循指定的间隔模式生成音阶.

西方音乐中的音阶基于彩色(12音符)音阶.该比例可表示为以下一组音高:

A,A#,B,C,C#,D,D#,E,F,F#,G,G#

给定的尖锐音符(用#表示)也可以表示为它上面音符的平面(用b表示),所以半音音阶也可以这样写:

A,Bb,B,C,Db,D,Eb,E,F,Gb,G,Ab

主要和次要的比例和模式是这个十二音高集合的子集.它们有七个音高,称为全音阶音阶.这些音阶中的音符集合使用锐利或平面,根据补品.这是一个列表,其中包括:

没有锐利或平底鞋:C大调未成年人

使用Sharps:G,D,A,E,B,F#major e,b,f#,c#,g#,d#minor

使用平板:F,Bb,Eb,Ab,Db,Gb主要d,g,c,f,bb,eb minor

全音阶音阶以及源自半音音阶的所有其他音阶都是间隔建立的.间隔是两个音高之间的间距.

最简单的间隔是在两个相邻音符之间,称为"半步"或"小秒"(有时写为小写"m").具有中间音符的两个音符之间的间隔称为"整步"或"大秒"(写为大写"M").仅使用相邻音符之间的这两个间隔来构建全音阶音阶.

非全音阶音阶可以包含其他音程.写入"A"的"增强的第一"间隔具有两个中间音符(例如,从A到C或从D到E).间隔也越来越小,但他们不会参与这个练习.

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
