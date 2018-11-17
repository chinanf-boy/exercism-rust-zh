# 简单链表

编写一个使用元素和列表的简单链表实现.

链表是计算机科学中的一种基本数据结构,常用于其他数据结构的实现.它们在函数式编程语言(如 Clojure、Erlang 或 Haskell)中很普遍,但是在命令式语言(如 Ruby 或 Python)中很少见.

最简单的链表是单链表.列表中的每个元素包含数据和指向元素列表中的下一个元素的"下一个"字段.

链接列表的这种变体通常用于表示序列或下推堆栈(也称为 LIFO 堆栈;.In, .Out).

作为第一步,让我们创建一个包含范围(1..10)的单一链接列表,并提供函数来反转链接列表并转换为数组或从数组转换为数组.

在使用内置链表的语言实现这一点时,实现自己的抽象数据类型.

## 实现提示

不要实现结构`SimpleLinkedList`作为一个包装`Vec`. 相反,在堆上分配节点.\
这可以实现为:

pub struct SimpleLinkedList<T> {
head: Option<Box<Node<T>>>,
}

这个`head`字段指向该链表的第一个元素(节点).\
这个实现也需要一个结构`Node`具有以下字段:

struct Node<T> {
data: T,
next: Option<Box<Node<T>>>,
}

`data`包含存储的数据,以及`next`指向以下节点(如果可用)或无.

### 为什么?`Option<Box<Node<T>>>`而不仅仅是`Option<Node<T>>`?

自己试试.您将得到以下错误.

| struct Node<T>
| ^^^^^^^^^^^^^^ recursive type has infinite size
...
| next: Option<Node<T>>,
| --------------------- recursive without indirection

问题是,在编译时,必须知道下一个的大小.自从`next`是递归的("一个节点有一个节点有一个节点…"),编译器不知道要分配多少内存.相反,[Box](https://doc.rust-lang.org/std/boxed/)是一个具有定义大小的堆指针.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

受"Ruby"中面向对象设计模式的数据结构和算法的启发,即单链表.<http://www.brpreiss.com/books/opus8/html/page96.html#SECTION004300000000000000000>
