# 说

提供一个 0 到 999,999,999,999 之间的数字,用英语拼出这个数字.

## 步骤 1

处理 0 到 99 的基本情况.

如果程序的输入是`22`,那么输出应该是`'twenty-two'`.

如果给出超出规定范围的数字,您的程序应该大声抱怨。

这个程序的一些好的测试用例是:

- `0`
- `14`
- `50`
- `98`
- `-1`
- `100`

### 扩展

如果您使用的是 Mac,那就是 Mac OS X 的 shell 可以用`say`程序大声说出来。如果您使用的是 Linux 或 Windows,则可以使用`espeak`命令使用 eSpeakNG.

## 第 2 步

实现将数子分成块.

像`1234567890`，应该产生如 1,234,567 和 890 的列表,而更简单`1000`应该只产生 1 和 0.

该程序还必须报告超出范围的值。

## 第 3 步

现在处理在这些块之间，插入适当的缩放词.

所以`1234567890`应该输出`'1 billion 234 million 567 thousand 890'`

该程序还必须报告超出范围的值。上限为"兆（trillion）"处就可以.

## 第 4 步

把它们放在一起，除了简单的英语之外什么都没有.

`12345`应该给出`twelve thousand three hundred forty-five`.

该程序还必须报告超出范围的值.

### 扩展

(正确地)使用英语*and*合并数字:

- `14` => "fourteen".
- `100` => "one hundred".
- `120` => "one hundred and twenty".
- `1002` => "one thousand and two".
- `1323` => "one thousand three hundred and twenty-three".

## 特定的 Rust 练习笔记

与本练习的其他语言版本相比,Rust 版本略有改变。我们使用 Rust 的强类型系统来限制输入,而不是要求您返回超出范围的错误。让函数处理所有有效输入要容易得多,而不是要求，使用模块的用户处理错误。

有一个 -1 版本的测试用例,但它被注释掉了。如果您的函数正确实现,则不应编译 -1 测试用例.

在测试用例中尚未实现，将"and"添加到数字文本中.

### 扩展

添加转换为 u64 的最大值的功能:9,223,372,036,854,775,807.

有关输出的提示,请查看最后一个测试用例.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

JavaRanch CattleDrive 的变体,练习 4a<http://www.javaranch.com/say.jsp>
