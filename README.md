# YK - 命令管理工具

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

YK 是一个命令管理工具，支持简单命令和复杂插件化命令的交互式管理。

## 核心特性

- **零配置启动**：开箱即用，无需配置
- **交互式搜索**：基于 fzf 的模糊查找
- **插件化架构**：支持复杂命令的插件扩展
- **剪贴板集成**：自动复制命令到剪贴板
- **跨平台支持**：Windows、Linux、macOS
- **可视化配置**：通过 JSON 文件灵活配置

## 快速开始

### 安装

#### 源码编译

```bash
git clone <repository-url>
cd yk
cargo build --release
```

#### 添加到系统 PATH

**Windows:**

```powershell
copy target\release\yk.exe C:\Windows\System32\
```

**Linux/macOS:**

```bash
sudo cp target/release/yk /usr/local/bin/
sudo chmod +x /usr/local/bin/yk
```

### 初始化配置

```bash
yk init
```

初始化后生成以下目录结构：

```plaintext
~/.config/yk/
├── config.json          # 主配置
├── simple_commands.json # 简单命令配置文件
└── plugins/            # 插件目录
```

## 使用方法

### 基本命令

| 命令 | 功能 |
|------|------|
| `yk` 或 `yk find` | 查找并执行命令 |
| `yk new` | 创建新命令 |
| `yk init` | 初始化配置 |

### 交互式界面

运行 `yk` 后进入交互式选择界面：

请选择一个命令🔍: >

界面显示信息：

- 命令索引：左侧数字
- 可执行文件：使用的程序
- 命令名称：自定义标识
- 标签：命令分类
- 配置详情：JSON 配置信息

#### 快捷键

- `Enter`：选择并执行
- `Esc/Ctrl+C`：取消
- `↑/↓` 或 `Ctrl+P/N`：导航
- 直接输入：模糊搜索

## 配置说明

### 主配置文件

文件路径：`~/.config/yk/config.json`

```json
{
  "fzf_executable": "fzf",
  "rg_executable": "rg",
  "editor": "hx",
  "if_run": true,
  "if_run_confirm": true,
  "if_yank": true
}
```

#### 配置参数

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `fzf_executable` | string | "fzf" | fzf 可执行文件路径 |
| `rg_executable` | string | "rg" | ripgrep 可执行文件路径 |
| `editor` | string | "hx" | 文本编辑器路径 |
| `if_run` | boolean | true | 是否执行选中命令 |
| `if_run_confirm` | boolean | true | 执行前是否确认 |
| `if_yank` | boolean | true | 是否复制到剪贴板 |

## 命令管理

一个完整命令的参数说明

| 参数 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `labels` | array | [] | 命令标签 |
| `description` | string | "" | 命令描述 |
| `executable` | string | "" | 可执行文件路径 |
| `entry_point` | string | "" | 插件入口文件路径是相对与插件目录的路径 |
| `args` | array | [] | 命令参数 |
| `if_shell` | boolean | false | 是否在shell中执行 |

### 创建简单命令或者插件

简单命令使用 `yk new` 交互式创建,根据提示进行创建即可,也可以进行手动修改对应文件

#### 示例配置

```json
{
  "hello": {
    "labels": ["greeting", "demo"],
    "description": "打印问候信息",
    "executable": "echo",
    "args": ["Hello, World!"],
    "if_shell": false
  }
}
```

#### 创建插件

插件目录结构：

```plaintext
~/.config/yk/plugins/
└── myplugin.yk/
    ├── myplugin.yk.json  # 插件配置
    └── scripts/          # 插件脚本(自己随便搞没有什么要求)
```

#### 插件配置格式

```json
{
  "complex-command": {
    "labels": ["deployment", "production"],
    "description": "部署应用到生产环境",
    "executable": "/usr/local/bin/deploy.sh",
    "entry_point": "scripts/deploy.js",
    "args": ["--env", "production", "--verbose"],
    "if_shell": true
  }
}
```

#### 命令执行规则

- **简单命令**：`可执行文件 参数`
- **插件命令**：`可执行文件 插件入口文件 参数`
- **Shell执行**：`if_shell` 为 `true` 时，命令在 shell 中执行,在windows使用cmd,类unix系统使用bash

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [fzf](https://github.com/junegunn/fzf) - 模糊查找器
- [ripgrep](https://github.com/BurntSushi/ripgrep) - 快速搜索
- [serde](https://github.com/serde-rs/serde) - 序列化框架
