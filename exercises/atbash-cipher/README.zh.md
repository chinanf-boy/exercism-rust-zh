# Atbash Cipher

创建 atbash 密码的实现,这是在中东创建的古老加密系统.

Atbash 密码是一种简单的替换密码,它依赖于转置字母表中的所有字母,使得生成的字母表向后.第一个字母替换为最后一个字母,第二个字母替换为倒数第二个字母,依此类推.

拉丁字母的 Atbash 密码如下:

```text
Plain:  abcdefghijklmnopqrstuvwxyz
Cipher: zyxwvutsrqponmlkjihgfedcba
```

它是一个非常弱的密码,因为它只有一个可能的密钥,它是一个简单的单字母替换密码.但是,这可能不是密码时代的问题.

密文以固定长度的组写出,传统的组大小为 5 个字母,并且不包括标点符号.这是为了使基于单词边界的东西更难猜测.

## 例子

- 编码`test`给`gvhg`
- 解码`gvhg`给`test`
- 解码`gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt`给`thequickbrownfoxjumpsoverthelazydog`

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<http://en.wikipedia.org/wiki/Atbash>

```

```
