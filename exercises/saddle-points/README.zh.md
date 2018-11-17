# 鞍点

检测矩阵中的鞍点.

所以说你有一个像这样的矩阵:

```text
    0  1  2
  |---------
0 | 9  8  7
1 | 5  3  2     <--- saddle point at (1,0)
2 | 6  6  7
```

它在(1,0)处有一个鞍点.

它被称为"鞍点",因为它大于或等于其行中的每个元素,并且小于或等于其列中的每个元素.

矩阵可以具有零个或多个鞍点.

您的代码应该能够为任何给定矩阵提供所有鞍点的(可能为空)列表.

矩阵可以具有不同数量的行和列(非正方形).

请注意,您可能会在线找到矩阵鞍点的其他定义,但本练习的测试遵循上述明确的定义.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

J Dalbey 的编程实践问题<http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html>
