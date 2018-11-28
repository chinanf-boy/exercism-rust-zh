# Paasio

报告网络 IO 统计.

你正在写一个[PaaS][]，你需要一种给网络和文件系统使用的帐单.

为报告 IO 统计信息的网络连接和文件，创建包装器。包装器必须报告:

- 读取/写入的字节总数.
- 读取/写入操作的总数.

[paas]: http://en.wikipedia.org/wiki/Platform_as_a_service

## 网络和文件的抽象

网络和文件操作，是实现了[`io::Read`][read]和[`io::Write`][write] `traits` 的。因此,有必要为您的类型实现这些特性.

[read]: https://doc.rust-lang.org/std/io/trait.Read.html
[write]: https://doc.rust-lang.org/std/io/trait.Write.html
[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

布瑞恩松岛<https://github.com/bmatsuo>
