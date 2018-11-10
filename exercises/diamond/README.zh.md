# 钻石

钻石kata将输入字母作为输入,并以菱形输出.给定一个字母,它会打印一个以'A'开头的钻石,并在最宽处提供所提供的字母.

## 要求

-   第一行包含一个'A'.
-   最后一行包含一个'A'.
-   除第一行和最后一行之外的所有行都有两个完全相同的字母.
-   所有行都具有与前导空格一样多的尾随空格.(这可能是0).
-   钻石是水平对称的.
-   钻石是垂直对称的.
-   钻石具有方形(宽度等于高度).
-   字母形成菱形.
-   上半部分的字母按升序排列.
-   下半部分的字母按降序排列.
-   四个角(包含空格)是三角形.

## 例子

在以下示例中,空格表示为`·`字符.

字母'A'的钻石:

```text
A
```

字母'C'的钻石:

```text
··A··
·B·B·
C···C
·B·B·
··A··
```

字母'E'的钻石:

```text
····A····
···B·B···
··C···C··
·D·····D·
E·······E
·D·····D·
··C···C··
···B·B···
····A····
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

Seb Rose<http://claysnow.co.uk/recycling-tests-in-tdd/>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
