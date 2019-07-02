# 多米诺骨牌

制作多米诺骨牌.

计算订购给定多米诺骨牌的方法,使它们形成一个正确的多米诺骨牌链(石头的一半上的点与相邻石头的一半上的点相匹配),并且石头的一半上的点与没有邻居(第一块石头和最后一块石头)的点匹配另一方面.

比如石头`[2|1]`,`[2|3]`和`[1|3]`你应该计算一些类似的东西`[1|2] [2|3] [3|1]`或`[3|2] [2|1] [1|3]`或`[1|3] [3|2] [2|1]`等,其中第一个和最后一个数字是相同的.

石头`[1|2]`,`[4|1]`和`[2|3]`结果链无效:`[4|1] [1|2] [2|3]`第一个和最后一个数字不一样.4!= 3

一些测试用例可以在链式解决方案中使用重复的石头,假设使用了多个 Domino 集.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
