# 二分查找

实现二分查找算法.

搜索已排序的集合是一项常见任务。字典是定义单词的排序列表。有了一个词，就可以找到它的定义。电话簿是人员姓名，地址和电话号码的分类列表。知道某人的姓名可以让他们快速找到他们的电话号码和地址。

如果要搜索的列表包含多个项目(比如十几个)，则二分查找将比线性搜索需要更少的比较，但它强制要求对列表进行排序。

在计算机科学中，二分查找或半间隔搜索算法在按键值排序的数组中，查找指定输入值(搜索"关键字")的位置。

在每个步骤中，算法将搜索关键字值与数组中间元素的值进行比较。

如果匹配，则找到匹配元素，并返回其索引或位置。

否则，如果搜索关键字小于中间元素的键，则算法在中间元素左侧的子阵列上重复其操作，或者如果搜索关键字更大，则在右侧的子阵列上重复其操作。

如果要搜索的剩余阵列为空，则在阵列中找不到该键值，并返回特殊的"not found"指示。

二分查找将每次迭代检查的项目数减半，因此定位项目(或确定其不存在)需要对数时间。二分搜索是一种对半分，并渐进的搜索算法。

## 限制

Rust 已在其标准库中提供了一个[二分查找 函数](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search).对于本练习，您不应使用此函数，而应使用其他基本工具.

## 提示

[Slices](https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html)除了具有通过索引，正常访问元素(slice[索引])，还有很多有用的函数，像[split_at](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)[给予 子区间](https://doc.rust-lang.org/std/primitive.slice.html#method.get)(slice[start..end]).

您可以通过索引，使用无聊的旧元素访问来解决此练习，但也许其他提供的函数可以使您的代码更清晰，更安全.

## 加分

你是否通过了测试并且代码干净了?如果你愿意，你可以尝试一些其他的东西.

- 目前，您的查找函数可能仅适用于数字切片，但 Rust 类型系统足够灵活，可以创建一个查找函数，该函数适用于包含可排序元素的所有切片.
- 此外，此查找函数不仅可以在切片上工作，还可以在 Vec 或数组上同时工作.

要运行加分测试，请删除`#[ignore]`标记，并执行测试`generic`函数，像这样:

```bash
$ cargo test --features generic
```

### 加分的提示

- 为了使您的函数与所有可排序的元素一起使用，请查看[Ord Trait](https://doc.rust-lang.org/std/cmp/trait.Ord.html).
- 要使您的函数直接在 Vec 和 Array 上运行，您可以使用[AsRef Trait](https://doc.rust-lang.org/std/convert/trait.AsRef.html)

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<http://en.wikipedia.org/wiki/Binary_search_algorithm>
