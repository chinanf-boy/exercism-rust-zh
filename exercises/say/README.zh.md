# 说

鉴于0到999,999,999,999之间的数字,用英语拼出这个数字.

## 步骤1

处理0到99的基本情况.

如果程序的输入是`22`,那么输出应该是`'twenty-two'`.

如果给出超出祝福范围的数字,您的程序应该大声抱怨.

这个程序的一些好的测试用例是:

-   0
-   14
-   50
-   98
-   \-1
-   100

### 延期

如果您使用的是Mac,那就是Mac OS X的外壳`say`程序大声说出来.如果您使用的是Linux或Windows,则可以使用该命令使用eSpeakNG`espeak`.

## 第2步

实施将数量分成几千块.

所以`1234567890`应该产生如1,234,567和890的列表,而更简单`1000`应该只产生1和0.

该程序还必须报告超出范围的任何值.

## 第3步

现在处理在这些块之间插入适当的缩放词.

所以`1234567890`应该屈服`'1 billion 234 million 567 thousand 890'`

该程序还必须报告超出范围的任何值.停在"兆"处可以.

## 第4步

把它们放在一起除了简单的英语之外什么都没有.

`12345`应该给`twelve thousand three hundred forty-five`.

该程序还必须报告超出范围的任何值.

### 扩展

使用*和*(正确地)用英语拼出数字时:

-   14变成"十四".
-   100变成"一百".
-   120变成"一百二十".
-   1002变成"一千零二".
-   1323年成为"一千三百二十三".

## 特定的铁锈练习笔记

与本练习的其他语言版本相比,Rust版本略有改变.我们使用Rust的强类型系统来限制输入,而不是要求您返回超出范围的错误.使函数处理所有有效输入要容易得多,而不是要求模块的用户处理错误.

有一个-1版本的测试用例,但它被注释掉了.如果您的函数正确实现,则不应编译-1测试用例.

在测试用例中尚未实现将"和"添加到数字文本中.

### 延期

添加转换为u64的最大值的功能:9,223,372,036,854,775,807.

有关输出的提示,请查看最后一个测试用例.

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

JavaRanch CattleDrive的变体,练习4a<http://www.javaranch.com/say.jsp>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
