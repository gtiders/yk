# YK - 命令管理工具

原仓库在<https://github.com/gybwins/yk.git>  因不知名原因被封禁，仍未解封，以后更新会在现在这个仓库；更新主要仓库是 <https://gitee.com/bobo-wins/yk.git>

---
YK 是一个命令管理工具，支持简单命令和复杂插件化命令的交互式管理。通过简洁的交互界面，您可以快速查找、执行和管理各种命令片段，让日常工作流程更加高效。

## 核心特性

- **零配置启动**：开箱即用，无需任何配置即可开始使用
- **交互式搜索**：基于 fzf 的模糊查找，快速定位所需命令
- **插件化架构**：支持复杂命令的插件扩展，灵活应对各种场景
- **剪贴板集成**：自动复制命令到剪贴板，即选即用
- **跨平台支持**：完美兼容 Windows、Linux 和 macOS
- **可视化配置**：通过 JSON 文件灵活配置，易于管理和分享

## 编译安装

### 环境要求

- **Rust**: 1.70 或更高版本
- **Cargo**: Rust 的包管理器

### 从源码编译

#### 1. 克隆仓库

```bash
git clone git@gitee.com:bobo-wins/yk.git
# 或者
git clone https://gitee.com/bobo-wins/yk.git

cd yk
```

#### 2. 编译发布版本

```bash
cargo build --release
```

编译完成后，可执行文件位置：

- **Windows**: `target\release\yk.exe`
- **Linux/macOS**: `target/release/yk`

#### 3. 添加到系统 PATH

**Windows (PowerShell):**

```powershell
# 方法1: 复制到系统目录
copy target\release\yk.exe C:\Windows\System32\

# 方法2: 添加到用户PATH(推荐)
$env:PATH += ";$PWD\target\release"
```

**Linux/macOS:**

```bash
# 方法1: 安装到系统目录
sudo cp target/release/yk /usr/local/bin/
sudo chmod +x /usr/local/bin/yk

# 方法2: 添加到用户PATH(推荐)
export PATH="$PATH:$PWD/target/release"
# 添加到 ~/.bashrc 或 ~/.zshrc 使其永久生效
echo 'export PATH="$PATH:$PWD/target/release"' >> ~/.bashrc
```

### 验证安装

```bash
yk --help
```

## 快速开始

### 1. 初始化配置

首次使用 YK 时，需要初始化配置：

```bash
yk init
```

这将创建以下目录结构：

```plaintext

~/.config/yk/
├── config.json          # 主配置文件
├── simple_commands.json # 简单命令配置
└── plugins/             # 插件目录
```

### 2. 创建第一个命令

使用交互式方式创建简单命令：

```bash
yk new
```

按提示填写即可,创建完成后,会在 `~/.config/yk/simple_commands.json` 中添加一条记录.

### 4.1 插件化命令

插件化命令可以实现更复杂的功能,如执行 shell 脚本、调用外部程序等.
插件化命令的配置文件位于 `~/.config/yk/plugins/` 目录下,每个插件对应单独文件夹,
文件夹名称为插件名称必须使用`.yk`结尾,插件化命令的配置文件为一个json文件,名字是插件文件夹加上`.json`,格式如下:
目录结构如下:

```plaintext

~/.config/yk/plugins/
├── plugin1.yk/
│   └── plugin1.yk.json

├── plugin2.yk/
│   └── plugin2.yk.json
```

### 命令格式的统一说明(simple_commands.json或是任意插件化命令的json文件)

```json
{
 "command1":{
    labels:["cmd1","cmd2"],
    description:"这是一个简单的命令",
    executable: "一般是所需允许的二进制文件的路径",
    entry_point:"脚本的路径或者其他什么需要,位置应该在插件目录下,运行时会被自动转为绝对路径",
    args:["args1","args2"],
    if_shell: false
 }
"command2":{
    labels:["cmd3","cmd4"],
    description:"这是一个简单的命令",
    executable: "一般是所需允许的二进制文件的路径",
    entry_point:"脚本的路径或者其他什么需要,位置应该在插件目录下,运行时会被自动转为绝对路径",
    args:["args1","args2"],
    if_shell: false
}
}
```

