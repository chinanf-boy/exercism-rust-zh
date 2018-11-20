# 素数因子

计算给定自然数的素因子.

质数(素数): 只能被自身和 1 整除.

注意,1 不是素数.

## 例子

60 的主要因素是什么?

- 我们的第一个除数是 2 。 2 被除以 60,剩下 30。
- 2 被除以 30,剩下 15。
  - 2 不能被除以 15。让我们转到下一个除数,3.
- 3 被除以 15 分,5 剩下。
  - 3 不能被除以 5。下一个可能的因素是 4.
  - 4 不能被除以 5。下一个可能的因素是 5.
- 5 确实能被除以 5.
- 我们只剩下 1 ,所以现在,我们完成了。

我们在该计算中成功, 而 **60**除数代表 ∶ `2, 2, 3 和 5` 为主要因子的列表.

你可以自己检查一下:

- 2 × 2 × 3 × 5
- = 4 × 15
- = 60
- 成功!

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

Uncle Bob 的主要因素算法<http://butunclebob.com/ArticleS.UncleBob.ThePrimeFactorsKata>
