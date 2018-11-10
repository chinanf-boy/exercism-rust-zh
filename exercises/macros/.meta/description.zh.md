宏是锈蚀程序员工具箱中的一个强大部分,[macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)是一种相对简单的方法来访问这个电源.让我们写一个!

## 语境

什么是宏?[Wikipedia](https://en.wikipedia.org/wiki/Macro_(computer_science))这样描述:

> 在计算机科学中,宏(希腊语μακρ'long’的缩写)是指根据定义的,规定某个输入序列(通常是字符序列)应该如何映射到替换输出序列(也通常是字符序列)的规则或模式.程序.实例化(转换)宏使用到特定序列的映射过程称为宏扩展.

照亮!但更具体的说,宏是一种特殊的语法,允许您在编译时生成代码.宏可以用于编译时计算,但更多的是,它们只是抽象代码的另一种方式.例如,您可能已经使用过`println!()`和`vec![]`. 它们每个都取任意数量的参数,所以不能用简单函数表示它们.另一方面,它们总是扩展到一定数量的绝对标准锈代码.如果你感兴趣,你可以使用[cargo expand](https://github.com/dtolnay/cargo-expand)子命令以查看代码中宏扩展的结果.

有关锈蚀中的宏的进一步信息,锈书有[good chapter](https://doc.rust-lang.org/book/2018-edition/appendix-04-macros.html)在他们身上.

## 问题陈述

你可以生产一个`Vec`任意长度的内联`vec![]`宏.然而,Rust并没有产生一种方法.[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)内联.通过写一个`hashmap!()`宏.

例如,您的库的用户可能会编写`hashmap!('a' => 3, 'b' => 11, 'z' => 32)`. 这应该扩展到以下代码:

```rust
{
   let mut hm = HashMap::new();
   hm.insert('a', 3);
   hm.insert('b', 11);
   hm.insert('z', 32);
   hm
}
```

请注意[`maplit` crate](https://crates.io/crates/maplit)提供了一个很好地解决这个问题的宏.请执行您自己的解决方案,而不是使用这个板条箱,请在查看其源之前尝试自己.
