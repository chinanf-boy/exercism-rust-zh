# 子列表

给定两个列表确定第一个列表是否包含在第二个列表中；第二个列表是否包含在第一个列表中；两个列表是否包含在彼此，或者这些都不是真的。

具体来说，列表 A 是列表 B 的子列表，在于是否 B 的前面删除 0 个或更多元素和 B 的后面删除 0 个或更多元素，则得到完全等于 A 的列表。

例子:

- A = `[1,2,3]`，B = `[1,2,3,4,5]`，A 是 B 的子列表
- A = `[3,4,5]`，B = `[1,2,3,4,5]`，A 是 B 的子列表
- A = `[3,4]`，B = `[1,2,3,4,5]`，A 是 B 的子列表
- A = `[1,2,3]`，B = `[1,2,3]`，A 等于 B。
- A = `[1,2,3,4,5]`，B = `[2,3,4]`，A 是 B 的父列表
- A = `[1,2,4]`，B = `[1,2,3,4,5]`，A 不是 B 的父列表，子列表，或等于 B 的列表

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
