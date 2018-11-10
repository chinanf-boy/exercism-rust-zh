# 循环缓冲区

循环缓冲区,循环缓冲区或环形缓冲区是一种数据结构,它使用单个固定大小的缓冲区,就好像它是端到端连接一样.

循环缓冲区首先开始为空并具有一些预定义的长度.例如,这是一个7元素的缓冲区:

```
[ ][ ][ ][ ][ ][ ][ ]
```

假设1被写入缓冲区的中间(精确的起始位置在循环缓冲区中无关紧要):

```
[ ][ ][ ][1][ ][ ][ ]
```

然后假设添加了另外两个元素 -  2和3  - 在1之后附加:

```
[ ][ ][ ][1][2][3][ ]
```

如果从缓冲区中删除了两个元素,则删除缓冲区内最旧的值.在这种情况下,删除的两个元素是1和2,缓冲区只有3:

```
[ ][ ][ ][ ][ ][3][ ]
```

如果缓冲区有7个元素,那么它就完全填满了:

```
[6][7][8][9][3][4][5]
```

当缓冲区已满时,将引发错误,警告客户端进一步写入被阻止,直到插槽空闲为止.

当缓冲区已满时,客户端可以选择使用强制写入覆盖最旧的数据.在这种情况下,添加了另外两个元素 -  A和B  - 它们会覆盖3和4:

```
[6][7][8][9][A][B][5]
```

3和4已被A和B取代,使得5现在是缓冲区中最旧的数据.最后,如果删除了两个元素,那么返回的是5和6,产生缓冲区:

```
[ ][7][8][9][A][B][ ]
```

因为有可用的空间,如果客户端再次使用覆盖来存储C&D,那么先前存储5和6的空间将被使用而不是7和8的位置.7仍然是最旧的元素,缓冲区又是充分.

```
[D][7][8][9][A][B][C]
```

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

## 资源

维基百科<http://en.wikipedia.org/wiki/Circular_buffer>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
