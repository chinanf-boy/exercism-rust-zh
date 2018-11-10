# 电话号码

清理用户输入的电话号码,以便他们可以发送短信.

这个**北美编号计划(NANP)**是北美洲、加拿大或百慕大群岛等许多国家使用的电话号码系统.所有NANP国家共享相同的国际国家代码:`1`.

NANP数字是十位数字,由三位编号计划区域代码组成,俗称*地区代码*其次是一个七位数的本地号码.本地数字的前三位数字表示*交换码*其次是唯一的四位数字,这是*用户号码*.

格式通常表示为

```text
(NXX)-NXX-XXXX
```

在哪里?`N`是从2到9的任何数字`X`是从0到9的任何数字.

您的任务是通过删除标点符号和国家代码(1)来清理不同格式的电话号码.

例如,输入

-   `+1 (613)-995-0253`
-   `613-995-0253`
-   `1 613 995 0253`
-   `613.995.0253`

都应该产出

`6139950253`

**注:**因为这个练习只涉及NANP国家使用的电话号码,只有1被认为是有效的国家代码.

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

## 来源

事件管理器<http://tutorials.jumpstartlab.com/projects/eventmanager.html>

## 提交不完全解

有可能提交一个不完整的解决方案,这样你就可以看到其他人是如何完成练习的.
