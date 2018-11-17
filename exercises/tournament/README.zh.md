# 锦标赛

统计一场小型足球比赛的结果.

基于一个输入文件,其中包含哪个队与哪个队进行比赛,结果是什么,用这样的表创建一个文件:

```text
Team                           | MP |  W |  D |  L |  P
Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
Blithering Badgers             |  3 |  1 |  0 |  2 |  3
Courageous Californians        |  3 |  0 |  1 |  2 |  1
```

那些缩写是什么意思?

- 比赛:比赛
- W:比赛赢了
- 比赛:抽签(并列)
- L:比赛输了
- P:点

一场胜利夺得一支球队 3 分.抽签赚 1 英镑.损失 0 英镑.

结果应该按点排序,下降.在平局的情况下,球队按字母顺序排列.

###

输入

你的理货程序将接收看起来像:

```text
Allegoric Alaskans;Blithering Badgers;win
Devastating Donkeys;Courageous Californians;draw
Devastating Donkeys;Allegoric Alaskans;win
Courageous Californians;Blithering Badgers;loss
Blithering Badgers;Devastating Donkeys;loss
Allegoric Alaskans;Courageous Californians;win
```

比赛的结果是指第一队的名单.所以这条线

```text
Allegoric Alaskans;Blithering Badgers;win
```

意味着寓言阿拉斯加打败了獾.

这条线:

```text
Courageous Californians;Blithering Badgers;loss
```

意味着獾会打败勇敢的加利福尼亚人.

这条线:

```text
Devastating Donkeys;Courageous Californians;draw
```

意味着毁灭性的驴子和勇敢的加利福尼亚人并驾齐驱.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html
