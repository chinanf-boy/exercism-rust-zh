# 运行长度编码

实现行程编码和解码.

行程编码(RLE)是一种简单的数据压缩形式,其中运行(连续数据元素)仅由一个数据值和计数代替.

例如,我们可以只用 13 代表原始的 53 个字符.

```text
"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB"  ->  "12WB12W3B24WB"
```

RLE 允许从压缩数据中完美地重建原始数据,这使其成为无损数据压缩.

```text
"AABCCCDEEEE"  ->  "2AB3CD4E"  ->  "AABCCCDEEEE"
```

为简单起见,您可以假设未编码的字符串仅包含字母 A 到 Z(小写或大写)和空格.这样,要编码的数据将永远不会包含任何数字,并且要解码的数据内的数字始终表示后续字符的计数.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Run-length_encoding>
