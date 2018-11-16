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

-   比赛:比赛
-   W:比赛赢了
-   比赛:抽签(并列)
-   L:比赛输了
-   P:点

一场胜利夺得一支球队3分.抽签赚1英镑.损失0英镑.

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

## 锈蚀装置

参考[练习帮助页面][help-page]用于锈蚀安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了.在获得第一个测试通过后,打开位于`tests`目录和删除`#[ignore]`从下一个测试标志,并使测试再次通过.每个单独的测试都是一个函数.`#[test]`旗帜在上面.继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下运行所有测试,请使用:

```bash
$ cargo test -- --ignored
```

运行特定的测试,例如`some_test`,您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,则使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关锈蚀试验的更多信息,请参阅[在线测试文档][rust-tests]

请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)如果你还没有,它将帮助你组织你的文件.

## 反馈、问题、拉动请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有锈蚀演习的所在地.如果你有关于锻炼的反馈,或者想帮助实施新的锻炼,就去那里创造一个问题.铁锈队的队员们很乐意帮忙!

如果你想了解更多关于运动的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
