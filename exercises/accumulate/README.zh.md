# 积累

实施`accumulate`操作,给定对集合的每个元素执行的集合和操作,返回包含将该操作应用于输入集合的每个元素的结果的新集合.

鉴于数字的集合:

- 1,2,3,4,5

操作:

- 方数(`x => x * x`)

您的代码应该能够生成正方形集合:

- 1,4,9,16,25

查看测试套件以查看预期的功能签名.

## 限制

请关闭标准库提供的 collect / map / fmap / whatchamacallit 功能!使用其他基本工具自己解决这个问题.

## 提示

看看 Fn 可能会有所帮助\*性状:[Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html),[FnMut](https://doc.rust-lang.org/std/ops/trait.Fn.html)和[FnOnce](https://doc.rust-lang.org/std/ops/trait.Fn.html).

有关将闭包传递给函数的帮助可以在["closures as input parameters" section](https://doc.rust-lang.org/stable/rust-by-example/fn/closures/input_parameters.html)的[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/).

如果您的函数签名不适合它们,即使它们没有运行,此练习的测试也会导致编译时错误.您可能希望对某些测试进行评论,并逐个推广您的解决方案.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

与詹姆斯爱德华格雷二世的对话<https://twitter.com/jeg2>
