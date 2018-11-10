# grep的

在文件中搜索与正则表达式模式匹配的行.返回每个匹配行的行号和内容.

Unix[`grep`](http://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html)命令可用于搜索与用户提供的搜索查询匹配的一个或多个文件中的行(称为*图案*).

该`grep`命令有三个参数:

1.  用于匹配文件中的行的模式.
2.  零个或多个标志以自定义匹配行为.
3.  一个或多个要搜索匹配行的文件.

你的任务是实现`grep`function,应该读取指定文件的内容,找到与指定模式匹配的行,然后将这些行输出为单个字符串.请注意,行应按其找到的顺序输出,第一个文件中的第一个匹配行首先输出.

例如,假设有一个名为"input.txt"的文件,其中包含以下内容:

```text
hello
world
hello again
```

如果我们打电话`grep "hello" input.txt`,返回的字符串应该是:

```text
hello
hello again
```

### 旗

如前所述,`grep`命令还应该支持以下标志:

-   `-n`打印每个匹配行的行号.
-   `-l`仅打印包含至少一个匹配行的文件的名称.
-   `-i`使用不区分大小写的比较匹配行.
-   `-v`反转程序 - 收集所有与模式不匹配的行.
-   `-x`仅匹配整行,而不是匹配包含匹配的行.

如果我们跑`grep -n "hello" input.txt`,`-n`flag将要求匹配的行以其行号作为前缀:

```text
1:hello
3:hello again
```

如果我们跑`grep -i "HELLO" input.txt`,我们将做一个不区分大小写的匹配,输出将是:

```text
hello
hello again
```

该`grep`命令应该一次支持多个标志.

例如,跑步`grep -l -v "hello" file1.txt file2.txt`应该打印不包含字符串"hello"的文件的名称.

### 错误处理

本练习介绍了该用法`failure`crate,它为您提供了管理自定义错误类型的方法.要了解有关箱子的更多信息,请参阅[failure documentation](https://boats.gitlab.io/failure/intro.html)

### 补充阅读

虽然本练习要求您只实现最基本的功能`grep`,实际上有一个完全重新实施的项目`grep`在Rust  -[ripgrep](https://github.com/BurntSushi/ripgrep).

如果您喜欢在Rust中重写基本util程序的概念,请务必检查以下项目:

-   [fd](https://github.com/sharkdp/fd)- 克隆`find`
-   [exa](https://github.com/ogham/exa)- 克隆`ls`
-   [bat](https://github.com/sharkdp/bat)- 克隆`cat`
-   [coreutils](https://github.com/uutils/coreutils)- 重写GNU coreutils

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

与Nate Foster的对话.<http://www.cs.cornell.edu/Courses/cs3110/2014sp/hw/0/ps0.pdf>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
