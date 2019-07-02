# 完全数

根据 Nicomachus'(60-120CE)的自然数分类方案，确定一个数是- **Perfect(完全)**, **Abundant(过剩数)**, **Deficient(亏数)**

希腊数学家[Nicomachus](https://en.wikipedia.org/wiki/Nicomachus)设计了一种自然数的分类方案,将每一个数识别归类为 **perfect**, **abundant**, or **deficient** ，方案基于[aliquot sum（等值和）](https://en.wikipedia.org/wiki/Aliquot_sum)。 等值和定义为不包括数字本身的约数(可除出整数)的总和。例如,15 的等值和是(1+3+5)=9.

- **Perfect(完全)**: aliquot sum = number
  - 6 ，因为 (1 + 2 + 3) = 6
  - 28 ，因为 (1 + 2 + 4 + 7 + 14) = 28
- **Abundant(过剩数)**: aliquot sum > number
  - 12 , 因为 (1 + 2 + 3 + 4 + 6) = 16
  - 24 , 因为 (1 + 2 + 3 + 4 + 6 + 8 + 12) = 36
- **Deficient(亏数)**: aliquot sum < number
  - 8 ， 因为 (1 + 2 + 4) = 7
  - 素数 都是 deficient

实现一种方法来确定给定的数字是否为**Perfect**。 根据您的语言轨迹,您还可能需要实现一种方法来确定给定的数字是否为**Abundant**或**Deficient**.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

取自 Neal Ford 的函数思维第 2 章.<http://shop.oreilly.com/product/0636920029687.do>
