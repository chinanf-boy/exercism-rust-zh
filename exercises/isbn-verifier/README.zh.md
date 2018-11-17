# ISBN 检验器

这个[ISBN-10 verification process](https://en.wikipedia.org/wiki/International_Standard_Book_Number)用于验证图书识别号.这些通常包含破折号,看起来像:`3-598-21508-8`

## 国际标准书号

ISBN-10 格式是 9 位数字(0 到 9)加上一个校验字符(一个数字或一个 X).在校验字符为 x 的情况下,这表示值"10".这些可以与有或没有连字符通信,并且可以通过以下公式检查它们的有效性:

(x1 _ 10 + x2 _ 9 + x3 _ 8 + x4 _ 7 + x5 _ 6 + x6 _ 5 + x7 _ 4 + x8 _ 3 + x9 _ 2 + x10 _ 1) mod 11 == 0

如果结果是 0,那么它是一个有效的 ISBN-10,否则它是无效的.

## 例子

让我们用 ISBN-10`3-598-21508-8`. 我们把它插入到公式中,得到:

(3 _ 10 + 5 _ 9 + 9 _ 8 + 8 _ 7 + 2 _ 6 + 1 _ 5 + 5 _ 4 + 0 _ 3 + 8 _ 2 + 8 _ 1) mod 11 == 0

由于结果是 0,这证明我们的 ISBN 是有效的.

## 任务

给定一个字符串,程序应该检查所提供的字符串是否是有效的 ISBN-10.实现这一点需要在计算 ISBN 的校验位数之前考虑字符串的预处理/解析.

该程序应该能够验证 ISBN-10 既有,也不分离破折号.

## 告诫

在某些语言中,从字符串转换为数字可能是棘手的.现在,甚至更棘手的是,ISBN-10 的校验位可能是"X"(表示"10").例如`3-598-21507-X`是一个有效的 ISBN-10.

## 奖金任务

- 从输入 ISBN-10 生成有效的 ISBN-13(并且可能用派生验证器再次验证它).

- 生成有效的 ISBN,甚至可能从给定的起始 ISBN 中生成.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

将一个字符串转换成一个数字和一些基本的处理,利用一个可靠的真实世界的例子.<https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-10_check_digit_calculation>
