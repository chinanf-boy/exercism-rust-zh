# 汉明（Hamming）

给 2 个长度为 n 的 DNA 序列，求汉明距离差异为多少。

通过比较两条 DNA 链,并计算其中有多少核苷酸与其他序列中的同等核苷酸不同.

```
GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT
^ ^ ^ ^ ^ ^^
```

这两条 DNA 链之间的 汉明距离为 7.

# 实现说明

汉明长度仅定义为等长的序列，因此尝试计算不同长度的序列之间的汉明 距离不应该起作用。这种情况的一般处理(例如,引发异常与返回特殊值)可能在语言之间有所不同.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

罗瑟琳的计算点突变问题<http://rosalind.info/problems/hamm/>
