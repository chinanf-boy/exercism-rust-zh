# 海明

计算两条DNA链之间的汉明差异.

突变只是在核酸的创建或复制过程中发生的错误,特别是DNA.因为核酸对细胞功能至关重要,突变倾向于在整个细胞中产生涟漪效应.虽然突变在技术上是错误的,但非常罕见的突变可能为细胞提供有益的属性.事实上,进化的宏观效应归因于许多代有益微观突变的累积结果.

最简单和最常见的核酸突变类型是点突变,它在单个核苷酸处用一个碱基替换另一个碱基.

通过计算从具有共同祖先的不同基因组中获取的两个同源DNA链之间的差异数量,我们得到在两条链之间的进化路径上可能发生的最小点突变数量的度量.

这就是所谓的"海明距离".

通过比较两条DNA链,并计算其中有多少核苷酸与其他序列中的同等核苷酸不同.

```
GAGCCTACTAACGGGAT
CATCGTAATGACGGCCT
^ ^ ^  ^ ^    ^^
```

这两条DNA链之间的汉明距离为7.

# 实施说明

Hamming距离仅定义为等长的序列,因此尝试计算不同长度的序列之间的Hamming距离不应该起作用.这种情况的一般处理(例如,引发异常与返回特殊值)可能在语言之间有所不同.

## 锈蚀装置

参考[练习帮助页面][help-page]用于锈蚀安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了.在获得第一个测试通过后,打开位于`tests`目录和删除`#[ignore]`从下一个测试标志,并使测试再次通过.每个单独的测试都是一个函数.`#[test]`旗帜在上面.继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下运行所有测试,请使用:

```bash
$ cargo test -- --ignored
```

运行特定的测试,例如`some_test`,您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,则使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关锈蚀试验的更多信息,请参阅[在线测试文档][rust-tests]

请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)如果你还没有,它将帮助你组织你的文件.

## 反馈、问题、拉动请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有锈蚀演习的所在地.如果你有关于锻炼的反馈,或者想帮助实施新的锻炼,就去那里创造一个问题.铁锈队的队员们很乐意帮忙!

如果你想了解更多关于运动的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

罗瑟琳的计算点突变问题<http://rosalind.info/problems/hamm/>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
