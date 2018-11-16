# ISBN检验器

这个[ISBN-10 verification process](https://en.wikipedia.org/wiki/International_Standard_Book_Number)用于验证图书识别号.这些通常包含破折号,看起来像:`3-598-21508-8`

## 国际标准书号

ISBN-10格式是9位数字(0到9)加上一个校验字符(一个数字或一个X).在校验字符为x的情况下,这表示值"10".这些可以与有或没有连字符通信,并且可以通过以下公式检查它们的有效性:

```
(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0
```

如果结果是0,那么它是一个有效的ISBN-10,否则它是无效的.

## 例子

让我们用ISBN-10`3-598-21508-8`. 我们把它插入到公式中,得到:

```
(3 * 10 + 5 * 9 + 9 * 8 + 8 * 7 + 2 * 6 + 1 * 5 + 5 * 4 + 0 * 3 + 8 * 2 + 8 * 1) mod 11 == 0
```

由于结果是0,这证明我们的ISBN是有效的.

## 任务

给定一个字符串,程序应该检查所提供的字符串是否是有效的ISBN-10.实现这一点需要在计算ISBN的校验位数之前考虑字符串的预处理/解析.

该程序应该能够验证ISBN-10既有,也不分离破折号.

## 告诫

在某些语言中,从字符串转换为数字可能是棘手的.现在,甚至更棘手的是,ISBN-10的校验位可能是"X"(表示"10").例如`3-598-21507-X`是一个有效的ISBN-10.

## 奖金任务

-   从输入ISBN-10生成有效的ISBN-13(并且可能用派生验证器再次验证它).

-   生成有效的ISBN,甚至可能从给定的起始ISBN中生成.

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

将一个字符串转换成一个数字和一些基本的处理,利用一个可靠的真实世界的例子.<https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-10_check_digit_calculation>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
