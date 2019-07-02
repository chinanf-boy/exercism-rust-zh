# 鲁恩 trait

# Luhn: 使用自定义 trait

在做这个练习之前，你应该做原始的 luhn 练习和进阶练习，"luhn:使用 From trait".

要获得原始的 Luhn 练习，运行

```shell
exercism download --exercise=luhn --track=rust
```

要获得"Lunn:使用 From trait"的练习，运行

```shell
exercism download --exercise=luhn-from --track=rust
```

在原始的 luhn 练习中，您只验证字符串，但 luhn 算法也可以应用于整数。

在"Lurn:使用 From trait"中，你实现了一个 From trait，还需要你创建一个 luhn 结构。

要是不创建一个结构来执行验证呢，如果要您自己验证了原语本身(即，String，u8 等)呢?

在本练习中，您将创建并实现自定义[trait](https://doc.rust-lang.org/book/ch10-02-traits.html)，来执行验证.

注:它是[实现原语的 trait，不是 Rust 的习惯](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html#a%E4%B8%BA%E7%B1%BB%E5%9E%8B%E5%AE%9E%E7%8E%B0-trait)。 在这个练习中，我们展示了一些你*可以*做的东西，而不是你*应该*做的东西。如果你发现自己在原语上实现了 trait，也许你有一个[Primitive Obsession](http://wiki.c2.com/?PrimitiveObsession)例子。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

基于原始 luhn 练习，延续 Rust 轨迹
