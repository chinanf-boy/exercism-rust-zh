# 机器人模拟器

编写机器人模拟器。

机器人工厂的测试设施需要一个程序，来验证机器人的运动。

机器人有三种可能的运动:

- 右转
- 左转
- 前进

机器人被放置在一个假设的无限网格上，以一组{x，y}坐标，例如{3，8}面向特定方向(北、东、南或西)，能向北和东前进。

然后，机器人接收许多指令，测试设备验证机器人的新位置以及指向哪个方向。

- 字母串"RAALAR"的意思是:
  - 右转
  - 前两次
  - 向左拐
  - 前一次
  - 再次左转
- 假设一个机器人从{ 7， 3 }向北开始，然后运行这个指令流，它应该就放在面向西方的{ 9, 4 }上。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html

## 源

灵感来自一个著名公司的面试问题.
