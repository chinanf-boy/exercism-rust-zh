# 旋转密码

创建旋转密码的实现,有时也称为Caesar密码.

Caesar密码是一个简单的移位密码,它依赖于使用整数键在字母表中转置所有字母`0`和`26`.使用密钥`0`要么`26`由于模运算,总是会产生相同的输出.将字母移动为与键值一样多的值.

旋转密码的一般表示法是`ROT + <key>`.最常用的旋转密码是`ROT13`.

一个`ROT13`拉丁字母表如下:

```text
Plain:  abcdefghijklmnopqrstuvwxyz
Cipher: nopqrstuvwxyzabcdefghijklm
```

它比Atbash密码更强大,因为它有27个可能的密钥和25个可用的密钥.

密文以与输入相同的格式写出,包括空格和标点符号.

## 例子

-   ROT5`omg`给`trl`
-   ROT0`c`给`c`
-   ROT26`Cool`给`Cool`
-   ROT13`The quick brown fox jumps over the lazy dog.`给`Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`
-   ROT13`Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`给`The quick brown fox jumps over the lazy dog.`

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

维基百科<https://en.wikipedia.org/wiki/Caesar_cipher>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
