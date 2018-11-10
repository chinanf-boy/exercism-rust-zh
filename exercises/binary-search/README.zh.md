# 二进制搜索

实现二进制搜索算法.

搜索已排序的集合是一项常见任务.字典是单词定义的排序列表.有了一个词,就可以找到它的定义.电话簿是人员姓名,地址和电话号码的分类列表.知道某人的姓名可以让他们快速找到他们的电话号码和地址.

如果要搜索的列表包含多个项目(比如十几个),则二进制搜索将比线性搜索需要更少的比较,但它强制要求对列表进行排序.

在计算机科学中,二进制搜索或半间隔搜索算法在按键值排序的数组中查找指定输入值(搜索"键")的位置.

在每个步骤中,算法将搜索关键字值与数组中间元素的关键值进行比较.

如果键匹配,则找到匹配元素并返回其索引或位置.

否则,如果搜索关键字小于中间元素的键,则算法在中间元素左侧的子阵列上重复其操作,或者如果搜索关键字更大,则在右侧的子阵列上重复其操作.

如果要搜索的剩余阵列为空,则在阵列中找不到该键,并返回特殊的"未找到"指示.

二进制搜索将每次迭代检查的项目数减半,因此定位项目(或确定其不存在)需要对数时间.二分搜索是一种二分法并征服搜索算法.

## 限制

Rust已在其标准库中提供了一个[binary search function](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search).对于本练习,您不应使用此功能,而应使用其他基本工具.

## 提示

[Slices](https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html)另外还有通过索引的正常元素访问(切片[指数])很多有用的功能[split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)要么[getting
subslices](https://doc.rust-lang.org/std/primitive.slice.html#method.get)(切片[start..end]).

您可以通过索引使用无聊的旧元素访问来解决此练习,但也许其他提供的函数可以使您的代码更清晰,更安全.

## 奖励积分

你是否通过了测试并且代码干净了?如果你愿意,你可以尝试一些其他的东西.

-   目前,您的查找函数可能仅适用于数字切片,但Rust类型系统足够灵活,可以创建一个查找函数,该函数适用于包含可订购元素的所有切片.
-   此外,此查找功能不仅可以在切片上工作,还可以在Vec或数组上同时工作.

要运行奖励测试,请删除`#[ignore]`标记并执行测试`generic`功能,像这样:

```bash
$ cargo test --features generic
```

那么请在提交的评论中分享您的想法.这个实验是否使代码更好?更差?你从中学到了什么吗?

### 奖励积分的提示

-   为了使您的功能与所有可订购的元素一起使用,请查看[Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
-   要使您的功能直接在Vec和Array上运行,您可以使用[AsRef Trait](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

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

维基百科<http://en.wikipedia.org/wiki/Binary_search_algorithm>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
