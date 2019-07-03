> 本库在[exercism/rust 的中文翻译](https://github.com/chinanf-boy/exercism-rust-zh)

exercism 是一个不错的题目网站，但是它的过程并不是很方便，需要下载，构建测试才能知道，你的对错。所以有没有方便点的方式呢？

### 网页练习功能

这项功能是借助[mdBook]工具完成的，在 mdBook 生成的静态页面中，Rust 代码可以直接执行，它的工作原理是 js 底层与 [Rust 游乐场](https://play.rust-lang.org) API 的交互。但是，原有的实现并不是 Rust 测试模式，所以我们要改些事情。(请往下阅读)

[mdbook]: https://github.com/rust-lang-nursery/mdBook

## TODO

我们要改造的关键：

- `theme/book.js` 改造
  - [x] 使用`cargo test`
  - [x] 合并用户输入与测试用例

修改默认主题的`book.js`，如果你对这个修改过程感兴趣，请查阅[使用 Cargo test](./add-test-code.md)

- [x] 自动化测试静态页面的代码执行。

为了对改造代码的有效性，有一定了解，我选择对页面进行自动化测试，我把它放在了[github 上](https://github.com/chinanf-boy/exercism-rust-zh-webdriver)，不过我不建议你自行运行，因为会耗费时间与计算机资源，甚至无法完整通过测试(需要浏览器，打开 88 个网页)。其中借助 webdriver 测试项目，如有相关需求，可以自行了解，它的 API 还是很好用的。

## 说明

每个练习网页主要分为四个部分

- 1. 说明/题目
- 2. 未完成的代码(可编辑，直接运行就好)
- 3. 测试代码(不可编辑)
- 4. 示例答案(不可编辑，直接运行就好)

### 中文翻译列表

每个练习，最初来自英文人群，所以可能在某些说明上，和我国国情并不吻合，如遇到这种情况，完全可以 Issue/PR 该 github 项目，改成符合国情的问题。

测试代码与未完成代码之间，会有重叠的库导入（因我会把他俩合并，扔给 Rust 游乐场），所以需要修正。

还有，每个练习本身，都是完整的 Cargo 项目。其中自然少不了对其他非内置箱子的导入，在这时，我能做的，只是提示你们，某某项目无法通过网页测试(因，Rust 游乐场并没有该箱子)：

- [x] 为中文校对，但有时出题的人，就只是搬维基百科的资料，所以，题目描述可能不是说百分百贴切。
- `>` 为测试代码修正。
- `no` 具有[非内置箱子](https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml)，无法编译成功, 需要非内置的其他箱子，这时，只能说句“抱歉，要本机下载”。

> [Issue me , if you want](https://github.com/chinanf-boy/exercism-rust-zh/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc)

### [易](./low.md)

---

- [x] `>` [hello world](./hello-world/README.zh.md)
- [x] `>` [千兆秒 >< Gigasecond)](./gigasecond/README.zh.md)
- [x] `>` [闰年 >< Leap](./leap/README.zh.md)
- [x] `>` [雨滴声 >< Raindrops](./raindrops/README.zh.md)
- [x] `>` [反转字符串 >< Reverse String](./reverse-string/README.zh.md)
- [x] `>` [第 n 个素数 >< Nth Prime](./nth-prime/README.zh.md)
- [x] `>` [迟钝孩子 >< Bob](./bob/README.zh.md)
- [x] `>` [啤酒之歌 >< Beer Song](./beer-song/README.zh.md)
- [x] `>` [谚语串烧 >< Proverb](./proverb/README.zh.md)
- [x] `>` [平方差 >< Difference Of Squares](./difference-of-squares/README.zh.md)
- [x] `>` [倍数之和 >< Sum Of Multiples](./sum-of-multiples/README.zh.md)
- [x] `>` [谷物 >< Grains](./grains/README.zh.md)
- [x] `>` [勾股数 >< Pythagorean Triplet](./pythagorean-triplet/README.zh.md)
- [x] `>` [素数因子 >< Prime Factors](./prime-factors/README.zh.md)
- [x] `>` [子串 >< Series](./series/README.zh.md)
- [x] `>` [水仙花数 >< Armstrong Numbers](./armstrong-numbers/README.zh.md)
- [x] `>` [3n+1 猜想 >< Collatz Conjecture](./collatz-conjecture/README.zh.md)
- [x] `>` [迪菲-赫尔曼密钥交换 >< Diffie Hellman](./diffie-hellman/README.zh.md)

### [中等](./medium.md)

---

- [x] `>` - [鞍点 >< Saddle Points](./saddle-points/README.zh.md)
- [x] `>` [等值线 >< Isogram](./isogram/README.zh.md)
- [x] `>` [英文说数字 >< Say](./say/README.zh.md)
- [x] `>` [游程编码 >< Run Length Encoding](./run-length-encoding/README.zh.md)
- [x] `>` [图书编号 >< ISBN Verifier](./isbn-verifier/README.zh.md)
- [x] `>` [数字也能分类 >< Perfect Numbers](./perfect-numbers/README.zh.md)
- [x] `>` [时钟 >< Clock](./clock/README.zh.md)
- [x] `no` [DOT DSL](./dot-dsl/README.zh.md)
- [x] `>` [汉明距离 >< Hamming](./hamming/README.zh.md)
- [x] `>` [简单链表 >< Simple Linked List](./simple-linked-list/README.zh.md)
- [x] `>` [杨辉三角形 >< Pascal's Triangle](./pascals-triangle/README.zh.md)
- [x] `>` [字母的分数游戏 >< Scrabble Score](./scrabble-score/README.zh.md)
- [x] `>` [全字母句 >< Pangram](./pangram/README.zh.md)
- [x] `no` [PaaS-IO-报告 >< Paasio](./paasio/README.zh.md)
- [x] `>` [核苷酸计数 >< Nucleotide Count](./nucleotide-count/README.zh.md)
- [x] `>` [模 10 算法 >< Luhn](./luhn/README.zh.md)
- [x] `>` [最大数字子串乘积 >< Largest Series Product](./largest-series-product/README.zh.md)
- [x] `>` [单词计数 >< Word Count](./word-count/README.zh.md)
- [x] `>` [Atbash 加密 >< Atbash Cipher](./atbash-cipher/README.zh.md)
- [x] `>` [密码矩形 >< Crypto Square](./crypto-square/README.zh.md)
- [x] `>` [旋转密码 >< Rotational Cipher](./rotational-cipher/README.zh.md)
- [x] `>` [简单加密 >< Simple Cipher](./simple-cipher/README.zh.md)
- [x] `>` [栅栏密码 >< Rail Fence Cipher](./rail-fence-cipher/README.zh.md)
- [x] `>` [ETL](./etl/README.zh.md)
- [x] `>` [集合操作 >< Accumulate](./accumulate/README.zh.md)
- [x] `>` [术语 >< Acronym](./acronym/README.zh.md)
- [x] `>` [素数筛 >< Sieve](./sieve/README.zh.md)
- [x] `>` [RNA 转录 >< RNA Transcription](./rna-transcription/README.zh.md)
- [x] `>` [三角形](./triangle/README.zh.md)
- [x] `>` [罗马数字 >< Roman Numerals](./roman-numerals/README.zh.md)
- [x] `>` [你所的基本](./all-your-base/README.zh.md)
- [x] `>` [学册](./grade-school/README.zh.md)
- [x] `>` [二分查找](./binary-search/README.zh.md)
- [x] `>` [机器人模拟器](./robot-simulator/README.zh.md)
- [x] `>` [括号配套](./bracket-push/README.zh.md)
- [x] `>` [Luhn From](./luhn-from/README.zh.md)
- [x] `>` [皇后 攻击](./queen-attack/README.zh.md)
- [x] `>` [保龄球](./bowling/README.zh.md)
- [x] `>` [子列表](./sublist/README.zh.md)
- [x] `>` [地球年](./space-age/README.zh.md)
- [x] `>` [Luhn Trait](./luhn-trait/README.zh.md)
- [x] `>` [宏](./macros/README.zh.md)
- [x] `>` [过敏](./allergies/README.zh.md)
- [x] `>` [可变长度数量](./variable-length-quantity/README.zh.md)
- [x] `>` [电话号码](./phone-number/README.zh.md)
- [x] `>` [罗唆](./wordy/README.zh.md)
- [x] `>` [比赛](./tournament/README.zh.md)
- [x] `>` [自定义 set](./custom-set/README.zh.md)
- [x] `no` [字母谜题](./alphametics/README.zh.md)
- [x] `>` [两个桶](./two-bucket/README.zh.md)
- [x] `>` [猪的拉丁文](./pig-latin/README.zh.md)
- [x] `>` [钻石](./diamond/README.zh.md)
- [x] `>` [螺旋矩阵](./spiral-matrix/README.zh.md)
- [x] `>` [回文产品](./palindrome-products/README.zh.md)
- [x] `no` [扑克](./poker/README.zh.md)
- [x] `>` [grep](./grep/README.zh.md)
- [x] `no` [音阶生成器](./scale-generator/README.zh.md)
- [x] `no` [十进制](./decimal/README.zh.md)
- [x] `>` [字谜](./anagram/README.zh.md)
- [x] `>` [蛋白质翻译](./protein-translation/README.zh.md)
- [x] `>` [机器人名称](./robot-name/README.zh.md)
- [x] `>` [书店](./book-store/README.zh.md)

### [难](./high.md)

---

- [x] `>` [OCR 号码](./ocr-numbers/README.zh.md)
- [x] `>` [扫雷](./minesweeper/README.zh.md)
- [x] `>` [骨牌](./dominoes/README.zh.md)
- [x] `>` [并行字母频率](./parallel-letter-frequency/README.zh.md)
- [x] `>` [矩形](./rectangles/README.zh.md)
- [x] `>` [Forth](./forth/README.zh.md)
- [x] `>` [循环缓冲区](./circular-buffer/README.zh.md)
- [x] `>` [React](./react/README.zh.md)

### [还没标签](./untag.md)

- [x] `>` [十六进制](./hexadecimal/README.zh.md)
- [x] `>` [核苷酸密码子](./nucleotide-codons/README.zh.md)
- [x] `>` [two-fer](./two-fer/README.zh.md)
