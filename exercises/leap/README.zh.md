# Leap

给出年份,报告是不是闰年.

这里的棘手问题是公历中，闰年计算:

> 我想也很清楚了，就不译了

```text
on every year that is evenly divisible by 4
  except every year that is evenly divisible by 100
    unless the year is also evenly divisible by 400
```

例如,1997不是闰年,而是1996是，1900不是闰年,而2000是。

如果在您的语言标准库中，提供了执行此实现的方法, 请假装它不存在,并自己实现它.

## 笔记

虽然我们的采用一些非常简单的规则,但还有更多的东西要学!

为了一个令人愉快的,为什么有闰年现象的解释,请观看[这个YouTube视频][video].

> 译: 我给个[李老师的视频]()

[video]: http://www.youtube.com/watch?v=xX96xng7sAE

## Rust装置

参考[练习帮助页面][help-page]的Rust安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了。在获得第一个测试通过后,打开位于`tests`目录和，在下一个测试标志删除`#[ignore]`,并使测试再次通过。每个单独的测试都是一个函数.`#[test]`标志在上面。继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下，运行所有测试,请使用:

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

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

如果你还没有，请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节，它将帮助你组织你的文件.

## 反馈、提问、拉取请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有Rust演习的所在地。如果你有关于锻炼的反馈,或者想帮助建立新的练习,就去那里创造一个问题.rust track 团队的队员们很乐意帮忙!

如果你想了解更多关于exercism.io的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 来源

这是一个介绍使用者，使用Exercism进行练习.<http://en.wikipedia.org/wiki/%22Hello,_world!%22_program>

## 提交不完全解法

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
