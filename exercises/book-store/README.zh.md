# 书店

为了尝试鼓励更多来自流行的5本书系列的不同书籍的销售,书店决定提供多本书购买的折扣.

这五本书中的任何一本都要花8美元.

但是,如果您购买两本不同的书籍,那么这两本书将获得5%的折扣.

如果您购买3本不同的书籍,您将获得10%的折扣.

如果您购买4本不同的书籍,您将获得20%的折扣.

如果您全部购买5件,即可获得25%的折扣.

注意:如果您购买了四本书,其中3本是不同的书籍,那么您可以在3本书中获得10%的折扣,但是第四本书的价格仍为8美元.

你的任务是写一段代码来计算任何可以想象的购物篮的价格(只包含同一系列的书籍),给予尽可能大的折扣.

例如,这篮子书的价格是多少?

-   第一本书的2份
-   第二本书的2份
-   第三本书的2份
-   第四本书的1份
-   第五本书的1份

将这8本书分组的一种方法是:

-   1组5  - > 25%折扣(第1,第2,第3,第4,第5)
-   \+1组3  - > 10%折扣(第1名,第2名,第3名)

这将总共给出:

-   5本书,25%的折扣
-   \+3本书可享受10%的折扣

导致:

-   5 x(8  -  2.00)== 5 x 6.00 == $ 30.00
-   \+3 x(8  -  0.80)== 3 x 7.20 == 21.60美元

总计51.60美元

但是,将这8本书分组的另一种方法是:

-   1组4本书 - > 20%折扣(第1,第2,第3,第4)
-   \+1组4本书 - > 20%折扣(第1,第2,第3,第5)

这将总共给出:

-   4本书,20%的折扣
-   \+4本书,20%的折扣

导致:

-   4 x(8  -  1.60)== 4 x 6.40 == 25.60美元
-   \+4 x(8  -  1.60)== 4 x 6.40 == 25.60美元

总计51.20美元

51.20美元是最大折扣的价格.

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

灵感来自Cyber​​-Dojo的哈利波特卡塔.<http://cyber-dojo.org>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