- `labels`: 命令的标签,可以是多个,用于搜索.
- `description`: 命令的描述,用于显示在搜索结果中.
- `executable`: 可执行文件的路径
- `entry_point`: 命令的入口点,一般是应对复杂的命令,或者直接运行一些脚本(简单命令默认是没有这个的,但是可以手动添加)
- `args`: 命令的参数,可以是多个,每个参数都是一个字符串.
- `if_shell`: 是否通过 shell 执行命令,在windows下使用cmd执行,在其他系统下使用bash执行.

### 3. 使用命令

运行 YK 进入交互界面：

```bash
yk # or yk find
```

在交互界面中：

- 输入 `hello` 或 `demo` 进行搜索
- 按 `Enter` 执行选中的命令(行为取决于yk的配置文件)

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

- `fzf_executable`: fzf 可执行文件的路径(如果在环境变量那么可以不设置)
- `rg_executable`: rg 可执行文件的路径(如果在环境变量那么可以不设置)
- `editor`: 编辑器的路径(默认是hx(helix),可以使用其他的,只在`yk find -e`命令中使用,会直接打开选中命令的配置文件以供编辑)
- `if_run`: 是否运行选中的命令
- `if_run_confirm`: 是否在运行命令前确认
- `if_yank`: 是否在命令运行前复制到剪贴板
- 按 `Esc` 退出

### 4. 命令行参数

YK 支持以下命令：

| 命令 | 说明 |
|------|------|
| `yk` 或 `yk find` | 查找并执行命令 |
| `yk find -e` | 编辑选中的命令 |
| `yk new` | 创建新命令(只能创建简单命令) |
| `yk init` | 初始化配置 |
| `yk --help` | 显示帮助信息 |

## 一个典型的配置文件

```json
{
    "scp-down": {
        "labels": [
            "scp",
            "download",
            "win",
        ],
        "description": "在windows系统上使用scp进行文件下载",
        "executable": "nu",
        "entry_point": "scp-down.nu",
        "args": [
            "--help"
        ],
        "if_shell": false
    },
    "scp-up": {
        "labels": [
            "scp",
            "upload",
            "win",
        ],
        "description": "在windows系统上使用scp进行文件上传",
        "executable": "nu",
        "entry_point": "scp-up.nu",
        "args": [
            "--help"
        ],
        "if_shell": false
    },
    "scp-hello": {
        "labels": [
            "scp",
            "hello",
            "win",
        ],
        "description": "打印一个hello scp",
        "executable": null,
        "entry_point": null,
        "args": [
            "echo",
            "hello scp"
        ],
        "if_shell": true
    }
}
上述是一个典型的配置文件,其包含三个命令,分别是scp-up-down,scp-up,scp-hello,其最后组合完成后的完整命令是:
- `nu ~/.config/yk/plugins/scp-hello.yk/scp-hello.nu --help`
- `nu ~/.config/yk/plugins/scp-up.yk/scp-up.nu --help`
- `echo hello scp`

上述`~`其实也会被展开为用户的home目录,如`C:\Users\me`或者`/home/me/`
`scp-down`和`scp-up`命令的参数是一样的,是典型的需要有entry_point的复杂命令,命令的具体实现逻辑在`scp-down.nu`和`scp-up.nu`中,通过nushell进行执行调用
`scp-hello`命令是一个简单命令,没有entry_point,直接通过系统的shell(if_shell=true) 执行`echo hello scp`


## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [clap](https://github.com/clap-rs/clap) - 命令行参数解析
- [fzf](https://github.com/junegunn/fzf) - 模糊查找器
- [ripgrep](https://github.com/BurntSushi/ripgrep) - 快速搜索
- [serde](https://github.com/serde-rs/serde) - 序列化框架






