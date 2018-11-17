# 旋转密码

创建旋转密码的实现,有时也称为 Caesar 密码.

Caesar 密码是一个简单的移位密码,它依赖于使用整数键在字母表中转置所有字母`0`和`26`.使用密钥`0`要么`26`由于模运算,总是会产生相同的输出.将字母移动为与键值一样多的值.

旋转密码的一般表示法是`ROT + <key>`.最常用的旋转密码是`ROT13`.

一个`ROT13`拉丁字母表如下:

```text
Plain:  abcdefghijklmnopqrstuvwxyz
Cipher: nopqrstuvwxyzabcdefghijklm
```

它比 Atbash 密码更强大,因为它有 27 个可能的密钥和 25 个可用的密钥.

密文以与输入相同的格式写出,包括空格和标点符号.

## 例子

- ROT5`omg`给`trl`
- ROT0`c`给`c`
- ROT26`Cool`给`Cool`
- ROT13`The quick brown fox jumps over the lazy dog.`给`Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`
- ROT13`Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`给`The quick brown fox jumps over the lazy dog.`

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Caesar_cipher>
