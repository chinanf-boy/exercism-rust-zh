## 改为使用Cargo test

正如万千生成工具，总是具有`主题/css样式/模版`之类的概念

[mdBook](https://github.com/rust-lang-nursery/mdBook) 也不例外

我们为了，让这份练习可在网页上运行并知道结果，借用了mdBook工具，它本身就具有`Rust Code`html元素运行的功能

可惜，也就相当于`Cargo run`,而我们要使用的是`Cargo test`。下面进入到我们的修改之旅

## 目录

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [基础，给第一次使用者](#%E5%9F%BA%E7%A1%80%E7%BB%99%E7%AC%AC%E4%B8%80%E6%AC%A1%E4%BD%BF%E7%94%A8%E8%80%85)
- [入主题，使用过mdBook](#%E5%85%A5%E4%B8%BB%E9%A2%98%E4%BD%BF%E7%94%A8%E8%BF%87mdbook)
  - [book.js](#bookjs)
    - [使用Rust游乐场的Cargo test](#%E4%BD%BF%E7%94%A8rust%E6%B8%B8%E4%B9%90%E5%9C%BA%E7%9A%84cargo-test)
    - [合并(用户/答案代码)与测试代码](#%E5%90%88%E5%B9%B6%E7%94%A8%E6%88%B7%E7%AD%94%E6%A1%88%E4%BB%A3%E7%A0%81%E4%B8%8E%E6%B5%8B%E8%AF%95%E4%BB%A3%E7%A0%81)
- [至此，完成。谢谢浏览！](#%E8%87%B3%E6%AD%A4%E5%AE%8C%E6%88%90%E8%B0%A2%E8%B0%A2%E6%B5%8F%E8%A7%88)
  - [相关](#%E7%9B%B8%E5%85%B3)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## 基础，给第一次使用者

> 安装就不讲了

- 初始化mdBook项目是这样的

``` bash
mdBook init --theme
```

- 它的主题之类，放在`src/theme`,结构是这样的

``` bash
- theme
    - book.js # 默认的js改造
    - favicon.png
    - highlight.css
    - highlight.js
    - index.hbs # 默认的模版，引擎是hbs
    - css
        - chrome.css
        - general.css
        - print.css
        - variables.css
```

主要的，代码修改，落在了`book.js`中，它本身就是给`rust代码 元素`，(重点是)加上可与[rust游乐场](https://play.rust-lang.org/)联系和运行代码，并返回结果的 **播放按钮**。

## 入主题，使用过mdBook

### book.js

> 这个代码文件本身有600多行，着重在使用`Cargo test`的部分

#### 使用Rust游乐场的Cargo test

- 原代码

``` js
// ...
        var params = {
            version: "stable",
            optimize: "0",
            code: text
        };

        if (text.indexOf("#![feature") !== -1) {
            params.version = "nightly";
        }

        result_block.innerText = "Running...";
        // TODO cargo test
        fetch_with_timeout("https://play.rust-lang.org/evaluate.json", {
            headers: {
                'Content-Type': "application/json",
            },
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        })
```

- 修改

1. 修改 POST的网址
2. 修改 POST的body

``` js
        var params = {
//           version: "stable",
//           optimize: "0",
            code: text,
            channel: "stable",
            "mode":"debug",
            "backtrace":false,
            tests: true,
            crateType: "lib"
        };

        if (text.indexOf("#![feature") !== -1) {
            params.version = "nightly";
        }

        result_block.innerText = "Running...";
        // TODO cargo test
//       fetch_with_timeout("https://play.rust-lang.org/evaluate.json", {
        fetch_with_timeout("https://play.rust-lang.org/execute", {
            headers: {
                'Content-Type': "application/json",
            },
            
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        })
```

> 这些是怎么知道，

主要是通过游乐场，点击Test按钮后的调试器的流量检测。

#### 合并(用户/答案代码)与测试代码

1. mdBook会为哪些没有写入`fn main//...`的Rust代码，写上类似下面`+`加号的代码

``` md
+ #![allow(unused_variables)] 
+ fn main() {
#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello());
}

+ }
```

那好，我们就在`book.js`中，搞个去掉默认的函数

- 全局函数:移除默认的`main`

``` js
function remove_default_main(text){
    var tArr = text.split("\n")
    if(tArr.slice(0,3).some(t =>t.trim() == "#![allow(unused_variables)]")){
        return tArr.slice(3,-1).join("\n")
    }
    return text

}
```

2. 测试代码与其他(用户/答案代码),最大的不同就是`#[test]`

所有，我们为测试代码，找一个全局变量`document`下的安身之所

> 为`document.mdBookTextCode`，book.js本身也为每个代码元素添加按钮，那正好用来检测哪个代码元素是测试代码

``` js
    // Process playpen code blocks
    Array.from(document.querySelectorAll(".playpen")).forEach(function (pre_block) {
        // 添加测试代码，到全局变量document
        var codewithtest = playpen_text(pre_block)

        if(codewithtest.includes("#[test")){ // 测试代码，简单检验

            document.mdBookTextCode =  document.mdBookTextCode || remove_default_main(codewithtest)
        }
```

3. 当点击运行代码的播放按钮，我们要适时加上测试代码

> 主要是(用户/答案代码)的播放按钮

``` js
// 播放按钮的点击事件
    function run_rust_code(code_block) {
        var result_block = code_block.querySelector(".result");
        if (!result_block) {
            result_block = document.createElement('code');
            result_block.className = 'result hljs language-bash';

            code_block.append(result_block);
        }

        let text = playpen_text(code_block);
        // 如 不相等和 不包括测试代码，添加测试代码上去
        if(text != document.mdBookTextCode && !text.includes(document.mdBookTextCode)){
            text = text + '\n' + document.mdBookTextCode
        }
// 说来也巧，我们的第一个修改也在这个点击事件，拼在一起看看
        var params = {
            code: text,
            channel: "stable",
            "mode":"debug",
            "backtrace":false,
            tests: true,
            crateType: "lib"
        };

        if (text.indexOf("#![feature") !== -1) {
            params.version = "nightly";
        }

        result_block.innerText = "Running...";
//       fetch_with_timeout("https://play.rust-lang.org/evaluate.json", {
        fetch_with_timeout("https://play.rust-lang.org/execute", {
            headers: {
                'Content-Type': "application/json",
            },
            
            method: 'POST',
            mode: 'cors',
            body: JSON.stringify(params)
        })
```

## 至此，完成。谢谢浏览！

### 相关

[Github库/book.js](https://github.com/chinanf-boy/exercism-rust-zh/blob/master/theme/book.js)

