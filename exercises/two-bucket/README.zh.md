# 两个桶

给定两个不同尺寸的铲斗,演示如何通过在铲斗之间策略性地传输液体来测量精确的升数.

由于这个数学问题很容易受到解释/个体方法的影响,因此这些测试专门针对期望一个总体解决方案而编写.

为了提供帮助,测试为您提供了首先填充的存储桶.这意味着,当从较大的桶装满开始时,不允许在任何时刻将较小的桶装满并且较大的桶装空(也就是说,相反的起点);这会破坏比较两种方法的目的!

您的程序将作为输入:

- 斗一大小
- 斗二的大小
- 达到的理想升数
- 首先要填充哪个桶,要么是桶 1 还是桶 2

您的计划应确定:

- 达到所需的升数所需的"移动"总数,包括第一次填充
- 哪个桶应该以所需的升数结束(假设这是桶 A) - 桶 1 或桶 2
- 另一个桶中剩下多少升(桶 B)

注意:任何时候对其中一个或两个桶进行更改都会计为一(1)次移动.

示例:铲斗最多可容纳 7 升,铲斗 2 最多可容纳 11 升.让我们说第一桶,在给定的步骤,持有 7 升,桶 2 持有 8 升(7,8).如果您清空铲斗 1 并且不对铲斗 2 进行任何更改,则分别为 0 升和 8 升(0,8),这将作为一个"移动".相反,如果你已经从桶 1 倒入桶 2,直到桶 2 充满,留下你在桶 2 中的 4 升和桶 2 中的 11 升(4,11),这也算作只有一个"移动".

总而言之,唯一有效的举措是:

- 从一个桶倒到另一个桶
- 清空一个桶,对另一个什么都不做
- 填充一个桶,对另一个什么也不做

写着\<3 at[Fullstack Academy](http://www.fullstackacademy.com/)作者:Lindsay Levine.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

水浇注问题<http://demonstrations.wolfram.com/WaterPouringProblem/>
