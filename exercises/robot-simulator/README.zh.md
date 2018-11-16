# 机器人模拟器

编写机器人模拟器.

机器人工厂的测试设施需要一个程序来验证机器人的运动.

机器人有三种可能的运动:

-   右转
-   向左拐
-   提前

机器人被放置在一个假设的无限网格上,以一组{x,y}坐标,例如{3,8}面向特定方向(北、东、南或西),坐标向北和东增加.

然后,机器人接收许多指令,测试设备验证机器人的新位置以及指向哪个方向.

-   字母串"Raalar"的意思是:
    -   右转
    -   提前两次
    -   向左拐
    -   提前一次
    -   再次左转
-   假设一个机器人从{ 7, 3 }向北开始.然后运行这个指令流应该把它放在面向西方的{ 9, 4 }上.

## 锈蚀装置

参考[练习帮助页面][help-page]用于锈蚀安装和学习资源.

## 编写代码

用下列方法执行测试:

```bash
$ cargo test
```

但第一个测试都被忽视了.`tests`你得到第一个测试通过后,打开测试源文件位于`#[ignore]`国旗下再次测试,使测试通过.`#[test]`每一个单独的测试是一个函数

如果你想运行所有测试没有编辑测试源文件,使用:

```bash
$ cargo test -- --ignored
```

例如,运行一个特定的测试`some_test`您可以使用:

```bash
$ cargo test some_test
```

如果忽略特定测试使用:

```bash
$ cargo test some_test -- --ignored
```

了解更多关于生锈测试参考[在线测试文档][rust-tests]

确保阅读[Modules](https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html)如果你还没有章,它将帮助你组织你的文件.

## 反馈,问题,把请求

的[exercism/rust](https://github.com/exercism/rust)在GitHub库所有生锈的练习.

如果你有反馈一个练习,或者想要帮助实现新练习,那边的负责人并创建一个问题.[contribution guide](https://github.com/exercism/docs/blob/master/contributing-to-language-tracks/README.md)锈田径队的成员都乐于帮助!

[help-page]: https://exercism.io/tracks/rust/learning

[modules]: https://doc.rust-lang.org/book/2018-edition/ch07-00-modules.html

[cargo]: https://doc.rust-lang.org/book/2018-edition/ch14-00-more-about-cargo.html

[rust-tests]: https://doc.rust-lang.org/book/2018-edition/ch11-02-running-tests.html

## 源

灵感来自一个著名公司的面试问题.

## 提交完整的解决方案

可以提交一个不完整的解决方案,这样你就可以看到其他人已经完成了锻炼.
