# 算术谜题

写一个函数来解决字母谜题.

[Alphametics](https://en.wikipedia.org/wiki/Alphametics)是一个拼图,其中单词中的字母被数字替换.

例如`SEND + MORE = MONEY`:

```text
  S E N D
  M O R E +
-----------
M O N E Y
```

用有效数字替换它们会给出:

```text
  9 5 6 7
  1 0 8 5 +
-----------
1 0 6 5 2
```

这是正确的,因为每个字母都被不同的数字替换,并且单词被翻译成数字,然后产生有效的总和.

每个字母必须代表不同的数字,并且多位数的前导数字不得为零.

写一个函数来解决字母谜题.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html
