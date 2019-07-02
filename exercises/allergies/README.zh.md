# 过敏

给出一个人的过敏分，确定他们是否对某一物品过敏，以及他们的过敏列表.

过敏测试产生单个数字分数，其中包含有关该人所有过敏的信息(他们进行了测试)。

测试的物品列表(及其值)为:

- 鸡蛋(1)
- 花生(2)
- 贝类(4)
- 草莓(8)
- 西红柿(16)
- 巧克力(32)
- 花粉(64)
- 猫(128)

因此，如果汤姆对花生和巧克力过敏，他会得到 34 分.

现在，只要得到 34 分，你的程序应该说:

- 汤姆是否对上面列出的任何一种过敏原过敏.
- 汤姆所有过敏原。

注意：给出的分数可能**不**包括上面列出的过敏原(即分数为 256，512，1024 等的过敏原)。您的程序应忽略这些组成部分。例如，如果过敏分数是 257，您的程序应该只报告鸡蛋(1)过敏。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 资源

Jumpstart 实验室热身<http://jumpstartlab.com>
