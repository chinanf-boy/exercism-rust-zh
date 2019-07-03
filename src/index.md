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

- [x] 为中文校对
- `>` 为测试修正，
- `no` 具有[非内置箱子](https://github.com/integer32llc/rust-playground/blob/master/compiler/base/Cargo.toml)，无法编译成功, 需要非内置的其他箱子，这时，只能说句“抱歉，要本机下载”。

> [Issue me , if you want](https://github.com/chinanf-boy/exercism-rust-zh/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc)

### [易](./low.md)

---

- [x] `>` [hello world](./hello-world/README.zh.md)
- [x] `>` [千兆秒-Gigasecond](./gigasecond/README.zh.md)
- [x] `>` [闰年-Leap](./leap/README.zh.md)
- [x] `>` [雨滴声-Raindrops](./raindrops/README.zh.md)
- [x] `>` [反转字符串-Reverse String](./reverse-string/README.zh.md)
- [x] `>` [第 n 个素数-Nth Prime](./nth-prime/README.zh.md)
- [x] `>` [迟钝孩子-Bob](./bob/README.zh.md)
- [x] `>` [啤酒之歌-Beer Song](./beer-song/README.zh.md)
- [x] `>` [谚语串烧-Proverb](./proverb/README.zh.md)
- [x] `>` [平方差-Difference Of Squares](./difference-of-squares/README.zh.md)
- [x] `>` [Sum Of Multiples](./sum-of-multiples/README.zh.md)
- [x] `>` [Grains](./grains/README.zh.md)
- [x] `>` [Pythagorean Triplet](./pythagorean-triplet/README.zh.md)
- [x] `>` [Prime Factors](./prime-factors/README.zh.md)
- [x] `>` [Series](./series/README.zh.md)
- [x] `>` [Armstrong Numbers](./armstrong-numbers/README.zh.md)
- [x] `>` [Collatz Conjecture](./collatz-conjecture/README.zh.md)
- [x] `>` [Diffie Hellman](./diffie-hellman/README.zh.md)

### [中等](./medium.md)

---

- [ ] `>` [Saddle Points](./saddle-points/README.zh.md)
- [ ] `>` [Isogram](./isogram/README.zh.md)
- [ ] `>` [Say](./say/README.zh.md)
- [ ] `>` [Run Length Encoding](./run-length-encoding/README.zh.md)
- [ ] `>` [ISBN Verifier](./isbn-verifier/README.zh.md)
- [ ] `>` [Perfect Numbers](./perfect-numbers/README.zh.md)
- [ ] `>` [Clock](./clock/README.zh.md)
- [ ] `no` [DOT DSL](./dot-dsl/README.zh.md)
- [ ] `>` [Hamming](./hamming/README.zh.md)
- [ ] `>` [Simple Linked List](./simple-linked-list/README.zh.md)
- [ ] `>` [Pascal's Triangle](./pascals-triangle/README.zh.md)
- [ ] `>` [Scrabble Score](./scrabble-score/README.zh.md)
- [ ] `>` [Pangram](./pangram/README.zh.md)
- [ ] `no` [Paasio](./paasio/README.zh.md)
- [ ] `>` [Nucleotide Count](./nucleotide-count/README.zh.md)
- [ ] `>` [Luhn](./luhn/README.zh.md)
- [ ] `>` [Largest Series Product](./largest-series-product/README.zh.md)
- [ ] `>` [Word Count](./word-count/README.zh.md)
- [ ] `>` [Atbash Cipher](./atbash-cipher/README.zh.md)
- [ ] `>` [Crypto Square](./crypto-square/README.zh.md)
- [ ] `>` [Rotational Cipher](./rotational-cipher/README.zh.md)
- [ ] `>` [Simple Cipher](./simple-cipher/README.zh.md)
- [ ] `>` [Rail Fence Cipher](./rail-fence-cipher/README.zh.md)
- [ ] `>` [ETL](./etl/README.zh.md)
- [ ] `>` [Accumulate](./accumulate/README.zh.md)
- [ ] `>` [Acronym](./acronym/README.zh.md)
- [ ] `>` [Sieve](./sieve/README.zh.md)
- [ ] `>` [RNA Transcription](./rna-transcription/README.zh.md)
- [ ] `>` [Triangle](./triangle/README.zh.md)
- [ ] `>` [Roman Numerals](./roman-numerals/README.zh.md)
- [ ] `>` [All Your Base](./all-your-base/README.zh.md)
- [ ] `>` [Grade School](./grade-school/README.zh.md)
- [ ] `>` [Binary Search](./binary-search/README.zh.md)
- [ ] `>` [Robot Simulator](./robot-simulator/README.zh.md)
- [ ] `>` [Bracket Push](./bracket-push/README.zh.md)
- [ ] `>` [Luhn From](./luhn-from/README.zh.md)
- [ ] `>` [Queen Attack](./queen-attack/README.zh.md)
- [ ] `>` [Bowling](./bowling/README.zh.md)
- [ ] `>` [Sublist](./sublist/README.zh.md)
- [ ] `>` [Space Age](./space-age/README.zh.md)
- [ ] `>` [Luhn Trait](./luhn-trait/README.zh.md)
- [ ] `>` [Macros](./macros/README.zh.md)
- [ ] `>` [Allergies](./allergies/README.zh.md)
- [ ] `>` [Variable Length Quantity](./variable-length-quantity/README.zh.md)
- [ ] `>` [Phone Number](./phone-number/README.zh.md)
- [ ] `>` [Wordy](./wordy/README.zh.md)
- [ ] `>` [Tournament](./tournament/README.zh.md)
- [ ] `>` [Custom Set](./custom-set/README.zh.md)
- [ ] `no` [Alphametics](./alphametics/README.zh.md)
- [ ] `>` [Two Bucket](./two-bucket/README.zh.md)
- [ ] `>` [Pig Latin](./pig-latin/README.zh.md)
- [ ] `>` [Diamond](./diamond/README.zh.md)
- [ ] `>` [Spiral Matrix](./spiral-matrix/README.zh.md)
- [ ] `>` [Palindrome Products](./palindrome-products/README.zh.md)
- [ ] `no` [Poker](./poker/README.zh.md)
- [ ] `>` [Grep](./grep/README.zh.md)
- [ ] `no` [Scale Generator](./scale-generator/README.zh.md)
- [ ] `no` [Decimal](./decimal/README.zh.md)
- [ ] `>` [Anagram](./anagram/README.zh.md)
- [ ] `>` [Protein Translation](./protein-translation/README.zh.md)
- [ ] `>` [Robot Name](./robot-name/README.zh.md)
- [ ] `>` [Book Store](./book-store/README.zh.md)

### [难](./high.md)

---

- [ ] `>` [OCR Numbers](./ocr-numbers/README.zh.md)
- [ ] `>` [Minesweeper](./minesweeper/README.zh.md)
- [ ] `>` [Dominoes](./dominoes/README.zh.md)
- [ ] `>` [Parallel Letter Frequency](./parallel-letter-frequency/README.zh.md)
- [ ] `>` [Rectangles](./rectangles/README.zh.md)
- [ ] `>` [Forth](./forth/README.zh.md)
- [ ] `>` [Circular Buffer](./circular-buffer/README.zh.md)
- [ ] `>` [React](./react/README.zh.md)

### [还没标签](./untag.md)

- [ ] `>` [hexadecimal](./hexadecimal/README.zh.md)
- [ ] `>` [nucleotide-codons](./nucleotide-codons/README.zh.md)
- [ ] `>` [two-fer](./two-fer/README.zh.md)
