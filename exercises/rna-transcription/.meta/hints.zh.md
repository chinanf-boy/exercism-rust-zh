## Rust 实施注意事项

在公共结构中使用私有字段`new`功能返回`Option`或`Result`(和这里一样)`DNA::new` & `RNA::new`),我们可以保证内部表示`DNA`是正确的.因为每一个有效的 DNA 串都有一个有效的 RNA 串,所以我们不需要返回一个`Result`/`Option`从`to_rna`.

这将解释您将在测试中看到的类型签名.
