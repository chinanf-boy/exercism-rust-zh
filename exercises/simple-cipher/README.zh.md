# 简单密码

实现一个简单的移位密码,像 Caesar 和，一个更安全的替换密码.

## 步骤 1

"如果他有什么秘密要说的话，他就是用密码写的，也就是说，通过改变字母表的字母顺序，一个字也说不出来。如果有人想破译这些，并理解它们的意思，他必须用字母表中的第四个字母，即 D，代替 A，以及其他的字母。
—Suetonius, Life of Julius Caesar

密码是非常直截了当的算法，使文本不可读，同时仍容易允许破译。他们容易受到许多形式的密码分析，但我们幸运的是，我们的小姐妹通常不是密码学家。

用 Caesar密码，加密来自 Julius Caesar 的消息。现在 Caesar 知道加密方式不是很好，但他还是有个好处: 几乎没有人能读对。所以对于一对夫妇的信件已经足够了，让人们无法识别他们所知道的几个字。

你的任务是创建一个简单的移位密码，就像 Caesar 密码一样。这个图像是 Caesar 密码的一个很好的例子:

![Caesar Cipher][1]

例如:

将"iamapandabear"作为输入到 `encode` 函数返回密码"ldpdsdqgdehdu"。(虽不足以让我们的信息在运输过程中，确保高保密性).

当将"ldpdsdqgdehdu"放入`decode`函数时，它将返回原始的"iamapandabear"，让您的朋友阅读您的原始消息.

## 步骤 2

移位密码是没有乐趣，直到你的妹妹算了出来后！尝试修改代码,允许我们指定一个密钥(key)，并用作移位距离。这称为代换密码.

下面是一个例子:

给定密钥"aaaaaaaaaaaaaaaaaaa"，对字符串"iamapandabar"进行编码将返回原来的"iamapandable".

给定密钥"ddddddddddddd"，编码我们的字符串"iamapandabore"会返回疑惑的"ldpdsdqgdhdu".

在上面的示例中,我们为键值设置了 `a=0`。因此,当明文添加到密钥时,我们最终得到相同的消息。所以"aaaa"不是一个理想的密钥。但是,如果我们把密钥设置为`dddd`,我们将得到与 caesar 密码相同的东西.

## 步骤 3

任何密码中最薄弱的环节都是人。让我们通过提供随机性，并确保密钥仅包含小写字母，来使替换密码更具容错性.

如果有人根本不提交密钥,则生成一个长度至少为 100 个字符的真正随机密钥.

如果提交的密钥不只由小写字母组成,那解决方案应该以适当提醒的方式处理错误.

## 扩展

移位密码通过使文本略微疑惑，达到目的，但易受频率分析的影响。替换密码有助于这一点,但当密钥较短，或空格被保留时,仍然非常脆弱。稍后,你会看到一个解决这个问题的练习"crypto-square".

如果你想在这一领域走得更远，问题引导你，走进如何以安全的方式交换密钥。看一看[维基百科上的 Diffie Hellman][dh]对于该方案的最早实现之一.

[1]: https://upload.wikimedia.org/wikipedia/commons/thumb/4/4a/Caesar_cipher_left_shift_of_3.svg/320px-Caesar_cipher_left_shift_of_3.svg.png
[dh]: http://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange
[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

维基百科的替代密码<http://en.wikipedia.org/wiki/Substitution_cipher>
