# 让我们开始了解些东东

这些练习依赖于测试驱动开发(TDD),但它们不是一个精确的匹配测试.

以下步骤，假设您处于与练习相同的目录中.

你必须安装rust.跟随[Rust的安装步骤](https://doc.rust-lang.org/book/ch01-01-installation.html). 这个Exercism带来的[Rust 语言 部分](http://exercism.io/languages/rust)也是有用的.

## 步骤1

运行测试套件.它可以运行`cargo`装锈的.

```
$ cargo test
```

这将编译`hello-world`装箱，并运行测试,但失败了.

```
running 1 test
test test_hello_world ... FAILED

failures:

---- test_hello_world stdout ----
thread 'test_hello_world' panicked at 'assertion failed: `(left == right)`
(left: `"Hello, World!"`, right: `"Goodbye, World!"`)', tests/hello-world.rs:5

failures:
    test_hello_world

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured
```

### 理解测试失败

这个`test_hello_world`失败表示它正在期望`"Hello, World!"`值，这个值由`hello()`返回。 断言的`left`(在第5行)应该等于`right`.

```
---- test_hello_world stdout ----
thread 'test_hello_world' panicked at 'assertion failed: `(left == right)`
(left: `"Hello, World!"`, right: `"Goodbye, World!"`)', tests/hello-world.rs:5
```

### 修正误差

修复它,先打开`src/lib.rs`，并且改变`hello`函数，改为返回`"Hello, World!"`而不是`"Goodbye, World!"`.

```rust
pub fn hello() -> &'static str {
    "Hello, World!"
}
```

## 步骤2

再次运行测试.这一次,它将通过.

```
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/hello_world-bd1f06dc726ef14f

running 1 test
test test_hello_world ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests hello-world

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```

## 提交

一旦测试通过,您可以用，以下命令提交代码:

```
$ exercism submit src/lib.rs
```
