# 猪的拉丁文

实现一个从英语翻译成猪拉丁语的程序.

猪拉丁语是一种拼凑的儿童语言，目的是使人困惑。它遵循一些简单的规则(下面)，但是当它说得很快时，对于非儿童(以及非母语者)来说真的很难理解.

- **规则 1**如果一个单词以元音开头，在单词的末尾加上一个"ay"音。请注意，在单词开头的"xr"和"yt"会产生元音(例如 "xray" -> "xrayay", "yttria" -> "yttriaay"）。
- **规则 2**如果一个单词以辅音开头，把它移到单词的末尾，然后在单词的末尾加上一个"ay"音。辅音可以由多个辅音组成，例如辅音群(例如"chair" -> "airchay").
- **规则 3**如果一个单词以辅音开头，后面跟着"qu"，把它移动到单词的结尾，然后在单词的结尾加上"ay"音(例如，"square" -> "aresquay").
- **规则 4**如果一个单词在辅音群后面包含"y"，或者作为两个字母元音的单词的第二个字母(例如，"rhythm" -> "ythmrhay", "my" -> "ymay")。

边缘案例还有一些规则，也有区域性的变化.

见<http://en.wikipedia.org/wiki/Pig_latin>更多细节.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

猪拉丁文运动，在第一次超声波教学中的应用<https://github.com/ultrasaurus/test-first-teaching/blob/master/learn_ruby/pig_latin/>
