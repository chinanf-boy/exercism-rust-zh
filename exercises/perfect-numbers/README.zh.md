# 完全数

根据Nicomachus'(60-120CE)的自然数分类方案确定一个数是完美的、丰富的还是有缺陷的.

希腊数学家[Nicomachus](https://en.wikipedia.org/wiki/Nicomachus)设计了一种自然数的分类方案,将每一个识别为唯一属于类别的类别.**很完美**,**大量的**或**缺乏的**基于其[aliquot sum](https://en.wikipedia.org/wiki/Aliquot_sum). 等值和定义为不包括数字本身的数的和的总和.例如,15的等值和是(1+3+5)=9.

-   **珀费克特**等值和数
    -   6是一个完全数,因为(1+2+3)=6.
    -   28是一个完全数,因为(1+2+4+7+14)=28.
-   **丰富的**等号和数
    -   12是一个丰富的数字,因为(1+2+3+4+6)=16.
    -   24是一个丰富的数字,因为(1+2+3+4+6+8+12)=36.
-   **缺乏的**等号和数
    -   8是亏数,因为(1+2+4)=7.
    -   素数不足

实现一种方法来确定给定的数字是否为**很完美**. 根据您的语言轨迹,您还可能需要实现一种方法来确定给定的数字是否为**大量的**或**缺乏的**.

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

## 资源

取自Neal Ford的功能思维第2章.<http://shop.oreilly.com/product/0636920029687.do>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
