# 素数筛

使用 Eratosthenes 的 Sieve 查找从 2 到给定数字的所有素数.

这是一种简单且历史悠久的筛法，用来找出一定范围内所有的素数。

所使用的原理是从 2 开始，将每个素数的各个倍数，标记成合数。一个素数的各个倍数，是一个差为此素数本身的等差数列。此为这个筛法和试除法不同的关键之处，后者是以素数来测试每个待测数能否被整除。

埃拉托斯特尼筛法是列出所有小素数最有效的方法之一，其名字来自于古希腊数学家埃拉托斯特尼，并且被描述在另一位古希腊数学家尼科马库斯所著的《算术入门》中。

维基百科文章有一个有用的图解解释算法:

请注意,这是一个非常具体的算法,并且测试不会检查您是否实现了算法,只要您已经提出了正确的素数列表.<https://zh.wikipedia.org/wiki/埃拉托斯特尼筛法>

一个好的第一个测试是，检查你不使用除法或余数运算(div, /, mod or % 具体语言所具有的)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

维基百科的 素数 筛<https://zh.wikipedia.org/wiki/埃拉托斯特尼筛法>
