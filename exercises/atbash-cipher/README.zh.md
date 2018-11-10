# Atbash Cipher

创建atbash密码的实现,这是在中东创建的古老加密系统.

Atbash密码是一种简单的替换密码,它依赖于转置字母表中的所有字母,使得生成的字母表向后.第一个字母替换为最后一个字母,第二个字母替换为倒数第二个字母,依此类推.

拉丁字母的Atbash密码如下:

```text
Plain:  abcdefghijklmnopqrstuvwxyz
Cipher: zyxwvutsrqponmlkjihgfedcba
```

它是一个非常弱的密码,因为它只有一个可能的密钥,它是一个简单的单字母替换密码.但是,这可能不是密码时代的问题.

密文以固定长度的组写出,传统的组大小为5个字母,并且不包括标点符号.这是为了使基于单词边界的东西更难猜测.

## 例子

-   编码`test`给`gvhg`
-   解码`gvhg`给`test`
-   解码`gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt`给`thequickbrownfoxjumpsoverthelazydog`

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

维基百科<http://en.wikipedia.org/wiki/Atbash>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
