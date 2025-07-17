[<img src="https://img.shields.io/badge/lang-English-blue?style=for-the-badge">](README.md) 

# RustVim - 一个用 Rust 实现的类 Vim 文本编辑器

RustVim 是一个用 Rust 编写的功能全面的类 Vim 文本编辑器，支持多种编辑模式、可视化选择（包括块模式）、文件操作、搜索功能以及强大的撤销/重做系统。

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-vim--like-green?style=for-the-badge)
[![codecov](https://codecov.io/gh/zangjiucheng/rustvim/branch/main/graph/badge.svg)](https://codecov.io/gh/zangjiucheng/rustvim)
[![Build Status](https://github.com/zangjiucheng/rustvim/workflows/Build%20RustVim/badge.svg)](https://github.com/zangjiucheng/rustvim/actions)

## 功能特性

### 🎯 模态编辑
- **普通模式**：导航与命令执行
- **插入模式**：完整的文本插入与编辑能力
- **可视模式**：字符、行、块级文本选择
- **命令模式**：执行 ex 命令（如 `:w`, `:q`, `:wq`, `:e` 等）
- **搜索模式**：支持高亮的前后向文本搜索

### 📝 文本操作
- **可视块模式**：使用 Ctrl+V 进行矩形区域选择
- **块操作**：复制、删除和操作矩形文本块
- **复合撤销**：复杂操作可一次性撤销
- **复制/粘贴**：完整的寄存器系统支持复制粘贴
- **行操作**：插入、删除和操作整行文本

### 🔍 搜索与导航
- **模式搜索**：支持前向（`/`）和后向（`?`）搜索
- **搜索高亮**：匹配项高亮显示
- **单词导航**：前进（`w`）和后退（`b`）单词移动
- **行导航**：行首（`0`, `^`）和行尾（`$`）
- **文件导航**：跳转到文件开头（`gg`）和结尾（`G`）

### 💾 文件管理
- **多文件支持**：单次会话编辑多个文件
- **文件操作**：加载、保存、另存为、新建文件
- **变更检测**：未保存变更时警告，防止数据丢失
- **换行符保留**：保持原始文件格式

### ⚡ 高级功能
- **撤销/重做**：完整历史追踪，支持 `u` 和 `Ctrl+R`
- **计数前缀**：命令支持数字倍数（如 `5j`, `3dd` 等）
- **操作符-动作**：操作符与动作组合（如 `d3w`, `y5j`）
- **状态栏**：显示文件信息、模式和光标位置
- **错误处理**：全面的错误信息与恢复机制

## 安装

### 依赖环境
- Rust 1.85+（可从 [rustup.rs](https://rustup.rs/) 安装）

### 源码构建
```bash
git clone <repository-url>
cd rustvim
cargo build --release
```

### 运行
```bash
# 启动空缓冲区
cargo run

# 加载指定文件
cargo run filename.txt

# 或运行已构建的二进制文件
./target/release/rustvim [filename]
```

## 使用方法

### 基本命令

#### 模式切换
| 按键 | 操作 |
|------|------|
| `ESC` | 返回普通模式 |
| `i` | 进入插入模式 |
| `v` | 进入可视（字符）模式 |
| `V` | 进入可视（行）模式 |
| `Ctrl+V` | 进入可视块模式 |
| `:` | 进入命令模式 |
| `/` | 进入搜索模式 |

#### 导航
| 按键 | 操作 |
|------|------|
| `h/j/k/l` | 左/下/上/右移动 |
| `w/b` | 单词前进/后退 |
| `0` | 行首 |
| `$` | 行尾 |
| `gg` | 跳转到首行 |
| `G` | 跳转到末行 |

#### 文本操作
| 按键 | 操作 |
|------|------|
| `o/O` | 在下/上方插入新行 |
| `dd` | 删除当前行 |
| `yy` | 复制当前行 |
| `p` | 在光标后粘贴 |
| `u` | 撤销 |
| `Ctrl+R` | 重做 |

### 可视模式操作

#### 字符选择（`v`）
```
v           # 开始选择
<移动>      # 扩展选择
d           # 删除选中文本
y           # 复制选中文本
ESC         # 退出可视模式
```

#### 行选择（`V`）
```
V           # 开始行选择
<移动>      # 扩展到整行
d           # 删除选中行
y           # 复制选中行
```

#### 块选择（`Ctrl+V`）
```
Ctrl+V      # 开始块选择
<移动>      # 创建矩形选择
d           # 删除块
y           # 复制块
ESC         # 退出可视模式
```

### 文件命令
| 命令 | 操作 |
|------|------|
| `:w` | 保存当前文件 |
| `:w filename` | 另存为指定文件 |
| `:q` | 退出（有变更检测） |
| `:q!` | 强制退出（丢弃变更） |
| `:wq` | 保存并退出 |
| `:e filename` | 编辑新文件 |

### 搜索操作
| 命令 | 操作 |
|------|------|
| `/pattern` | 前向搜索 |
| `?pattern` | 后向搜索 |
| `n` | 下一个匹配 |
| `N` | 上一个匹配 |
| `ESC` | 退出搜索 |

## 示例

### 基本编辑流程
```bash
# 打开文件
cargo run example.txt

# 跳转到单词并进入可视模式
w v 3w    # 选择 3 个单词

# 复制选择内容
y

# 跳转到其他位置并粘贴
G p

# 保存并退出
:wq
```

### 可视块操作
```bash
# 选择矩形块
Ctrl+V    # 进入可视块模式
3j 5l     # 选择 4 行 × 6 列

# 删除块
d         # 删除矩形选择

# 撤销整个操作
u         # 恢复完整块
```

### 多文件编辑
```bash
# 打开多个文件
cargo run file1.txt file2.txt

# 编辑第一个文件
i "Hello World" ESC

# 切换到第二个文件
:bn

# 修改并保存两个文件
:w
:bp
:w
```

## 架构

### 项目结构
```
src/
├── main.rs              # 应用入口
├── lib.rs               # 模块导出
├── editor.rs            # 编辑器核心状态与事件循环
├── buffer.rs            # 文本缓冲区管理
├── terminal.rs          # 终端控制与渲染
├── input.rs             # 按键输入处理
├── commands.rs          # 命令处理
├── keymap.rs            # 按键映射系统
├── io.rs                # 文件 I/O 操作
└── history.rs           # 撤销/重做系统

tests/                   # 全面测试套件
├── visual_block_mode_tests.rs
├── visual_mode_tests.rs
├── buffer_tests.rs
└── ...（共 148+ 测试）
```

### 设计原则
- **模态架构**：编辑模式分离清晰
- **可组合命令**：操作符与动作自然组合
- **内存安全**：Rust 所有权系统防止常见编辑器 bug
- **全面测试**：148+ 测试保证可靠性
- **Vim 兼容性**：忠实还原 Vim 行为

## 测试

运行全面测试套件：

```bash
# 运行所有测试
cargo test

# 运行指定测试类别
cargo test --test visual_block_mode_tests
cargo test --test buffer_tests
cargo test --test integration_tests

# 显示输出
cargo test -- --nocapture
```

## 代码覆盖率

生成并查看代码覆盖率报告，确保测试全面：

### 依赖环境
LLVM 覆盖率工具已包含在开发依赖中，或可手动安装：
```bash
cargo install cargo-llvm-cov
```

### 快速覆盖率分析
使用内置脚本便捷分析：
```bash
# 首次需赋予可执行权限
chmod +x coverage.sh

# 生成覆盖率摘要（默认）
./coverage.sh

# 生成 HTML 报告并在浏览器打开
./coverage.sh html

# 生成所有报告格式
./coverage.sh all

# 显示帮助与可用选项
./coverage.sh help
```

### 手动覆盖率命令
```bash
# 生成 HTML 覆盖率报告（推荐）
cargo llvm-cov --html

# 终端显示摘要
cargo llvm-cov --summary-only

# 显示详细逐行覆盖率
cargo llvm-cov --show-missing-lines

# 生成 LCOV 格式用于 CI/CD 集成
cargo llvm-cov --lcov --output-path coverage.lcov
```

### 查看覆盖率报告
- **HTML 报告**：在浏览器打开 `target/llvm-cov/html/index.html`
- **终端**：覆盖率百分比直接显示在控制台
- **CI 集成**：LCOV 格式可用于 Codecov 等服务

### 自动化覆盖率（CI/CD）
- **GitHub Actions**：每次推送和 PR 自动运行覆盖率
- **Codecov 集成**：详细覆盖率跟踪与趋势分析
- **PR 评论**：覆盖率摘要直接发布在 PR 上
- **覆盖率工件**：报告归档可下载分析

## 性能

- **高效文本存储**：基于行的表示，O(1) 行访问
- **最小化分配**：精细内存管理保证响应速度
- **非阻塞输入**：所有操作即时反馈
- **复合操作**：复杂操作（如块删除）原子执行

## 贡献

欢迎贡献！详细信息请参见 [贡献指南](CONTRIBUTING.md)：

- 🚀 **快速开始**：开发环境与流程
- ✅ **质量检查**：测试、覆盖率、格式化与 lint
- 📋 **规范**：代码标准与最佳实践
- 🔄 **CI/CD 流程**：自动化质量保障
- 🎯 **贡献方向**：最需要帮助的领域

### 快速上手
1. Fork 仓库并创建功能分支
2. 编写更改并补充全面测试
3. 运行质量检查：`cargo test && ./coverage.sh && cargo clippy`
4. 提交 Pull Request

**所有 PR 都会自动进行质量检查和覆盖率分析。**

## 路线图

### 已完成 ✅
- [x] 模态编辑（普通、插入、命令、搜索）
- [x] 可视选择（字符、行、块）
- [x] 文件操作与多文件支持
- [x] 复合撤销/重做系统
- [x] 搜索与导航
- [x] 全面测试覆盖

### 规划中 🚧
- [ ] 语法高亮
- [ ] 配置系统
- [ ] 插件架构
- [ ] 正则表达式搜索
- [ ] 多窗口/分屏
- [ ] 性能优化

## 文档

详细实现与技术细节请参见：
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - 完整系统架构与设计概述
- **[docs/daily-summaries/](docs/daily-summaries/)** - 按天记录的开发进度日志

## 许可证

本项目采用 MIT 许可证，详见 [LICENSE](LICENSE) 文件。

## 鸣谢

- 灵感来源于 Vim 及其模态编辑理念
- 基于 Rust 构建，保障内存安全与高性能
- 终端界面采用标准 ANSI 转义序列，兼容性强

## 资源与参考

- [Kilo 编辑器教程](https://viewsourcecode.org/snaptoken/kilo/) - 原始终端实现灵感
- [Vim 文档](https://vimhelp.org/) - Vim 行为参考
- [ANSI 转义码](https://en.wikipedia.org/wiki/ANSI_escape_code) - 终端控制序列
- [Rust 书籍](https://doc.rust-lang.org/book/) - Rust 语言参考

## 持续集成

本项目包含完善的 GitHub Actions 工作流，实现自动化质量保障：

### 🔧 构建工作流（`.github/workflows/build.yml`）
- **跨平台构建**：自动构建 Linux、macOS
- **质量检查**：运行格式化（`cargo fmt`）、lint（`cargo clippy`）和测试
- **依赖缓存**：智能缓存优化构建时间
- **自动发布**：推送标签时自动创建 GitHub 发布并附带二进制文件
- **工件存储**：构建工件可下载

### 📊 覆盖率工作流（`.github/workflows/coverage.yml`）
- **自动覆盖率**：每次推送和 PR 自动运行
- **多种报告**：生成 HTML、LCOV、摘要等格式
- **Codecov 集成**：上传到 Codecov 进行详细分析与趋势跟踪
- **PR 评论**：覆盖率摘要直接发布在 PR 上
- **GitHub 摘要**：覆盖率结果在 Actions 标签页可见

### 工作流触发
- **推送到 main/develop**：完整构建与覆盖率分析
- **Pull Request**：质量检查与覆盖率报告
- **版本标签**：自动发布跨平台二进制文件

## 开发环境

### 依赖环境
- Rust 1.85+（可从 [rustup.rs](https://rustup.rs/) 安装）
- Git 版本控制

### 设置 Pre-commit 钩子

为保证代码质量和一致性，项目提供 pre-commit 钩子自动检查格式和 lint：

```bash
# 快速安装 Git pre-commit 钩子
./scripts/install-pre-commit-hook.sh

# 备选方案：使用 pre-commit 框架
pip install pre-commit
pre-commit install
```

钩子会在每次提交前自动运行，检查：
- **代码格式化**（`cargo fmt`）
- **Clippy lint**（`cargo clippy`）
- **文件格式**（YAML、TOML 语法、行尾空格等）

详细设置说明见 [scripts/README.md](scripts/README.md)。

### 质量检查

```bash
# 格式化代码
cargo fmt --all

# 运行 lint
cargo clippy --all-targets --all-features -- -D warnings

# 运行测试
cargo test --all-features

# 生成覆盖率报告
./coverage.sh
```
