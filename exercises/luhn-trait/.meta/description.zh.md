# Luhn:使用自定义特征

在做这个练习之前,你应该做原始的LuHn练习和它的继任者,"Luhn:使用来自特征".

要获得原始的LuHn练习,运行

```shell
exercism download --exercise=luhn --track=rust


为了获得"Lunn:使用来自特征"的练习,运行

```shell
exercism download --exercise=luhn-from --track=rust


在原始的LuHn练习中,您只验证字符串,但LuHN算法也可以应用于整数.

在"Lurn:使用from特性"中,你实现了一个从特性,这也需要你创建一个LuHn结构.

而不是创建一个结构来执行验证,如果您自己验证了原语(即,字符串,U8等)呢?

在本练习中,您将创建并实现自定义[trait](https://doc.rust-lang.org/book/ch10-02-traits.html)执行验证.

注:它是[not idiomatic Rust to implement traits on on primitives](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type). 在这个练习中,我们展示了一些东西*可以*做,而不是你*应该*做.如果你发现自己在图元上实现了特性,也许你有一个例子.[Primitive Obsession](http://wiki.c2.com/?PrimitiveObsession).
