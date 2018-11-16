# 两个桶

给定两个不同尺寸的铲斗,演示如何通过在铲斗之间策略性地传输液体来测量精确的升数.

由于这个数学问题很容易受到解释/个体方法的影响,因此这些测试专门针对期望一个总体解决方案而编写.

为了提供帮助,测试为您提供了首先填充的存储桶.这意味着,当从较大的桶装满开始时,不允许在任何时刻将较小的桶装满并且较大的桶装空(也就是说,相反的起点);这会破坏比较两种方法的目的!

您的程序将作为输入:

-   斗一大小
-   斗二的大小
-   达到的理想升数
-   首先要填充哪个桶,要么是桶1还是桶2

您的计划应确定:

-   达到所需的升数所需的"移动"总数,包括第一次填充
-   哪个桶应该以所需的升数结束(假设这是桶A) - 桶1或桶2
-   另一个桶中剩下多少升(桶B)

注意:任何时候对其中一个或两个桶进行更改都会计为一(1)次移动.

示例:铲斗最多可容纳7升,铲斗2最多可容纳11升.让我们说第一桶,在给定的步骤,持有7升,桶2持有8升(7,8).如果您清空铲斗1并且不对铲斗2进行任何更改,则分别为0升和8升(0,8),这将作为一个"移动".相反,如果你已经从桶1倒入桶2,直到桶2充满,留下你在桶2中的4升和桶2中的11升(4,11),这也算作只有一个"移动".

总而言之,唯一有效的举措是:

-   从一个桶倒到另一个桶
-   清空一个桶,对另一个什么都不做
-   填充一个桶,对另一个什么也不做

写着\<3 at[Fullstack Academy](http://www.fullstackacademy.com/)作者:Lindsay Levine.

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

水浇注问题<http://demonstrations.wolfram.com/WaterPouringProblem/>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
