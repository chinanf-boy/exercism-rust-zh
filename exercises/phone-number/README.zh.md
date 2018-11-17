# 电话号码

清理用户输入的电话号码,以便他们可以发送短信.

这个**北美编号计划(NANP)**是北美洲、加拿大或百慕大群岛等许多国家使用的电话号码系统.所有 NANP 国家共享相同的国际国家代码:`1`.

NANP 数字是十位数字,由三位编号计划区域代码组成,俗称*地区代码*其次是一个七位数的本地号码.本地数字的前三位数字表示*交换码*其次是唯一的四位数字,这是*用户号码*.

格式通常表示为

```text
(NXX)-NXX-XXXX
```

在哪里?`N`是从 2 到 9 的任何数字`X`是从 0 到 9 的任何数字.

您的任务是通过删除标点符号和国家代码(1)来清理不同格式的电话号码.

例如,输入

- `+1 (613)-995-0253`
- `613-995-0253`
- `1 613 995 0253`
- `613.995.0253`

都应该产出

`6139950253`

**注:**因为这个练习只涉及 NANP 国家使用的电话号码,只有 1 被认为是有效的国家代码.

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

事件管理器<http://tutorials.jumpstartlab.com/projects/eventmanager.html>
