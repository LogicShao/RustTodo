# 🚀 快速开始指南

## 第一步：安装依赖

由于 Electron 下载可能遇到网络问题，建议使用国内镜像：

```bash
# 设置淘宝镜像
npm config set registry https://registry.npmmirror.com
npm config set electron_mirror https://registry.npmmirror.com/-/binary/electron/

# 安装依赖
npm install --save-dev electron typescript @types/node ts-node

# 恢复默认源（可选）
# npm config set registry https://registry.npmjs.org/
```

## 第二步：编译项目

```bash
# 1. 编译 Rust（生成可执行文件）
cargo build --release

# 2. 编译 TypeScript（生成 dist/ 目录）
npm run build:ts
```

## 第三步：运行应用

```bash
# 开发模式运行
npm run dev
```

你应该会看到一个漂亮的 GUI 窗口打开！

---

## 测试 Rust CLI（可选）

```bash
# 运行交互式模式
cargo run

# 测试命令行模式
cargo run -- add "学习 Rust"
cargo run -- list
cargo run -- list --json
cargo run -- complete 1
cargo run -- remove 1
```

---

## 常用命令

```bash
# 编译 TypeScript（监听模式，自动重新编译）
npm run watch

# 只编译 Rust
npm run build:rust

# 只编译 TypeScript
npm run build:ts

# 完整编译（Rust + TypeScript）
npm run build

# 运行应用
npm start
```

---

## 项目结构说明

```
📁 项目根目录
  ├── 📂 src/              Rust 源码
  │   ├── main.rs         主程序（CLI + 命令行模式）
  │   └── todo.rs         数据结构和逻辑
  │
  ├── 📂 electron/         Electron 主进程（TypeScript）
  │   └── main.ts         窗口管理 + IPC 通信
  │
  ├── 📂 renderer/         前端界面
  │   ├── index.html      HTML 结构
  │   ├── styles.css      样式
  │   └── renderer.ts     前端逻辑（TypeScript）
  │
  ├── 📂 dist/             编译输出（自动生成）
  ├── 📂 target/           Rust 编译输出（自动生成）
  │
  ├── 📄 Cargo.toml        Rust 配置
  ├── 📄 package.json      Node.js 配置
  ├── 📄 tsconfig.json     TypeScript 配置
  └── 📄 LEARNING.md       完整学习指南
```

---

## 遇到问题？

### 问题1：TypeScript 编译错误

```bash
# 清理并重新编译
rm -rf dist
npm run build:ts
```

### 问题2：Electron 依赖安装失败

**原因**：网络问题，无法从 GitHub 下载 Electron

**解决方案**：
1. 使用镜像源（见第一步）
2. 或使用代理/VPN

### 问题3：找不到 Rust 可执行文件

**检查**：
```bash
# Windows
ls target/release/hello_rust.exe

# Linux/Mac
ls target/release/hello_rust
```

**解决**：如果没有这个文件，运行 `cargo build --release`

---

## 下一步学习

阅读 `LEARNING.md` 文件，按照学习路线逐步掌握：

1. **Rust 基础**：数据结构、错误处理、序列化
2. **TypeScript 基础**：异步编程、DOM 操作、类型系统
3. **Electron 基础**：进程模型、IPC 通信、窗口管理

祝学习愉快！🎉
