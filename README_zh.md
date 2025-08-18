[<img src="https://img.shields.io/badge/lang-English-blue?style=for-the-badge">](README.md) 

# RustVim - 一个用 Rust 实现的类 Vim 文本编辑器

RustVim 是一个用 Rust 编写的功能全面的类 Vim 文本编辑器，支持多种编辑模式、可视化选择（包括块模式）、文件操作、搜索功能、强大的撤销/重做系统，以及灵活的配置系统。

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-vim--like-green?style=for-the-badge)
[![codecov](https://codecov.io/gh/zangjiucheng/rustvim/branch/main/graph/badge.svg)](https://codecov.io/gh/zangjiucheng/rustvim)
[![Build Status](https://github.com/zangjiucheng/rustvim/workflows/Build%20RustVim/badge.svg)](https://github.com/zangjiucheng/rustvim/actions)

## 功能特性

- **模态编辑**：普通、插入、可视（字符/行/块）、命令、搜索模式
- **可视块模式**：矩形选择与块操作
- **复合撤销/重做**：复杂操作可一次性撤销/重做
- **复制/粘贴**：完整寄存器系统
- **模式搜索**：前后向搜索与高亮
- **多文件支持**：单次会话编辑多个文件
- **状态栏**：文件信息、模式显示、光标位置
- **铃声/闪屏反馈**：无效按键即时反馈
- **可配置按键映射**：为自定义按键绑定奠定基础
- **TOML 配置系统**：人类可读的 `~/.rustvimrc` 配置文件，支持运行时 `:set` 命令
- **插件系统**：原生 Rust 插件架构，支持功能扩展

## 快速开始

### 安装
- **需要 Rust 1.85+** ([Rust 安装](https://rustup.rs/))
- 克隆并构建：
```bash
git clone <repository-url>
cd rustvim
cargo build --release
```
- 运行：
```bash
cargo run [filename]
# 或运行已构建的二进制文件
target/release/rustvim [filename]
```

### 配置设置
1. 复制示例配置：
   ```bash
   cp .rustvimrc.example ~/.rustvimrc
   ```
2. 用你喜欢的编辑器修改 `~/.rustvimrc`（TOML 格式）
3. 启动时自动加载配置，随时可用 `:set` 命令修改选项

#### 示例 `.rustvimrc`
```toml
tab_size = 4
show_line_numbers = true
word_wrap = false
auto_save = false
search_case_sensitive = false
search_highlight = true
```

#### 常用 `:set` 命令
- `:set number` / `:set nonumber` — 显示/隐藏行号
- `:set tabsize=8` — 设置 Tab 宽度
- `:set wrap` / `:set nowrap` — 启用/禁用自动换行
- `:set auto_save` / `:set noauto_save` — 启用/禁用自动保存
- `:set hlsearch` / `:set nohlsearch` — 启用/禁用搜索高亮

## 使用方法

### 模态编辑与导航
| 按键         | 操作                       |
|--------------|----------------------------|
| `ESC`        | 普通模式                   |
| `i`          | 插入模式                   |
| `v`          | 可视模式                   |
| `V`          | 可视行模式                 |
| `Ctrl+V`     | 可视块模式                 |
| `:`          | 命令模式                   |
| `/`          | 搜索模式                   |
| `h/j/k/l`    | 左/下/上/右移动            |
| `w/b`        | 单词前进/后退              |
| `0/$`        | 行首/行尾                  |
| `gg/G`       | 文件首/尾                  |

### 文本操作
| 按键         | 操作                       |
|--------------|----------------------------|
| `o/O`        | 在下/上方插入新行          |
| `dd`         | 删除当前行                 |
| `yy`         | 复制当前行                 |
| `p`          | 粘贴                       |
| `u`          | 撤销                       |
| `Ctrl+R`     | 重做                       |

### 文件命令
| 命令         | 操作                       |
|--------------|----------------------------|
| `:w`         | 保存                       |
| `:q`         | 退出                       |
| `:wq`        | 保存并退出                 |
| `:e filename`| 编辑新文件                 |
| `:wc`        | 统计字数（插件）           |
| `:hello`     | Hello World（插件）        |
| `:sort`      | 行排序（插件）             |

### 搜索
| 命令         | 操作                       |
|--------------|----------------------------|
| `/pattern`   | 前向搜索                   |
| `?pattern`   | 后向搜索                   |
| `n/N`        | 下一个/上一个匹配          |
| `ESC`        | 退出搜索                   |

## 架构概览

### 语法高亮
- 基于 Tree-sitter 的语法高亮，支持多种语言
- 可通过 `:set syntax` / `:set nosyntax` 或在 `~/.rustvimrc` 中设置（`syntax_highlighting = true`）启用/禁用
- 根据文件扩展名自动检测语言（可切换）
- 代码结构和搜索结果高亮显示

```
src/
├── main.rs           # 应用入口
├── editor.rs         # 编辑器核心逻辑
├── buffer.rs         # 文本缓冲区管理
├── terminal.rs       # 终端控制
├── syntax.rs         # 语法高亮
├── input.rs          # 按键输入处理
├── commands.rs       # 命令处理
├── keymap.rs         # 按键映射系统
├── io.rs             # 文件 I/O
├── history.rs        # 撤销/重做系统
├── config.rs         # 配置系统
├── plugin.rs         # 插件系统核心
└── plugins/          # 插件实现
    ├── mod.rs        # 插件模块组织
    └── utils.rs      # 内置插件（如 wc、hello、sort）

tests/                # 160+ 测试保证可靠性
```

## 终端原始模式
- 进入原始模式：使用 `Terminal::enter_raw_mode()`，返回一个 guard 对象，期间终端输入不会被缓冲或回显。
- 退出原始模式：调用 `Terminal::exit_raw_mode(guard)`，guard 被释放后终端设置自动恢复。

## 测试与质量
- 运行所有测试：`cargo test`
- 代码覆盖率：`./coverage.sh html`（见 `target/llvm-cov/html/index.html`）
- Lint 检查：`cargo clippy --all-targets --all-features -- -D warnings`
- Pre-commit 钩子：`./scripts/install-pre-commit-hook.sh`

## 文档与资源
- **[ARCHITECTURE.md](ARCHITECTURE.md)** — 系统架构概述
- **[docs/PLUGIN_SYSTEM.md](docs/PLUGIN_SYSTEM.md)** — 插件开发指南
- **[docs/daily-summaries/](docs/daily-summaries/)** — 按天记录的开发进度
- **[.rustvimrc.example](.rustvimrc.example)** — 配置文件示例
- **Vim 文档** — [vimhelp.org](https://vimhelp.org/)
- **Kilo 编辑器教程** — [viewsourcecode.org/snaptoken/kilo/](https://viewsourcecode.org/snaptoken/kilo/)

## 许可证
MIT 许可证 — 详见 [LICENSE](LICENSE)

## 鸣谢
- 灵感来源于 Vim 及其模态编辑理念
- 基于 Rust 构建，保障内存安全与高性能
- 终端界面采用标准 ANSI 转义序列，兼容性强
