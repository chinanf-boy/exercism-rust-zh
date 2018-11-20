# 水仙花数

一个[Armstrong number](https://en.wikipedia.org/wiki/Narcissistic_number)是一个数字,它是自身每个单数字与数字量的幂，之和。

例如:

- 9 是水仙花数,因为`9 = 9^1 = 9`
- 10 *不是*一个水仙花数,因为`10 != 1^2 + 0^2 = 1`
- 153 是水仙花数,因为:`153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153`
- 154 *不是*一个水仙花数,因为:`154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190`

写一些代码来确定一个数字是否水仙花数.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Narcissistic_number>
