# 核苷酸密码子

编写一个函数，返回一个特定密码子的氨基酸名称，可能使用速记，编码.

在 DNA 序列中，3 个核苷酸称为密码子，编码氨基酸。通常密码子编码相同的氨基酸。国际纯和应用化学联盟开发了一个简写系统，用于命名密码子组，其编码为相同氨基酸。

简单地说，他们把四个字母 A、C、G 和 T 扩展成一堆代表不同可能性的字母。例如 R 代表 A 和 G，
所以 TAR 可表示为 TAA 和 TAG (把 "TAR" 当成正则式形式的 "TA[AG]")。

编写一些代码，给出一个密码子，可以使用速记，然后返回密码子所编码的氨基酸的名称。您将得到一个非速记密码/名称对的列表，可作为您计算的基础。

见:[维基百科](https://en.wikipedia.org/wiki/DNA_codon_table).

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
