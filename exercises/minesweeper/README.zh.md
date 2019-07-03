# 扫雷

将数字添加到扫雷板上.

扫雷器是一个流行的游戏，其中用户要用数字提示，来找到地雷，这些数字提示指示有多少地雷直接相邻(水平，垂直，对角，总 9 个格)，围成一个正方形.

在这个练习中，您必须创建一些代码，这些代码计算与正方形相邻的地雷数量，并且像这样转换地雷板(其中`*`表示地雷):

```
    +-----+
    | * * |
    |  *  |
    |  *  |
    |     |
    +-----+
```

变成:

```
    +-----+
    |1*3*1|
    |13*31|
    | 2*2 |
    | 111 |
    +-----+
```

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
