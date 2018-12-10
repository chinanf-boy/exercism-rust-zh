# 罗马数字

写一个函数,从普通数字，转换成罗马数字.

罗马人是一群聪明的人。他们征服了欧洲大部分国家,统治了几百年。他们发明了混凝土和直路,甚至 Bikinis 夜店。他们从来没有发现过的一件事就是数字零。这使得写作和约会他们的功绩的广泛历史稍有挑战性，但他们提出的数字系统仍在使用。例如,英国广播公司使用罗马数字制定他们的节目。

罗马人用字母 I（1）、V（5）、X（10）、L（50）、C（100）、D（500）和 M（1000） 写数字(注意这些字母有很多直线,因此很容易侵入石碑)。

```text
 1  => I
10  => X
 7  => VII
```

> 不需要能 转换 超过 3000 的罗马数字

在较大的罗马数字的右边记上较小的罗马数字，表示大数字加小数字。
在较大的罗马数字的左边记上较小的罗马数字，表示大数字减小数字。

要在实践中看到这一点,请考虑 1990 的例子.

在罗马数字中,1990 是 MCMXC:

1000=M 900=CM 90=XC

> CM = 1000 - 100 = 900

2008 被写成 MMVIII:

2000=MM 8=Ⅷ

参见:<http://www.novaroma.org/via_romana/numbers.html>

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

维基百科<https://zh.wikipedia.org/wiki/%E7%BD%97%E9%A9%AC%E6%95%B0%E5%AD%97>
