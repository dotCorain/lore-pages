# Lore Pages

## src/ir

所有的语法设计都在这个 crate 体现.

目前有 `Anhor` 和 `Category` 这两种要素.以后还会有 `Tag` 和 `Domain`.

## src/parser

从代码到数据实例.

目前有标题和段落这两种行类型.

## src/render

从数据实例到目标.

标题转译为 `<hn>`,段落转化为 `<p>`.

## src/framework

基于 parser 和 render 导出 cli.

# TODO List

重写路径系统

`serve` 指令

抽象层和序列集成
