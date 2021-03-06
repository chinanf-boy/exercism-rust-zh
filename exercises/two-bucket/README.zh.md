# 两个桶

给定两个不同尺寸的桶，演示如何通过两个桶，策略性地传输液体来测量精确的升数.

由于这个数学问题，很容易受到解释/个体方法的影响，因此这些测试代码专门针对期望一个总体解决方案而编写.

为了提供帮助，测试代码首先为桶(1/2)装满。这意味着，若开始时，选择较大的桶装满，那就不会出现，较小的桶装满而较大的桶为空的情况。(也就是说，起点应该用较小桶)；不然这会破坏比较两种方法的目的!

您的程序将作为输入:

- 桶 1 的大小
- 桶 2 的大小
- 要达到的理想升数
- 首先要装满哪个桶，要么是桶 1 还是桶 2

您的计划应确定:

- 达到所需的升数，所需的"移动"总数，包括第一次装满
- 哪个桶应该以所需的升数结束(假设这是桶 A) - 要么桶 1 ，要么桶 2
- 另一个桶中，剩下多少升(桶 B)

注意:任何时候对其中一个或两个桶进行更改，都计为一(1)次移动。

示例：桶 1 最多可容纳 7 升，桶 2 最多可容纳 11 升。让我们说桶 1，第一步，持有 7 升，桶 2 持有 8 升(7，8)。如果您清空桶 1，并且不对桶 2 进行任何更改，则分别为 0 升和 8 升(0，8)，这将作为一个"移动"。相反，如果你已经把桶 1 倒入桶 2，直到桶 2 充满，那留下的，桶 1 中的 4 升和桶 2 中的 11 升(4，11)，这也算作只有一个"移动"。

总而言之，唯一有效的行动是:

- 从一个桶，倒到另一个桶
- 清空一个桶，对另一个什么都不做
- 装满一个桶，对另一个什么也不做

写\<3 于[全栈教程 Academy](http://www.fullstackacademy.com/)作 者:Lindsay Levine.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

水浇注问题<http://demonstrations.wolfram.com/WaterPouringProblem/>
