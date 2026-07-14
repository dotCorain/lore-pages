# 配置

Lore Pages 使用项目根目录下的 `Lore.toml` 作为配置文件，分为 `[parser]` 和 `[renderer]` 两个段。

## 完整示例

```toml
[parser]
# ── 链接语法 ──
url_link_key = "|"
url_link_key_escape = "\\|"
lore_link_key = "="
lore_link_key_escape = "\\="

# ── 注释 ──
comment_key = "%"
comment_key_escape = "\\%"

# ── 分隔线 ──
breakline_key = "---"
breakline_key_escape = "\\---"

# ── 图片 ──
image_key = "|"
image_key_escape = "\\|"

# ── 占位符 ──
placeholder_key = "_"
placeholder_key_escape = "\\_"

# ── 折叠块 ──
inner_open_key = "-"
inner_open_key_escape = "\\-"
inner_close_key = "+"
inner_close_key_escape = "\\+"
inner_url_key = ">"
inner_url_key_escape = "\\>"
inner_lore_key = "="
inner_lore_key_escape = "\\="


[renderer]
site_title = "My Lore Site"
from_lore_path = "./docs-src"
to_html_path = "./docs"
lang = "zh-CN"
link_base = ""
css_url = "https://fleetinglore.github.io/css/style.css"
scripts = []
```

## [parser] — 解析器配置

所有字段都有默认值。只需覆盖你想要自定义的部分。

### 链接语法

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `url_link_key` | `\|` | 外部链接的分隔符：`名称 \| URL` |
| `url_link_key_escape` | `\\\|` | 转义写法，在段落中显示字面量的 `\|` |
| `lore_link_key` | `=` | 内部链接的分隔符：`名称 = 路径` |
| `lore_link_key_escape` | `\\=` | 转义写法 |

### 注释

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `comment_key` | `%` | 注释前缀：`% 这是一条注释` |
| `comment_key_escape` | `\\%` | 转义写法 |

### 分隔线与占位符

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `breakline_key` | `---` | 分隔线标记 |
| `breakline_key_escape` | `\\---` | 转义写法 |
| `placeholder_key` | `_` | 占位符前缀 |
| `placeholder_key_escape` | `\\_` | 转义写法 |

### 图片

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `image_key` | `\|` | 图片前缀：`\| https://...` |
| `image_key_escape` | `\\\|` | 转义写法 |

### 折叠块

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `inner_open_key` | `-` | 展开折叠块的前缀 |
| `inner_open_key_escape` | `\\-` | 转义写法 |
| `inner_close_key` | `+` | 收起折叠块的前缀 |
| `inner_close_key_escape` | `\\+` | 转义写法 |
| `inner_url_key` | `>` | 折叠块内部引用的 URL 分隔符 |
| `inner_url_key_escape` | `\\>` | 转义写法 |
| `inner_lore_key` | `=` | 折叠块内部引用的 Lore 路径分隔符 |
| `inner_lore_key_escape` | `\\=` | 转义写法 |

## [renderer] — 渲染器配置

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `site_title` | `"LorePages"` | 网站标题，用作 HTML `<title>` 的默认值 |
| `from_lore_path` | `"./lore"` | `.lore` 源文件目录 |
| `to_html_path` | `"./html"` | 生成的 HTML 输出目录 |
| `css_url` | 远程 CSS | 使用的样式表 URL |
| `lang` | `"en-US"` | HTML `<html lang="...">` 属性值 |
| `link_base` | `""` | 所有相对链接的前缀（部署到子目录时有用） |
| `scripts` | `[]` | 需要注入的 JavaScript 文件 URL 列表 |
