# ETL

我们要去做`Transform`提取转换负载的步骤.

### ETL

提取转换负载(ETL)是一种很有意思的说法,"我们在这个系统中有一些遗留的遗留数据,现在我们需要在这个闪亮的新系统中,所以我们将迁移它."

(通常情况下,接下来是,"我们只需要运行一次."之后,通常会有很多额头拍打,抱怨我们可能多么愚蠢.

### 目标

我们将从遗留系统中提取一些拼字游戏分数.

旧的系统存储每一个字母的列表:

-   1点:"A","E","I","O","U","L","N","R","S","T",
-   2点:"D","G",
-   3点:"B"、"C"、"M"、"P",
-   4分:"F"、"H"、"V"、"W"、"Y",
-   5点:"K",
-   8点:"J","X",
-   10点:"Q","Z",

闪亮的新拼写系统存储每个字母的分数,这使得计算一个单词的分数更快、更容易.它也把字母放在小写字母中,而不考虑输入字母的情况:

-   "A"值1分.
-   "B"值3分.
-   "C"值3分.
-   "D"值2分.
-   等.

你的任务,你应该选择接受它,是将遗留数据格式转换成闪亮的新格式.

### 笔记

关于得分的最后一点是,Scrabble用各种语言在世界各地播放,每种语言都有自己独特的得分表.例如,在毛利语版本的游戏中,"E"得分为2分,而在夏威夷语版本中为4分.

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

## 来源

JunpStab实验室团队<http://jumpstartlab.com>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
