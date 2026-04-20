# lang-core

`lang-core` 提供文档的中间表示（IR），用于解析器与渲染器之间的数据传递。IR 被设计为简单且可扩展，便于不同解析器/渲染器互操作。

快速示例

```rust
use lang_core::{Document, Node};

let mut doc = Document::new();
doc.push(Node::Heading { level: 1, content: "标题".to_string() });
doc.push(Node::Paragraph { content: "第一段内容".to_string() });
assert_eq!(doc.len(), 2);
```

API 参考（常用）

- `struct Document`:
	- 字段: `nodes: Vec<Node>`
	- `Document::new() -> Document`：创建空文档
	- `push(&mut self, node: Node)`：在文档末尾添加节点
	- `is_empty(&self) -> bool`：是否为空文档
	- `len(&self) -> usize`：节点数量

- `enum Node`：
	- `Heading { level: u8, content: String }`：标题节点，`level` 推荐范围 1-4
	- `Paragraph { content: String }`：段落节点

设计说明

本 crate 保持 IR 简洁，把语义复杂的逻辑留给 parser/renderer 实现。未来可以扩展节点类型（如列表、代码块、链接等）。

源码位置：`lang-core/src/ir.rs`
