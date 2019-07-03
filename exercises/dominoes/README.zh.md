# 多米诺骨牌

制作一个多米诺骨牌.

计算给出的多米诺骨牌的排序方法，使它们形成一个正确的多米诺骨牌链(石头的一半上的数值，与相邻石头的一半上的数值相匹配)，并且(第一块石头和最后一块石头)石头的一半上的数值没有邻居，且它们的数值匹配.

比如，给出石头`[2|1]`，`[2|3]`和`[1|3]`你应该计算一些类似`[1|2] [2|3] [3|1]`或`[3|2] [2|1] [1|3]`或`[1|3] [3|2] [2|1]`的东西等，其中第一个和最后一个数字是相同的。

对于以下石头`[1|2]`，`[4|1]`和`[2|3]`，它的结果骨牌链无效：`[4|1] [1|2] [2|3]`第一个和最后一个数字不一样。4!= 3

一些测试用例可以在一个骨牌链解决方案中，使用重复的石头，假设使用了多个骨牌集合。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
