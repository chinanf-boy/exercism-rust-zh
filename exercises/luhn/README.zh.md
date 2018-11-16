# 卢恩

给定一个数,判定它是否是LuHn公式有效的.

这个[Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)是一个简单的校验和公式,用于验证各种身份号码,如信用卡号码和加拿大社会保险号码.

任务是检查给定字符串是否有效.

## 验证一个数

长度为1或更小的字符串无效.在输入中允许使用空格,但在检查前应清除空格.所有其他非数字字符都是不允许的.

## 例子1:有效信用卡号码

```text
4539 1488 0343 6467
```

LuHN算法的第一步是从右边开始每一个第二个数字的加倍.我们将加倍

```text
4_3_ 1_8_ 0_4_ 6_6_
```

如果加倍的数字导致大于9的数字,则从产品减去9.我们加倍的结果:

```text
8569 2478 0383 3437
```

然后把所有数字加起来:

```text
8+5+6+9+2+4+7+8+0+3+8+3+3+4+3+7 = 80
```

如果总和可被10整除,则数字是有效的.这个号码是有效的!

## 例子2:信用卡号码无效

```text
8273 1232 7352 0569
```

把第二个数字加倍,从右边开始

```text
7253 2262 5312 0539
```

合计数字

```text
7+2+5+3+2+2+6+2+5+3+1+2+0+5+3+9 = 57
```

57不能被10整除,所以这个数字是无效的.

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

维基百科上的LuHn算法<http://en.wikipedia.org/wiki/Luhn_algorithm>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
