# 迪菲赫尔曼

Diffie-Hellman密钥交换.

Alice和Bob使用Diffie-Hellman密钥交换来共享秘密.它们以素数开头,选择私钥,生成和共享公钥,然后生成共享密钥.

## 第0步

测试程序提供素数p和g.

## 步骤1

Alice选择一个大于1且小于p的私钥.鲍勃做同样的事情来选择私钥b.

## 第2步

Alice计算公钥A.

```
A = g**a mod p
```

使用相同的p和g,Bob类似地从他的私钥b计算公钥B.

## 第3步

Alice和Bob交换公钥.Alice计算密钥s.

```
s = B**a mod p
```

鲍勃计算

```
s = A**b mod p
```

计算产生相同的结果!爱丽丝和鲍勃现在分享秘密.

本练习的一种可能解决方案是实现您自己的模幂运算功能.要了解更多信息,请参阅[following page](https://en.wikipedia.org/wiki/Modular_exponentiation).

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

维基百科,来自www.cryptopp.com/wiki的1024位密钥.<http://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
