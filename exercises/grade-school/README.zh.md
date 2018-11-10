# 小学

根据学生的姓名以及他们所处的成绩,为学校创建一个名册.

最后,你应该能够:

-   将学生的姓名添加到名册中以获得成绩
    -   "把吉姆加到2年级."
    -   "好."
-   获取所有注册成绩的学生的列表
    -   "哪个学生在二年级?"
    -   "我们刚才才有吉姆."
-   获取所有年级所有学生的排序列表.成绩应分为1,2,3等,成绩中的学生应按名称按字母顺序排序.
    -   "谁现在都在学校就读?"
    -   "一年级:安娜,巴伯和查理.二年级:亚历克斯,彼得和佐伊.三年级......"

请注意,我们所有学生只有一个名字.(这是一个小镇,你想要什么?)

## 奖励积分

你是否通过了测试并且代码干净了?如果您愿意,可以尝试以下一些额外的事情:

-   如果您使用的语言具有可变数据结构,并且您的实现允许外部代码直接改变学校的内部数据库,请查看是否可以阻止这种情况.随意介绍其他测试.

那么请在提交的评论中分享您的想法.这个实验是否使代码更好?更差?你从中学到了什么吗?

## 锈蚀安装

参考[运动帮助页面][help-page]用于Rust安装和学习资源.

## 编写代码

执行测试:

```bash
$ cargo test
```

除了第一次测试外,所有测试都被忽略了在第一个测试通过后,打开位于的测试源文件`tests`目录并删除`#[ignore]`从下一次测试中标记并再次通过测试.每个单独的测试都是一个函数`#[test]`它上面的旗帜.继续,直到通过每个测试.

如果您希望在不编辑测试源文件的情况下运行所有​​测试,请使用:

```bash
$ cargo test -- --ignored
```

例如,运行特定测试`some_test`, 您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试,请使用:

```bash
$ cargo test some_test -- --ignored
```

要了解有关Rust测试的更多信息,请参阅[在线测试文档][rust-tests]

一定要阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)章节如果你还没有,它将帮助你组织你的文件.

## 反馈,问题,请求

该[exercism/rust](https://github.com/exercism/rust)GitHub上的存储库是所有Rust练习的主页.如果您有关于练习的反馈,或者想要帮助实施新的练习,请前往那里并创建一个问题.铁轨团队成员很乐意为您提供帮助!

如果你想了解更多关于运动的知识,请看看[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md).

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 资源

与gSchool的Phil Battos进行配对<http://gschool.it>

## 提交不完整的解决方案

可以提交不完整的解决方案,以便您了解其他人如何完成练习.
