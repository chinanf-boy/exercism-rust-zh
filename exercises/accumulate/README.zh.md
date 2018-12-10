# Accumulate

实现`accumulate`操作, 给出一个集合，和一个操作行为，该行为会影响到集合中的每个值，并返回一个新的，包含影响结果值的集合

如:给出数字的集合:

- `1,2,3,4,5`

和一个平方操作:

- 平方它(`x => x * x`)

您的代码应该能够生成原集合的平方集合:

- `1,4,9,16,25`

查看测试套件，以查看预期的函数命名.

## 限制

请不要使用，标准库提供的 `collect/map/fmap/whatchamacallit` 函数! 使用其他基本工具自己解决这个问题.

## 提示

看看 Fn traits 可能会有所帮助:[Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html),[FnMut](https://doc.rust-lang.org/std/ops/trait.Fn.html)和[FnOnce](https://doc.rust-lang.org/std/ops/trait.Fn.html).

有关将闭包传递给函数的帮助可以在["闭包作为输入参数" 章节](https://rustwiki.org/zh-CN/rust-by-example/fn/closures/input_parameters.html)里面，更多可看[Rust by Example](https://rustwiki.org/zh-CN/rust-by-example).

> 改为中文网址

如果您的函数命名不适合它们，即使它们没有运行,此练习的测试也会导致编译时错误。您可能希望对某些测试进行注释，并逐个击破。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

与 James Edward Gray II 的对话<https://twitter.com/jeg2>
