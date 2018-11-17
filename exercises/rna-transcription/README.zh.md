# RNA 转录

给定 DNA 链,返回其 RNA 补体(每个 RNA 转录).

DNA 和 RNA 链都是核苷酸序列.

DNA 中发现的四个核苷酸是腺嘌呤(**一个**),胞嘧啶(**C**),鸟嘌呤(**G**)和胸腺嘧啶(**Ť**).

RNA 中发现的四个核苷酸是腺嘌呤(**一个**),胞嘧啶(**C**),鸟嘌呤(**G**)和尿嘧啶(**ü**).

给定 DNA 链,其转录的 RNA 链通过用其互补物替换每个核苷酸而形成:

- `G`- >`C`
- `C`- >`G`
- `T`- >`A`
- `A`- >`U`

## 关于 Rust 实现的注释

通过在公共结构中使用私有字段`new`函数返回`Option`要么`Result`(在这里`DNA::new`&`RNA::new`),我们可以保证内部的代表性`DNA`是正确的.因为每个有效的 DNA 字符串都有一个有效的 RNA 字符串,所以我们不需要返回一个`Result`/`Option`从`to_rna`.

这解释了您将在测试中看到的类型签名.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

Hyperphysics<http://hyperphysics.phy-astr.gsu.edu/hbase/Organic/transcription.html>
