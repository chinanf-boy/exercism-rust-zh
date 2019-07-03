# React

实现基本的 React 系统。

React 式编程是一种编程范例，它着重于如何根据彼此来计算值，以允许对一个值的更改，自动传播到其他值，如在电子表格中。

实现一个基本的 React 系统，其中，有带可设置值的单元("输入"单元)，还有能根据其他单元计算值的单元("计算"单元)。实现更新，以便当输入值改变时，（关联的）值会传播，直到一个新的稳定的系统状态。

此外，计算单元应该允许，注册更改通知的回调。当单元值，从之前的稳定状态到一个新的稳定状态时，调用一个单元格的回调。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
