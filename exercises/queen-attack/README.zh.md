# 女王攻击

给定棋盘上的两个皇后的位置，指示在各自位置的它们，是否能互相攻击。

在象棋游戏中，女王可以攻击同一行、列或对角线上的棋子。

棋盘可以用 8 乘 8 的数组来表示。

所以如果你告诉白皇后在(2， 3)和黑皇后在(5， 6)，那么设定像这样:

```text
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ W _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ B _
_ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _
```

你也可以回答女王是否可以互相攻击。而在这种情况下，答案是肯定的，他们可以，因为这两个部分共用一个对角线。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

J Dalbey 的程序设计实践问题<http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html>
