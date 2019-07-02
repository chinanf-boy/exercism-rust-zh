# 简单链表

编写一个使用元素(`Elements`)和列表(`List`)的简单链表实现.

链表是计算机科学中的一种基本数据结构，常用于其他数据结构的实现。它们在函数式编程语言(如 Clojure、Erlang 或 Haskell)中很普遍，但是在命令式语言(如 Ruby 或 Python)中很少见。

最简单的链表是单链表。列表中的每个元素，包含数据和一个"next"字段，指向元素列表中的下一个元素。

链接列表的这种变体通常用于表示序列，或推/取堆栈(也称为 LIFO 堆栈;后进先出)。

作为第一步,让我们创建一个包含范围(1..10)的单一链接列表,并提供反转链接列表，和转换为数组，或从数组转换的函数.

在使用内置链表的语言实现这一点时,实现自己的抽象数据类型.

## 实现提示

不要实现结构`SimpleLinkedList`作为一个`Vec`包装。 你应该,在堆上分配节点。
这可以实现为:

```rust,no_run
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}
```

这个`head`字段指向该链表的第一个元素(节点)。
这个实现也需要一个`Node`结构，其具有以下字段:

```rust,no_run
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```

`data`包含存储的数据,以及`next`指向下节点(如果可用)或无.

### 为什么使用`Option<Box<Node<T>>>`?而不仅用`Option<Node<T>>`?

自己试试.您将得到以下错误.

```
| struct Node<T>
| ^^^^^^^^^^^^^^ recursive type has infinite size
...
| next: Option<Node<T>>,
| --------------------- recursive without indirection
```

这个错误在于,编译时,rust 必须知道下一个节点的大小。因`next`会是递归的("一个节点，有一个节点-有一个节点…")，编译器不知道要分配多少内存。相反,[Box](https://doc.rust-lang.org/std/boxed/)是一个具有定义大小的堆指针.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

受"Ruby"中面向对象设计模式的数据结构和算法的启发,即单链表.<http://www.brpreiss.com/books/opus8/html/page96.html#SECTION004300000000000000000>
