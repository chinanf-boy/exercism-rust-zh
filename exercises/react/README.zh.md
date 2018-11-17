# 反应

实现基本的反应系统.

反应式编程是一种编程范例,它着重于如何根据彼此来计算值,以允许对一个值的更改自动传播到其他值,如在电子表格中.

实现一个基本的反应系统,其中单元具有可设置的值("输入"单元),单元具有按其他单元计算的值("计算"单元).实现更新,以便当输入值改变时,值传播到一个新的稳定的系统状态.

此外,计算单元应该允许注册更改通知回调.当一个新的稳定状态中的单元格值从先前的稳定状态改变时,调用一个单元格的回调.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html
