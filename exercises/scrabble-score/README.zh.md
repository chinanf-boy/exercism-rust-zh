# 拼字游戏分数

给出一个单词,计算该单词的拼字游戏分数.

## 字母价值

你需要这些:

```text
Letter                           Value
A, E, I, O, U, L, N, R, S, T       1
D, G                               2
B, C, M, P                         3
F, H, V, W, Y                      4
K                                  5
J, X                               8
Q, Z                               10
```

## 例子

"白菜"的得分值应为 14 分:

- C 得 3 分
- 1 分为 A,两次
- B 为 3 分,两次
- G 得 2 分
- E 点 1 分

总计:

- `3 + 2*1 + 2*3 + 2 + 1`
- = `3 + 2 + 6 + 3`
- = `5 + 9`
- = 14

## 扩展

- 您可以播放双重或三重字母.
- 你可以演奏一个双重或三个单词.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

受到 Extreme Startup 游戏的启发<https://github.com/rchatley/extreme_startup>
