# 锦标赛

统计一场小型足球比赛的结果.

基于一个输入文件，它包含哪个队与哪个队进行比赛，结果是什么。用下面的表格创建出一个文件:

```text
Team                           | MP |  W |  D |  L |  P
Devastating Donkeys            |  3 |  2 |  1 |  0 |  7
Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6
Blithering Badgers             |  3 |  1 |  0 |  2 |  3
Courageous Californians        |  3 |  0 |  1 |  2 |  1
```

那些缩写是什么意思?

- MP：赛次
- W：比赛赢了
- D：打平
- L：比赛输了
- P：分

一场胜利，3 分。打平 1 分。输了 0 分.

结果应该按分数下降排序。在平局的情况下，球队按字母顺序排列。

### 输入

你的梳理程序，将接收像这样的输入:

```text
Allegoric Alaskans;Blithering Badgers;win
Devastating Donkeys;Courageous Californians;draw
Devastating Donkeys;Allegoric Alaskans;win
Courageous Californians;Blithering Badgers;loss
Blithering Badgers;Devastating Donkeys;loss
Allegoric Alaskans;Courageous Californians;win
```

一行中首个队伍名是主队。所以这一行

```text
Allegoric Alaskans;Blithering Badgers;win
```

意味着，Allegoric Alaskans 打败了 Blithering Badgers。

这行:

```text
Courageous Californians;Blithering Badgers;loss
```

意味着，Courageous Californians 输给了 Blithering Badgers

这行:

```text
Devastating Donkeys;Courageous Californians;draw
```

意味着，Devastating Donkeys 与 Courageous Californians 打平

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
