# 宏指令

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

## 锈蚀装置

参考[练习帮助页面][help-page]用于锈蚀安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有的测试都被忽略了.在获得第一个测试通过后,打开位于`tests`目录和删除`#[ignore]`从下一个测试标志,并使测试再次通过.每个单独的测试都是一个函数.`#[test]`旗帜在上面.继续,直到你通过每一个测试.

如果希望在不编辑测试源文件的情况下运行所有测试,请使用:

```bash
$ cargo test -- --ignored
```

运行特定的测试,例如`some_test`,您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,则使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关锈蚀试验的更多信息,请参阅[在线测试文档][rust-tests]

请务必阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)如果你还没有,它将帮助你组织你的文件.

## 反馈、问题、拉动请求

这个[exercism/rust](https://github.com/exercism/rust)在Github上的储存库是所有锈蚀演习的所在地.如果你有关于锻炼的反馈,或者想帮助实施新的锻炼,就去那里创造一个问题.铁锈队的队员们很乐意帮忙!

如果你想了解更多关于运动的知识,请看一下[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

彼得古德斯特尼克劳斯

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
