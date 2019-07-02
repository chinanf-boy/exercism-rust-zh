# 篱笆密码法(栅栏密码)

实现篱笆密码法的编/解码.

篱笆密码法是一种置换式密码,它从编码的方式得到它的名字。它早被古希腊人所使用。

在 栅栏 密码中,信息向下写在虚构的篱笆的连续"rail"上，然后当我们到达底部时，又向上移动(像锯齿形)。最后,消息以行读取.

例如,使用三个"Rails(栅栏)"，和加密"WE ARE DISCOVERED FLEE AT ONCE"信息，密文写道:

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
```

然后读出:

```text
WECRLTEERDSOEEFEAOCAIVDEN
```

要解密一条消息,你要采用锯齿形读法，而密文则是一行一行看。

```text
? . . . ? . . . ? . . . ? . . . ? . . . ? . . . ?
. ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

第一行有七个点,可以用"WECRRLTE"填充.

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? . ? .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

现在第二行为"ERDSOEEFEAOC".

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

最后一排"AIVDEN".

```text
W(1) . . . E . . . C . . . R . . . L . . . T . . . E
. E(2) . R . D . S . O . E . E . F . E . A . O . C .
. . A(3) . . . I . . . V . . . D . . . E . . . N . .
```

> `1,2,3`只是为了方便理解，不存在任何地方

如果你现在锯齿形阅读,你可以阅读原始消息.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Transposition_cipher#Rail_Fence_cipher>

百度百科
<https://baike.baidu.com/item/%E6%A0%85%E6%A0%8F%E5%AF%86%E7%A0%81>