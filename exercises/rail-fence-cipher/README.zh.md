# 式密码

实现栅栏密码的编解码.

栅栏密码是转位密码的一种形式,它从编码的方式得到它的名字.它已经被古希腊人使用了.

在 Rail Fence 密码中,信息向下写在虚构的篱笆的连续"rail"上,然后当我们到达底部(像锯齿形)时向上移动.最后,消息以行读取.

例如,使用三个"Rails"和"我们立刻被发现逃跑"的信息,密文写道:

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
```

然后读出:

```text
WECRLTEERDSOEEFEAOCAIVDEN
```

要解密一条消息,你要采用锯齿形并沿着行填充密文.

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

现在第二行取名为"ErdSoEfEaoc".

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . ? . . . ? . . . ? . . . ? . . . ? . . . ? . .
```

最后一排离开"Avi 登".

```text
W . . . E . . . C . . . R . . . L . . . T . . . E
. E . R . D . S . O . E . E . F . E . A . O . C .
. . A . . . I . . . V . . . D . . . E . . . N . .
```

如果你现在阅读曲折形状,你可以阅读原始消息.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<https://en.wikipedia.org/wiki/Transposition_cipher#Rail_Fence_cipher>
