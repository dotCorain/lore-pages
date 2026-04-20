// Crate 入口：导出子模块（不使用文档注释，保持简单的内联说明）
// 这些模块组成了库的主要功能：framework 提供可配置的解析/渲染框架，
// ir 定义中间表示，parser/render 提供默认实现。
pub mod framework;
pub mod ir;
pub mod parser;
pub mod render;
