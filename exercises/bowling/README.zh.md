# 保龄球

打保龄球比赛。

保龄球是一种游戏，玩家掷出一个沉重的球来击倒排列成三角形的罐子。编写代码以跟踪保龄球比赛的得分。

## 保龄球得分

游戏由 10 轮组成。一轮由一个或两个球投掷组成，其中 10 个罐子处于该轮初始化状态。一个回合有三种情况。

- 开放(open)轮是轮的记录小于 10 的分数。在这种情况下，该轮的分数是被击倒数。

- 备用(spare)的是所有十个罐子被第二次投掷击倒。备用的总分是 10 加上，下一次投掷击倒的罐子数量。

- 全倒(strike)是所有十个罐子被第一次击倒。全倒的总分是 10 加上，在接下来的两次投掷击倒的罐子数量。如果进行第二次击球又触发 strike，则再扔球之前不确定第一次击球的值。

这是一个三轮的例子:

|  第 1 轮  |  第 2 轮   |  第 3 轮  |
| :-------: | :--------: | :-------: |
| X(strike) | 5 /(spare) | 9 0(open) |

第 1 轮是(10 + 5 + 5)= 20

第 2 轮是(5 + 5 + 9)= 19

轮 3 是(9 + 0)= 9

这意味着当前的运行总数为 48.

游戏中的第十轮是一个特例。如果有人投掷全倒或备用，那么他们会得到一个补球。补球会计入第 10 轮的总和。在补球上获得一次全倒或备用不会给球员带来更多的补球。第 10 轮的总分是被击倒的罐子总数。

对于 X1 /(全倒和备用)的第十轮，总分为 20。

对于 XXX (三次全倒)的第十轮，总分为 30。

## 要求

编写代码以跟踪保龄球比赛的得分。它应该支持两个操作:

- `roll(pins : int)`每次玩家滚球时都会调用。这个参数是被击倒的罐子数量.
- `score() : int`仅在游戏结束时才会被调用。它返回该游戏的总分。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

保龄球比赛， Kata 的 UncleBob<http://butunclebob.com/ArticleS.UncleBob.TheBowlingGameKata>
