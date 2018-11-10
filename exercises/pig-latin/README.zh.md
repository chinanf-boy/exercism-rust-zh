# 猪拉丁文

实现一个从英语翻译成猪拉丁语的程序.

猪拉丁语是一种拼凑的儿童语言,目的是使人困惑.它遵循一些简单的规则(下面),但是当它说得很快时,对于非儿童(以及非母语者)来说真的很难理解.

-   **规则1**如果一个单词以元音开头,在单词的末尾加上一个"YAY"音.请注意,在单词开头的"XR"和"YT"会产生元音(例如"X射线"->"XRAYAY"、"YTrIA-">"YTTIAAY").
-   **规则2**如果一个单词以辅音开头,把它移到单词的末尾,然后在单词的末尾加上一个"YAY"音.辅音可以由多个辅音组成,例如辅音群(例如"椅子">"Acjayy").
-   **规则3**如果一个单词以辅音开头,后面跟着"qu",移动到单词的结尾,然后在单词的结尾加上"ay"音(例如,"."->"aresquay").
-   **规则4**如果一个单词在辅音群后面包含"y",或者作为两个字母单词的第二个字母,它会发出元音发音(例如,"节奏"->"节奏"->"my"->"ymay").

边缘案例还有一些规则,也有区域性的变化.

见<http://en.wikipedia.org/wiki/Pig_latin>更多细节.

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

## 来源

猪拉丁文运动在第一次超声波教学中的应用<https://github.com/ultrasaurus/test-first-teaching/blob/master/learn_ruby/pig_latin/>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
