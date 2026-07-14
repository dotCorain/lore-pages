# 折叠块

折叠块是一种**可展开 / 可收起**的内容区域，可以引用外部 URL 或项目内的 Lore 页面。

## 基本语法

```
- 标题 分隔符 内容URL/路径
+ 标题 分隔符 内容URL/路径
```

- `-` 开头：**默认展开**的折叠块
- `+` 开头：**默认收起**的折叠块
- 标题后面**不能为空**
- 分隔符后面是内容的 URL 或路径

## 引用外部 URL

使用 `>` 作为分隔符（默认）：

```
- 参考资料 > https://example.com/notes.html
+ 更多阅读 > https://example.com/articles.html
```

渲染结果：

```html
<div class="foldable expanded"
     data-url="https://example.com/notes.html"
     data-title="参考资料"></div>

<div class="foldable"
     data-url="https://example.com/articles.html"
     data-title="更多阅读"></div>
```

折叠块的实际展开/收起行为由前端 JavaScript 和 CSS 控制——Lore Pages 只生成带有 `class="foldable"` 属性的 `<div>` 标签。

## 引用 Lore 页面

使用 `=` 作为分隔符（默认），可以引用项目内的 `.lore` 页面：

```
- 相关笔记 = notes/rust-basics
+ 附录 = appendix/glossary
```

路径会被加上 `link_base` 前缀（如果配置了的话），并转换为 `.html` 扩展名。

渲染结果：

```html
<div class="foldable lore expanded"
     data-url="notes/rust-basics.html"
     data-title="相关笔记"></div>

<div class="foldable lore"
     data-url="appendix/glossary.html"
     data-title="附录"></div>
```

注意 `class` 中多了 `lore`，表示这是内部页面引用。

## 关闭空标题的折叠块

如果关闭折叠块时标题为空，则不渲染：

```
+  > https://example.com   ← title 非空，会渲染
+ > https://example.com     ← title 为空，不渲染
```

## 完整示例

```
# 读书笔记

## Rust 编程

- 官方文档 > https://doc.rust-lang.org/book/
+ 社区资源 > https://rust-lang.org/community

## 相关笔记

- Rust 基础 = rust-basics
- 所有权 = rust-ownership
+ 高级主题 = rust-advanced
```

## 自定义符号

在 `Lore.toml` 中配置：

```toml
[parser]
inner_open_key = "▼"
inner_open_key_escape = "\\▼"
inner_close_key = "▲"
inner_close_key_escape = "\\▲"
inner_url_key = "→"
inner_url_key_escape = "\\→"
inner_lore_key = "⇒"
inner_lore_key_escape = "\\⇒"
```

然后可以这样写：

```
▼ 官网 → https://example.com
▲ 笔记 ⇒ notes/index
```
