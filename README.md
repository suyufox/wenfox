# Wenfox

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/Node.js-18%2B-green)](https://nodejs.org/)

混合 Rust 和 Node.js 的插件系统开发框架，提供高性能核心与灵活扩展能力。

## 特性

- 🚀 Rust 核心模块提供高性能基础服务
- 🎨 Node.js 插件系统支持快速功能扩展
- 🔌 跨语言插件通信接口
- 📦 开箱即用的开发环境配置

## 目录结构

```plaintext
wenfox/
├── .vscode/
├── app
├── dist
├── pom.xml            # Maven构建文件
├── package.json       # Node.js项目配置
└── README.md          # 项目说明
```

## 快速开始

```bash
# 克隆仓库
git clone https://github.com/xiehuqiqi/wenfox.git
cd wenfox

# 初始化Rust项目
cd rust-lib && cargo init && cd ..

# 初始化Node项目
cd node-app && npm init -y && cd ..
```

## 插件开发

```Rust
// Rust插件示例（占位）
fn plugin_entry() {
    println!("Rust插件加载成功！");
}
```

## 贡献指南

请阅读 [CONTRIBUTING.md](https://github.com/xiehuqiqi/wenfox/) 了解开发规范

## 许可证

本项目采用 [MIT 许可证](http://github.com/xiehuqiqi/wenfox/LICENSE)

## 注意事项

1. 开发前需安装：
   - Rust 工具链 (>=1.70)
   - Node.js LTS 版本 (>=18.x)
2. 环境变量配置参考 `.env.example`
