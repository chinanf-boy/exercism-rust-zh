# 筛

使用Eratosthenes的Sieve查找从2到给定数字的所有素数.

Eratosthenes的Sieve是一种简单,古老的算法,用于查找任何给定限制的所有素数.它通过迭代地将每个素数的倍数标记为复合(即非素数),从2的倍数开始.它不使用任何除法或余数运算.创建您的范围,从2开始并持续到包括给定限制.

(即2,限制[,]该算法包括反复重复以下内容:

在列表中取下一个可用的未标记数字(它是素数)

-   标记该数字的所有倍数(它们不是素数)
-   重复,直到处理了范围内的每个数字.

当算法终止时,列表中尚未标记的所有数字都是素数.

维基百科文章有一个有用的图解解释算法:

请注意,这是一个非常具体的算法,并且测试不会检查您是否实现了算法,只是您已经提出了正确的素数列表.<https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes>

一个好的第一个测试是检查你不使用除法或余数运算(div,/,mod或%,具体取决于语言).锈蚀安装

## 参考

运动帮助页面[用于Rust安装和学习资源.][help-page]编写代码

## 执行测试:

除了第一次测试外,所有测试都被忽略了

```bash
$ cargo test
```

在第一个测试通过后,打开位于的测试源文件`tests`目录并删除`#[ignore]`从下一次测试中标记并再次通过测试.每个单独的测试都是一个函数`#[test]`它上面的旗帜.继续,直到通过每个测试.

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

维基百科的Eratosthenes筛<http://en.wikipedia.org/wiki/Sieve_of_Eratosthenes>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
