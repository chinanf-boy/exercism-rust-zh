# 罗马数字

写一个函数,从普通数字转换成罗马数字.

罗马人是一群聪明的人.他们征服了欧洲大部分国家,统治了几百年.他们发明了混凝土和直路,甚至Bikinis夜店.他们从来没有发现过的一件事就是数字零.这使得写作和约会他们的功绩的广泛历史稍有挑战性,但他们提出的数字系统仍在使用.例如,英国广播公司使用罗马数字来和他们的节目约会.

罗马人用字母I、V、X、L、C、D、M.写数字(注意这些字母有很多直线,因此很容易侵入石碑).

```text
 1  => I
10  => X
 7  => VII
```

不需要能够转换大于大约3000的数字.(罗马人自己不想走得更高)

维基百科说:现代罗马数字…通过分别用最左数字表示每个数字并跳过任何值为零的数字来编写.

要在实践中看到这一点,请考虑1990的例子.

在罗马数字中,1990是MCMXC:

1000=M 900=CM 90=XC

2008被写成MMVIII:

2000=mm 8=Ⅷ

参见:<http://www.novaroma.org/via_romana/numbers.html>

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

罗马数字卡塔<http://codingdojo.org/cgi-bin/index.pl?KataRomanNumerals>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
