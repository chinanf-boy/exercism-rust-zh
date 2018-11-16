# OCR号码

给定一个3 x 4网格的管道,下划线和空格,确定代表哪个数字,或者是否是乱码.

# 第一步

首先,将简单的二进制字体转换为包含0或1的字符串.

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

更新程序以识别所有数字0到9,既可以单独识别,也可以作为更大字符串的一部分识别.

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

灵感来自银行OCR kata<http://codingdojo.org/cgi-bin/wiki.pl?KataBankOCR>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
