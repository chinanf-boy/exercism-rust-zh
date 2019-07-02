# 钻石

钻石 kata 将一个字母作为输入，并以菱形输出。给定一个字母，它会打印一个以'A'开头的钻石，并在最宽处提供所提供的字母。

## 要求

- 第一行包含一个'A'.
- 最后一行包含一个'A'.
- 除第一行和最后一行之外的所有行都有两个完全相同的字母.
- 所有行都具有与前导空格一样多的尾随空格.(这可能是 0).
- 钻石是水平对称的.
- 钻石是垂直对称的.
- 钻石具有方形(宽度等于高度).
- 字母形成菱形.
- 上半部分的字母按升序排列.
- 下半部分的字母按降序排列.
- 四个角(包含空格)是三角形.

## 例子

在以下示例中，空格表示为`·`字符.

字母'A'的钻石:

```text
A
```

字母'C'的钻石:

```text
··A··
·B·B·
C···C
·B·B·
··A··
```

字母'E'的钻石:

```text
····A····
···B·B···
··C···C··
·D·····D·
E·······E
·D·····D·
··C···C··
···B·B···
····A····
```

## 资源

Seb Rose<http://claysnow.co.uk/recycling-tests-in-tdd/>

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
