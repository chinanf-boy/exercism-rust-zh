# 保龄球

打保龄球比赛.

保龄球是一种游戏,玩家掷出一个沉重的球来击倒排列成三角形的别针.编写代码以跟踪保龄球比赛的得分.

## 保龄球得分

游戏由10帧组成.一个框架由一个或两个球投掷组成,其中10个引脚处于框架初始化状态.框架列表有三种情况.

-   开放帧是帧的记录小于10的分数.在这种情况下,框架的分数是被击倒的销数.

-   备用的是所有十个引脚被第二次投掷击倒的地方.备件的总价值是10加上下一次投掷中被击倒的引脚数量.

-   罢工是所有十个引脚被第一次击倒击倒的地方.罢工的总价值是10加上在接下来的两次投掷中击倒的引脚数量.如果击球后立即进行第二次击球,则在球再次被击出之前无法确定第一次击球的值.

这是一个三帧的例子:

| 第1帧 | 第2帧 | 第3帧 |
| :-: | :-: | :-: |
| X(打击) | 5 /(备用) | 9 0(开架) |

第1帧是(10 + 5 + 5)= 20

第2帧是(5 + 5 + 9)= 19

帧3是(9 + 0)= 9

这意味着当前的运行总数为48.

游戏中的第十帧是一个特例.如果有人投掷罢工或备用,那么他们会得到一个补球.存在填充球以计算第10帧的总和.在填充球上获得一次击球或备用不会给球员带来更多的补球.第10帧的总值是被击倒的引脚总数.

对于X1 /(打击和备用)的第十帧,总值为20.

对于XXX的第十帧(三次打击),总值为30.

## 要求

编写代码以跟踪保龄球比赛的得分.它应该支持两个操作:

-   `roll(pins : int)`每次玩家滚球时都会调用.这个论点是被击倒的引脚数量.
-   `score() : int`仅在游戏结束时才会被调用.它返回该游戏的总分.

## 锈蚀安装

参考[运动帮助页面][help-page]用于Rust安装和学习资源.

## 编写代码

执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有测试都被忽略了在第一个测试通过后,打开位于的测试源文件`tests`目录并删除`#[ignore]`从下一次测试中标记并再次通过测试.每个单独的测试都是一个函数`#[test]`它上面的旗帜.继续,直到通过每个测试.

如果您希望在不编辑测试源文件的情况下运行所有​​测试,请使用:

```bash
$ cargo test -- --ignored
```

例如,运行特定测试`some_test`, 您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,请使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

一定要阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节如果你还没有,它将帮助你组织你的文件.

## 反馈,问题,请求

该[exercism/rust](https://github.com/exercism/rust)GitHub上的存储库是所有Rust练习的主页.如果您有关于练习的反馈,或者想要帮助实施新的练习,请前往那里并创建一个问题.铁轨团队成员很乐意为您提供帮助!

如果你想了解更多关于运动的知识,请看看[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

保龄球比赛Kata但是UncleBob<http://butunclebob.com/ArticleS.UncleBob.TheBowlingGameKata>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
