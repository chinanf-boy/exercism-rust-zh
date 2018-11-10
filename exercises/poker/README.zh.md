# 扑克

从扑克手列表中挑选最好的手.

见[wikipedia](https://en.wikipedia.org/wiki/List_of_poker_hands)概述扑克手.

## 提示

-   排名扑克手可以被认为是一个排序问题.
-   锈提供[sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)方法`Vec<T> where T: Ord`.
-   [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html)是形式A[total order](https://en.wikipedia.org/wiki/Total_order)正是其中之一`a < b`,`a == b`或`a > b`一定是真的.
-   扑克手不符合一个总的顺序:两只手不相等,但有相同的排序.例子:`3S 4S 5D 6H JH"`,`"3H 4H 5C 6C JD"`.
-   锈提供[`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)处理不具有完全顺序的可处置事物的情况.然而,它没有提供标准.`sort`方法`Vec<T> where T: PartialOrd`. 在这种情况下,对向量进行排序的标准成语是`your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`这取决于你的需要.\`
-   您可以考虑实现表示扑克指针的类型.`PartialOrd`.

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

## 来源

受来自Udacity的培训课程的启发.<https://www.udacity.com/course/viewer#!/c-cs212/>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
