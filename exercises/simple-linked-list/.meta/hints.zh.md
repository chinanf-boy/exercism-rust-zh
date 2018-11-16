## 实施提示

不要实现结构`SimpleLinkedList`作为一个包装`Vec`.而是在堆上分配节点.\
这可以实现为:

```
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```

该`head`字段指向此链接列表的第一个元素(节点).\
此实现还需要结构`Node`包含以下字段:

```
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```

`data`包含存储的数据,和`next`指向以下节点(如果可用)或None.

### 为什么`Option<Box<Node<T>>>`而不只是`Option<Node<T>>`?

自己尝试一下.您将收到以下错误.

```
| struct Node<T>
| ^^^^^^^^^^^^^^ recursive type has infinite size
...
|     next: Option<Node<T>>,
|     --------------------- recursive without indirection
```

问题是在编译时必须知道next的大小.以来`next`是递归的("一个节点有一个节点有一个节点......"),编译器不知道要分配多少内存.相反,[Box](https://doc.rust-lang.org/std/boxed/)是具有已定义大小的堆指针.
