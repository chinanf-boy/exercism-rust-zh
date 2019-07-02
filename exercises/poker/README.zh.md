# 扑克

从扑克手牌列表中，挑选最好的手牌。

见[wikipedia](https://en.wikipedia.org/wiki/List_of_poker_hands)中的扑克手牌概述.

## 提示

- 排名扑克手牌可以被认为是一个排序问题.
- Rust 提供[sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)方法，用在`Vec<T> where T: Ord`。
- [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html)是一个[总顺序](https://en.wikipedia.org/wiki/Total_order)形式，`a < b`,`a == b`或`a > b`其中一个一定是真的.
- 扑克手牌不符合一个总顺序：两份手牌可以不相等，但有相同的排序。例子:`3S 4S 5D 6H JH"`,`"3H 4H 5C 6C JD"`。
- Rust 提供[`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)处理不具有完全顺序的可排序事物的情况。然而，它没有提供标准的`sort`方法，用于`Vec<T> where T: PartialOrd`。在这种情况下，对向量进行排序的标准方式是`your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`这取决于你的需要.\`
- 您可以考虑实现了`PartialOrd`，表示扑克手牌的类型。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

受来自 Udacity 的培训课程的启发.<https://www.udacity.com/course/viewer#!/c-cs212/>
