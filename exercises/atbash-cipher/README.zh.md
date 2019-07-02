# Atbash Cipher

创建 Atbash 密码的实现,这是在中东创建的古老加密系统.

Atbash 密码是一种简单的替换密码,它依赖于转置字母表中的所有字母,使得生成的字母表向后。

- 第一个字母替换为最后一个字母,
- 第二个字母替换为倒数第二个字母,
- 依此类推.

拉丁字母的 Atbash 密码如下:

```text
明文:  abcdefghijklmnopqrstuvwxyz
加密:  zyxwvutsrqponmlkjihgfedcba
```

它是一个非常弱的加密方式，因为它只有一个加密可能性，且为一个简单的单字母替换密码。但是,这并不是现在'加密游戏-练习时间'的问题.

密文以固定长度的组写出，传统的组大小为 5 个字母，并且不包括标点符号。这是为了使单词边界，更难猜测。

## 例子

- `test`，编码为`gvhg`
- `gvhg`，解码为`test`
- `gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt`解码为`thequickbrownfoxjumpsoverthelazydog`

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

维基百科<http://en.wikipedia.org/wiki/Atbash>
