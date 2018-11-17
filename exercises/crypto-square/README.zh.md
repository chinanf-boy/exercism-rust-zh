# 加密广场

实现用于组成称为方形代码的秘密消息的经典方法.

给定英文文本,输出该文本的编码版本.

首先,输入被规范化:从英文文本中删除空格和标点符号,并且消息是向下的.

然后,规范化字符被分成行.当使用插入的换行符打印时,这些行可被视为形成矩形.

例如,句子

```text
"If man was meant to stay on the ground, god would have given us roots."
```

规范化为:

```text
"ifmanwasmeanttostayonthegroundgodwouldhavegivenusroots"
```

明文应该组织成一个矩形.矩形的大小(`r x c`)应该根据消息的长度来决定`c >= r`和`c - r <= 1`,哪里`c`是列数和`r`是行数.

我们的标准化文本长度为 54 个字符,用矩形指示矩形`c = 8`和`r = 7`:

```text
"ifmanwas"
"meanttos"
"tayonthe"
"groundgo"
"dwouldha"
"vegivenu"
"sroots  "
```

通过向下读取从左到右的列来获得编码消息.

上面的消息编码为:

```text
"imtgdvsfearwermayoogoanouuiontnnlvtwttddesaohghnsseoau"
```

以填充完美矩形的块输出编码文本`(r X c)`,与`c`大块的`r`长度,以空格分隔.对于那些短语`n`字符短于完美的矩形,每个填充最后一个`n`有一个尾随空间的块.

```text
"imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau "
```

请注意,如果我们要堆叠这些,我们可以直观地将密文解码回原始消息:

```text
"imtgdvs"
"fearwer"
"mayoogo"
"anouuio"
"ntnnlvt"
"wttddes"
"aohghn "
"sseoau "
```

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

J Dalbey 的编程实践问题<http://users.csc.calpoly.edu/~jdalbey/103/Projects/ProgrammingPractice.html>
