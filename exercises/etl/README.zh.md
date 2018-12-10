# ETL

ETL，是英文 `Extract-Transform-Load` 的缩写，用来描述将数据从来源端经过萃取、转置、加载至目的端的过程

### ETL

提取转换负载(ETL)是一种很有意思的说法,"我们在这个系统中有一些遗留的遗留数据,现在我们需要在这个闪亮的新系统中使用,所以我们将迁移它."

(通常情况下,接下来是，"我们只需要运行一次就好啦。"之后，通常会有很多怒拍额头,并抱怨自身有多么愚蠢。

### 目标

我们将从遗留系统中，提取一些拼字游戏分数.

旧的系统存储每一个字母的列表:

- 1 分:"A","E","I","O","U","L","N","R","S","T",
- 2 分:"D","G",
- 3 分:"B"、"C"、"M"、"P",
- 4 分:"F"、"H"、"V"、"W"、"Y",
- 5 分:"K",
- 8 分:"J","X",
- 10 分:"Q","Z",

闪亮的新拼写系统存储每个字母的分数，这使得计算一个单词的分数更快、更容易。它也把字母存为小写字母，而不考虑输入字母的情况:

- "a" 值 1 分.
- "b" 值 3 分.
- "c" 值 3 分.
- "d" 值 2 分.
- 等.

你的任务,你应该选择接受它, 应将遗留数据格式，转换成闪亮的新格式。

### 笔记

关于得分的最后一点是,Scrabble 这款游戏在世界各地，多个国家，每种语言都有自己独特的得分表。例如,在毛利(Māori)语版本的游戏中,"E"得分为 2 分,而在夏威夷语版本中为 4 分。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

JunpStab 实验室团队<http://jumpstartlab.com>
