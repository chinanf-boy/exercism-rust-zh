## 提示

-   排名扑克手可以被认为是一个排序问题.
-   锈提供[sort](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort)方法`Vec<T> where T: Ord`.
-   [`Ord` types](https://doc.rust-lang.org/std/cmp/trait.Ord.html)是形式A[total order](https://en.wikipedia.org/wiki/Total_order)正是其中之一`a < b`,`a == b`或`a > b`一定是真的.
-   扑克手不符合一个总的顺序:两只手不相等,但有相同的排序.例子:`3S 4S 5D 6H JH"`,`"3H 4H 5C 6C JD"`.
-   锈提供[`PartialOrd` trait](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html)处理不具有完全顺序的可处置事物的情况.然而,它没有提供标准.`sort`方法`Vec<T> where T: PartialOrd`. 在这种情况下,对向量进行排序的标准成语是`your_vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::{Less|Equal|Greater}));`这取决于你的需要.\`
-   您可以考虑实现表示扑克指针的类型.`PartialOrd`.
