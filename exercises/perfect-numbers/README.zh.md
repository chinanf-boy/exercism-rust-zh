# 完全数

根据 Nicomachus'(60-120CE)的自然数分类方案确定一个数是完美的、丰富的还是有缺陷的.

希腊数学家[Nicomachus](https://en.wikipedia.org/wiki/Nicomachus)设计了一种自然数的分类方案,将每一个识别为唯一属于类别的类别.**很完美**,**大量的**或**缺乏的**基于其[aliquot sum](https://en.wikipedia.org/wiki/Aliquot_sum). 等值和定义为不包括数字本身的数的和的总和.例如,15 的等值和是(1+3+5)=9.

- **珀费克特**等值和数
  - 6 是一个完全数,因为(1+2+3)=6.
  - 28 是一个完全数,因为(1+2+4+7+14)=28.
- **丰富的**等号和数
  - 12 是一个丰富的数字,因为(1+2+3+4+6)=16.
  - 24 是一个丰富的数字,因为(1+2+3+4+6+8+12)=36.
- **缺乏的**等号和数
  - 8 是亏数,因为(1+2+4)=7.
  - 素数不足

实现一种方法来确定给定的数字是否为**很完美**. 根据您的语言轨迹,您还可能需要实现一种方法来确定给定的数字是否为**大量的**或**缺乏的**.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

取自 Neal Ford 的功能思维第 2 章.<http://shop.oreilly.com/product/0636920029687.do>
