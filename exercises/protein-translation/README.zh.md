# 蛋白质翻译

将RNA序列翻译成蛋白质.

RNA可以分解为三个称为密码子的核苷酸序列,然后翻译成如下多肽:

RNA:`"AUGUUUUCU"`=>翻译成

密码:`"AUG", "UUU", "UCU"`=>其成为具有以下序列的多肽=>

蛋白:`"Methionine", "Phenylalanine", "Serine"`

有64个密码子,而这些密码子又相当于20个氨基酸;然而,在本练习中,所有密码子序列和所得氨基酸都不重要.如果它适用于一个密码子,该程序应该适用于所有这些密码子.但是,您可以随意扩展测试套件中的列表以包含它们.

还有三个终止密码子(也称为'STOP'密码子);如果遇到任何这些密码子(通过核糖体),则所有翻译结束并终止蛋白质.

之后的所有后续密码子都会被忽略,如下所示:

RNA:`"AUGUUUUCUUAAAUG"`=>

密码:`"AUG", "UUU", "UCU", "UAA", "AUG"`=>

蛋白:`"Methionine", "Phenylalanine", "Serine"`

注意终止密码子`"UAA"`终止翻译,最终的蛋氨酸不会翻译成蛋白质序列.

以下是运动所需的密码子和产生的氨基酸.

| 密码子 | 蛋白 |
| :-- | :-- |
| AUG | 蛋氨酸 |
| UUU,UUC | 苯丙氨酸 |
| UUA,UUG | 亮氨酸 |
| UCU,UCC,UCA,UCG | 丝氨酸 |
| UAU,UAC | 酪氨酸 |
| UGU,UGC | 半胱氨酸 |
| UGG | 色氨酸 |
| UAA,UAG,UGA | 停 |

学习更多关于[protein translation on Wikipedia](http://en.wikipedia.org/wiki/Translation_(biology))

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

泰勒龙

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
