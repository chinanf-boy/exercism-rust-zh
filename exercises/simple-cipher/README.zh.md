# 简单密码

实现一个简单的移位密码,像凯撒和一个更安全的替换密码.

## 步骤1

"如果他有什么秘密要说的话,他就是用密码写的,也就是说,通过改变字母表的字母顺序,一个字也说不出来.如果有人想破译这些并理解它们的意思,他必须用字母表中的第四个字母,即D,代替A,以及其他的字母.

密码是非常直截了当的算法,使我们能够使文本不可读,同时仍然允许容易破译.他们容易受到许多形式的密码分析,但我们幸运的是,我们的小姐妹通常不是密码分析家.

凯撒密码被用来发送来自Julius Caesar的消息.现在罗楼迦知道密码不是很好,但他在这方面有一个盟友:几乎没有人能读得很好.所以即使是一对夫妇的信件是足够的,使人们无法识别他们所知道的几个字.

你的任务是创建一个简单的移位密码,就像凯撒密码一样.这个图像是凯撒密码的一个很好的例子:

![Caesar Cipher][1]

例如:

将"IAMAPANDABLE"作为输入到EnCODE函数返回密码"LDPDSDQGDEHDU".不足以让我们的信息在运输过程中保密.

当将"ldpdsdqgdehdu"放入解码函数时,它将返回原始的"iamapandabear",让您的朋友阅读您的原始消息.

## 步骤2

移位密码是没有乐趣,虽然当你的妹妹算了出来.尝试修改代码,允许我们指定一个键,并使用该移位距离.这称为代换密码.

下面是一个例子:

给定密钥"AAAAAAAAAAAAAAAAAAA",对字符串"IAMAPANDABAR"进行编码将返回原来的"IAMAPANDABLE".

给定密钥"DDDDDDDDDDDDD",编码我们的字符串"IAMAPANDABORE"会返回模糊的"LDPDSDQGDHDU".

在上面的示例中,我们为键值设置了a=0.因此,当明文添加到密钥时,我们最终得到相同的消息.所以"AAAA"不是一个理想的关键.但是,如果我们把密钥设置为DDDD,我们将得到与凯撒密码相同的东西.

## 步骤3

任何密码中最薄弱的环节都是人.让我们通过提供随机性来源并确保密钥仅包含小写字母来使替换密码更具容错性.

如果有人根本不提交密钥,则生成一个长度至少为100个字符的真正随机密钥.

如果提交的密钥不只由小写字母组成,则解决方案应该以适合语言的方式处理错误.

## 扩展

移位密码通过使文本略微奇特而工作,但易受频率分析的影响.替换密码有助于这一点,但当密钥较短或空间被保留时,仍然非常脆弱.稍后,你会看到一个解决这个问题的练习"密码广场".

如果你想在这一领域走得更远,问题就开始于我们如何以安全的方式交换密钥.看一看[维基百科上的Diffie Hellman][dh]对于该方案的第一个实现之一.

[1]: https://upload.wikimedia.org/wikipedia/commons/thumb/4/4a/Caesar_cipher_left_shift_of_3.svg/320px-Caesar_cipher_left_shift_of_3.svg.png

[dh]: http://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange

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

维基百科的替代密码<http://en.wikipedia.org/wiki/Substitution_cipher>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
