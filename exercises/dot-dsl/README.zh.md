# DOT DSL

编写类似于Graphviz点语言的领域特定语言.

一个[Domain Specific Language
(DSL)](https://en.wikipedia.org/wiki/Domain-specific_language)是针对特定域优化的小语言.

比如说[DOT language](https://en.wikipedia.org/wiki/DOT_(graph_description_language))允许您编写图形的文本描述,然后通过其中一个图形转换为图像[Graphviz](http://graphviz.org/)工具(如`dot`).一个简单的图形如下所示:

```
graph {
    graph [bgcolor="yellow"]
    a [color="red"]
    b [color="blue"]
    a -- b [color="green"]
}
```

把它放在一个文件中`example.dot`并运行`dot example.dot -T png
-o example.png`创建一个图像`example.png`与黄色背景上的绿线连接的红色和蓝色圆圈.

创建类似于点语言的DSL.

## 生成器模式

本练习希望您使用构建多个结构`builder pattern`.简而言之,此模式允许您将包含大量参数的结构的构造函数拆分为多个单独的函数.这种方法为您提供了实现紧凑但高度灵活的结构构造和配置的方法.你可以在上面阅读更多相关信息[following page](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).

## 锈蚀安装

参考[运动帮助页面][help-page]用于Rust安装和学习资源.

## 编写代码

执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有测试都被忽略了在第一个测试通过后,打开位于的测试源文件`tests`目录并删除`#[ignore]`从下一次测试中标记并再次通过测试.每个单独的测试都是一个函数`#[test]`它上面的旗帜.继续,直到通过每个测试.

如果您希望在不编辑测试源文件的情况下运行所有​​测试,请使用:

```bash
$ cargo test -- --ignored
```

例如,运行特定测试`some_test`, 您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,请使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

一定要阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节如果你还没有,它将帮助你组织你的文件.

## 反馈,问题,请求

该[exercism/rust](https://github.com/exercism/rust)GitHub上的存储库是所有Rust练习的主页.如果您有关于练习的反馈,或者想要帮助实施新的练习,请前往那里并创建一个问题.铁轨团队成员很乐意为您提供帮助!

如果你想了解更多关于运动的知识,请看看[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
