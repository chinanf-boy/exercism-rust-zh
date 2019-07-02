# 迪菲-赫尔曼

迪菲-赫尔曼密钥交换

Alice 和 Bob 使用 迪菲-赫尔曼密钥来共享秘密。它们以素数开头,选择私钥,生成和共享公钥,然后生成共享密钥.

## 第 0 步

测试程序提供素数 `p 和 g`.

## 步骤 1

Alice 选择一个大于 1 ，且小于 p 的私钥。鲍勃做同样的事情来选择私钥 b.

## 第 2 步

Alice 计算公钥 A.

```
A = g**a mod p
```

使用相同的 p 和 g, Bob 类似地从他的私钥 b 计算公钥 B.

## 第 3 步

Alice 和 Bob 交换公钥.Alice 计算密钥 s.

```
s = B**a mod p
```

鲍勃计算

```
s = A**b mod p
```

计算产生相同的结果! 爱丽丝和鲍勃现在分享秘密.

本练习的一种可能解决方案是实现您自己的模幂运算函数。要了解更多信息,请参阅[following page](https://en.wikipedia.org/wiki/Modular_exponentiation).

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

- 维基百科,来自 www.cryptopp.com/wiki 的 1024 位密钥.<http://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange>

- [中文维基](https://zh.wikipedia.org/wiki/%E8%BF%AA%E8%8F%B2-%E8%B5%AB%E7%88%BE%E6%9B%BC%E5%AF%86%E9%91%B0%E4%BA%A4%E6%8F%9B)