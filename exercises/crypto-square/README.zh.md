# 加密广场

实现用于组成称为方形代码的秘密消息的经典方法.

给定英文文本,输出该文本的编码版本.

首先,输入被规范化:从英文文本中删除空格和标点符号,并且消息是向下的.

然后,规范化字符被分成行.当使用插入的换行符打印时,这些行可被视为形成矩形.

例如,句子

```text
"If man was meant to stay on the ground, god would have given us roots."
```

规范化为:

```text
"ifmanwasmeanttostayonthegroundgodwouldhavegivenusroots"
```

明文应该组织成一个矩形.矩形的大小(`r x c`)应该根据消息的长度来决定`c >= r`和`c - r <= 1`,哪里`c`是列数和`r`是行数.

我们的标准化文本长度为54个字符,用矩形指示矩形`c = 8`和`r = 7`:

```text
"ifmanwas"
"meanttos"
"tayonthe"
"groundgo"
"dwouldha"
"vegivenu"
"sroots  "
```

通过向下读取从左到右的列来获得编码消息.

上面的消息编码为:

```text
"imtgdvsfearwermayoogoanouuiontnnlvtwttddesaohghnsseoau"
```

以填充完美矩形的块输出编码文本`(r X c)`,与`c`大块的`r`长度,以空格分隔.对于那些短语`n`字符短于完美的矩形,每个填充最后一个`n`有一个尾随空间的块.

```text
"imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
```

请注意,如果我们要堆叠这些,我们可以直观地将密文解码回原始消息:

```text
"imtgdvs"
"fearwer"
"mayoogo"
"anouuio"
"ntnnlvt"
"wttddes"
"aohghn "
"sseoau "
```

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

J Dalbey的编程实践问题<http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
