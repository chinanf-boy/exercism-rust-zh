# 海明

计算两条 DNA 链之间的汉明差异.

突变只是在核酸的创建或复制过程中发生的错误,特别是 DNA.因为核酸对细胞功能至关重要,突变倾向于在整个细胞中产生涟漪效应.虽然突变在技术上是错误的,但非常罕见的突变可能为细胞提供有益的属性.事实上,进化的宏观效应归因于许多代有益微观突变的累积结果.

最简单和最常见的核酸突变类型是点突变,它在单个核苷酸处用一个碱基替换另一个碱基.

通过计算从具有共同祖先的不同基因组中获取的两个同源 DNA 链之间的差异数量,我们得到在两条链之间的进化路径上可能发生的最小点突变数量的度量.

这就是所谓的"海明距离".

通过比较两条 DNA 链,并计算其中有多少核苷酸与其他序列中的同等核苷酸不同.

GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT
^ ^ ^ ^ ^ ^^

这两条 DNA 链之间的汉明距离为 7.

# 实施说明

Hamming 距离仅定义为等长的序列,因此尝试计算不同长度的序列之间的 Hamming 距离不应该起作用.这种情况的一般处理(例如,引发异常与返回特殊值)可能在语言之间有所不同.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

罗瑟琳的计算点突变问题<http://rosalind.info/problems/hamm/>
