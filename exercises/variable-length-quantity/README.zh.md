# 可变长度数量

实现可变长度数量编码和解码.

这项工作的目标是实施[VLQ](https://en.wikipedia.org/wiki/Variable-length_quantity)编码/解码.

简而言之,此编码的目标是以节省字节的方式对整数值进行编码.只有每个字节的前 7 位是有效的(右对齐;有点像 ASCII 字节).因此,如果您有 32 位值,则必须将其解压缩为一系列 7 位字节.当然,根据您的整数,您将拥有可变数量的字节.要指出哪个是系列的最后一个字节,请将第 7 位清零.在所有前面的字节中,您设置位#7.所以,如果一个整数介于

,它可以表示为一个字节.`0-127`虽然 VLQ 可以处理任意大小的数字,但对于本练习,我们将仅限于适合 32 位无符号整数的数字.以下是整数作为 32 位值的示例,以及它们转换为的可变长度数量:锈蚀安装

```text
 NUMBER        VARIABLE QUANTITY
00000000              00
00000040              40
0000007F              7F
00000080             81 00
00002000             C0 00
00003FFF             FF 7F
00004000           81 80 00
00100000           C0 80 00
001FFFFF           FF FF 7F
00200000          81 80 80 00
08000000          C0 80 80 00
0FFFFFFF          FF FF FF 7F
```

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

一个糟糕的 Splice 开发人员必须实现 MIDI 编码/解码.<https://splice.com>
