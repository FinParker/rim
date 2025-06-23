<!--
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-05-03 22:36:30
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-23 09:13:04
 * @FilePath: \rim\README.md
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
# rim-editor

rim-editor - A small text-editor written by Rust.

一个使用Rust编写的小型文本编辑器.

- Build follow [build-your-own-x](https://github.com/codecrafters-io/build-your-own-x) and [hecto指南](https://philippflenker.com/hecto/)

## ✨ 主要特性

    增强信息面板：实时显示操作反馈（相比 hecto 的新特性）

    支持打开/编辑文本文件

    多种导航方式：

        方向键移动光标

        PageUp/PageDown 翻页

        Home/End 跳转行首/行尾

    终端尺寸自适应

    内置基础日志系统

    轻量高效（Rust 原生编译）

## 🚀 安装

```bash
cargo install rim-editor
```

## 🖥 使用

```bash
rim-editor path/to/file.txt
```

## 🔧 开发

```bash
# 克隆仓库
git clone https://github.com/iming/rim.git

# 运行
cargo run -- filename.txt

# 代码检查
cargo clippy

# 生成文档
cargo doc --open
```

## 📦 发布到 crates.io

```bash
cargo login
cargo publish
```

    警告：crates.io 不允许删除已发布版本，请谨慎操作

## 📜 Git 提交规范

feat:     新增功能

fix:      修复bug

refactor: 代码重构

perf:     性能优化

text:     文本更新

docs:     文档更新

style:    代码样式

test:     测试相关

chore:    构建/依赖

build:    构建系统

ci:       CI/CD

revert:   撤销提交  

## git command

`git add -u` update tracked files  
`git commit -m "tag: xxx"`