# Collatz猜想

在Collatz猜想或summarized 3x+1问题可以如下:

任何正整数,如果n的带(2)是指,将得到的N的N / 2.如果n是奇数,(3),1和n的乘积得到3N +1.indefinitely重复的过程.这是在美国的猜想数物开始与你,你将永远eventually达到1.

数了数N的步骤,将需要达到1.

## 实例

起动与n = 12,步骤如下:将

0.  12
1.  6
2.  3
3.  10
4.  5
5.  16
6.  8
7.  4
8.  2
9.  1

9步骤中产生的.系统的输入是n = 12,返回值将是9.

## 防锈装置

Refer to the [exercism帮助页面][help-page]在锈病安装和学习资源.

## 写作:代码

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

## 资源

以数学家Lothar Collatz命名的一个未解决的数学问题<https://en.wikipedia.org/wiki/3x_%2B_1_problem>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
