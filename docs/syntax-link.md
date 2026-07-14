# 链接

Lore Pages 支持两种链接：**外部链接** (URL Link) 和 **内部链接** (Lore Link)。

## 外部链接 (URL Link)

指向外部网址的链接。

### 语法

```
名称 | URL
```

- 名称和 URL 之间用**空格 + `|` + 空格**分隔。
- 默认分隔符是 `|`，可通过 `Lore.toml` 中的 `url_link_key` 修改。

### 示例

```
GitHub | https://github.com
Crates.io | https://crates.io
```

### 渲染结果

```html
<p style="margin-left: 20px">
  <a href="https://github.com" class="link_url">GitHub</a>
</p>
<p style="margin-left: 20px">
  <a href="https://crates.io" class="link_url">Crates.io</a>
</p>
```

## 内部链接 (Lore Link)

指向同一个 Lore 项目中其他页面的链接。

### 语法

```
名称 = 路径
```

- 名称和路径之间用**空格 + `=` + 空格**分隔。
- 默认分隔符是 `=`，可通过 `Lore.toml` 中的 `lore_link_key` 修改。
- 路径是相对于源文件所在目录的 `.lore` 文件路径（不含扩展名）。
- 程序会自动检查目标文件是否存在，若不存在会发出警告。

### 示例

```
首页 = index
入门 = getting-started
详情 = subdir/details
```

### 路径解析规则

内部链接的路径会基于**源文件所在目录**进行解析。

例如：

| 源文件位置 | 链接写法 | 实际目标 |
|-----------|---------|---------|
| `index.lore` | `首页 = about` | `./about.lore` |
| `notes/index.lore` | `相关 = rust` | `./notes/rust.lore` |
| `notes/index.lore` | `上级 = ../index` | `./index.lore` |

渲染时，链接会被加上 `link_base` 前缀（如果配置了的话）。

### 渲染结果

```html
<p style="margin-left: 20px">
  <a href="index.html" class="link_lore">首页</a>
</p>
```

## 链接验证

Lore Pages 会在编译时验证所有内部链接指向的 `.lore` 文件是否存在。如果找不到目标文件，会输出类似这样的警告：

```
Warning: lore link "首页" -> "index"
  in source file: ./docs-src/getting-started.lore
  folder exists, but "index.lore" not found in "./docs-src"
```

## 自定义分隔符

如果你想用其他符号替代 `|` 和 `=`，在 `Lore.toml` 中配置：

```toml
[parser]
url_link_key = "->"
url_link_key_escape = "\\->"
lore_link_key = ":"
lore_link_key_escape = "\\:"
```

这样你可以写：

```
GitHub -> https://github.com
首页 : index
```
