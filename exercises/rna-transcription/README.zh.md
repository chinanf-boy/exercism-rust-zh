# RNA 转录

给定 DNA 链,返回其 RNA 补体(每个 RNA 转录).

DNA 和 RNA 链都是核苷酸序列.

DNA 中发现的四个核苷酸是腺嘌呤(**A**),胞嘧啶(**C**),鸟嘌呤(**G**)和胸腺嘧啶(**T**).

RNA 中发现的四个核苷酸是腺嘌呤(**A**),胞嘧啶(**C**),鸟嘌呤(**G**)和尿嘧啶(**T**).

给定 DNA 链,其转录的 RNA 链，通过用其互补物替换每个核苷酸而形成:

- `G`- >`C`
- `C`- >`G`
- `T`- >`A`
- `A`- >`U`

## 关于 Rust 实现的注释

通过在公共结构中使用私有字段，`new`函数会返回`Option`要么`Result`(在这里`DNA::new`&`RNA::new`)，我们可以保证`DNA`内部的表达是正确的。因为每个有效的 DNA 字符串，都有一个有效的 RNA 字符串,所以我们不需要从`to_rna`返回一个`Result`/`Option`.

这解释了您将在测试中看到的类型签名。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

Hyperphysics<http://hyperphysics.phy-astr.gsu.edu/hbase/Organic/transcription.html>
