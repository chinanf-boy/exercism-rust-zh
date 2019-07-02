# 音阶生成器

> 译：真不知道说什么。

给出一个音调，或是开始的音符以及一组间隔，从音调开始并遵循指定的间隔模式生成音阶。

西方音乐中的音阶，基于彩色(12 音符)音阶。该音阶可表示为以下一组音高:

A，A#，B，C，C#，D，D#，E，F，F#，G，G#

给出一个尖音符(用#表示)也可以表示为它前面的平音符(用 b 表示)，所以半音音阶也可以这样写:

A，Bb，B，C，Db，D，Eb，E，F，Gb，G，Ab

大调和小调的音阶和模式是这个十二音高集合的子集。它们有七个音高，称为全音阶音阶。这些音阶中的音符集合使用锐利或平面，根据音调。这是一个列表，其中包括:

没有尖(Sharps)或平(Flats)：C 大调（major）和 a 小调(minor)

使用 Sharps:
G, D, A, E, B, F# major
e, b, f#, c#, g#, d# minor

使用 Flats:
F, Bb, Eb, Ab, Db, Gb major
d, g, c, f, bb, eb minor

全音阶音阶以及源自半音音阶的所有其他音阶都是间隔建立的。间隔是两个音高之间的间距。

最简单的间隔是在两个相邻音符之间，称为"半步"或"小秒"(有时写为小写"m")。具有中间音符的两个音符之间的间隔称为"整步"或"大秒"(写为大写"M")。仅使用相邻音符之间的这两个间隔，来构建全音阶音阶。

非全音阶音阶可以包含其他间隔。一个"先增强"的间隔，写上 A，具有两个中间音符(例如，从 A 到 C 或从 D 到 E)。间隔也越来越小，但他们不会参与这个练习。

[help-page]: https://exercism.io/tracks/rust/learning
[modules]: https://doc.rust-lang.org/book/ch07-00-modules.html
[cargo]: https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
[rust-tests]: https://doc.rust-lang.org/book/ch11-02-running-tests.html
