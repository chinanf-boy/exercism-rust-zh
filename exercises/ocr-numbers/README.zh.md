# OCR 号码

给定一个 3 x 4 网格的管道,下划线和空格,确定代表哪个数字,或者是否是乱码.

# 第一步

首先,将简单的二进制字体转换为包含 0 或 1 的字符串.

二进制字体使用管道和下划线,四行高,三列宽.

```text
     _   #
    | |  # zero.
    |_|  #
         # the fourth row is always blank
```

转换为"0"

```text
         #
      |  # one.
      |  #
         # (blank fourth row)
```

转换为"1"

如果输入的大小正确但无法识别,则程序应返回"?"

如果输入的大小不正确,程序应该返回错误.

# 第二步

更新您的程序以识别多字符二进制字符串,用?替换乱码?

# 第三步

更新程序以识别所有数字 0 到 9,既可以单独识别,也可以作为更大字符串的一部分识别.

```text
 _
 _|
|_
```

转换为"2"

```text
      _  _     _  _  _  _  _  _  #
    | _| _||_||_ |_   ||_||_|| | # decimal numbers.
    ||_  _|  | _||_|  ||_| _||_| #
                                 # fourth line is always blank
```

被转换为"1234567890"

# 第四步

更新程序以处理多个数字,每行一个.转换多行时,请使用逗号连接行.

```text
    _  _
  | _| _|
  ||_  _|

    _  _
|_||_ |_
  | _||_|

 _  _  _
  ||_||_|
  ||_| _|
```

被转换为"123,456,789"

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

灵感来自银行 OCR kata<http://codingdojo.org/cgi-bin/wiki.pl?KataBankOCR>
