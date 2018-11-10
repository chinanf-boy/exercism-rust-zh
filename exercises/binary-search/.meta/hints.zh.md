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
