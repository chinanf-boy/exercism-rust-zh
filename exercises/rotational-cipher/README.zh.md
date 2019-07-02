# 旋转密码

创建旋转密码的实现,有时也称为 Caesar 密码.

Caesar 密码是一个简单的移位密码，它依赖于使用`0`到`26`整数(key)，在字母表中转置所有字母。由于模运算，使用`0`要么`26`，总是会产生相同的输出。将字母移动为与 key 值一样多的值。

旋转密码的一般表示法是`ROT + <key>`。最常用的旋转密码是`ROT13`.

一个拉丁字母表的`ROT13`加密如下:

```text
原文:  abcdefghijklmnopqrstuvwxyz
密文:  nopqrstuvwxyzabcdefghijklm
```

它比 Atbash 密码更强大,因为它有 27 个可能性 key，和 25 个可用的密文.

密文会与输入相同的格式写出,包括空格和标点符号.

## 例子

- ROT5 `omg`给`trl`
- ROT0 `c`给`c`
- ROT26 `Cool`给`Cool`
- ROT13 `The quick brown fox jumps over the lazy dog.`给`Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`
- ROT13 `Gur dhvpx oebja sbk whzcf bire gur ynml qbt.`给`The quick brown fox jumps over the lazy dog.`

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Caesar_cipher>
