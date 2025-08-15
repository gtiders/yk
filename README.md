# YK - Command Management Tool

[🇨🇳 中文说明](README_ZH.md) | [🇺🇸 English](README.md)

The original repository at <https://github.com/gybwins/yk.git> has been banned for unknown reasons and remains unblocked. Future updates will be made in the current repository; the main repository for updates is <https://gitee.com/bobo-wins/yk.git>

---

YK is a command management tool that supports both simple commands and complex plugin-based commands through an interactive interface. Quickly find, execute, and manage various command snippets to make your daily workflow more efficient.

## Core Features

- **Zero Configuration**: Works out of the box without any setup required
- **Interactive Search**: Fuzzy finding powered by fzf for quick command location
- **Plugin Architecture**: Supports plugin extensions for complex commands to handle various scenarios
- **Clipboard Integration**: Automatically copies commands to clipboard for immediate use
- **Cross-Platform**: Perfect compatibility with Windows, Linux, and macOS
- **Visual Configuration**: Flexible JSON-based configuration that's easy to manage and share

## Build & Install

### Requirements

- **Rust**: 1.70 or higher
- **Cargo**: Rust's package manager

### Build from Source

#### 1. Clone Repository

```bash
git clone git@github.com:gtiders/yk.git
# Or
https://github.com/gtiders/yk.git

cd yk
```

#### 2. Build Release Version

```bash
cargo build --release
```

After compilation, the executable will be located at:

- **Windows**: `target\release\yk.exe`
- **Linux/macOS**: `target/release/yk`

#### 3. Add to System PATH

**Windows (PowerShell):**

```powershell
# Method 1: Copy to system directory
copy target\release\yk.exe C:\Windows\System32\

# Method 2: Add to user PATH (recommended)
$env:PATH += ";$PWD\target\release"
```

**Linux/macOS:**

```bash
# Method 1: Install to system directory
sudo cp target/release/yk /usr/local/bin/
sudo chmod +x /usr/local/bin/yk

# Method 2: Add to user PATH (recommended)
export PATH="$PATH:$PWD/target/release"
# Add to ~/.bashrc or ~/.zshrc for permanent effect
echo 'export PATH="$PATH:$PWD/target/release"' >> ~/.bashrc
```

### Verify Installation

```bash
yk --help
```

## Quick Start

### 1. Initialize Configuration

First-time YK usage requires initialization:

```bash
yk init
```

This creates the following directory structure:

```plaintext
~/.config/yk/
├── config.json          # Main configuration file
├── simple_commands.json # Simple commands configuration
└── plugins/             # Plugins directory
```

### 2. Create Your First Command

Create a simple command interactively:

```bash
yk new
```

Follow the prompts:

- **Command Name**: `hello`
- **Labels**: `demo greeting`
- **Description**: `Print greeting message`
- **Executable**: `echo`
- **Arguments**: `Hello, YK!`
- **Shell Execution**: `n`

### 3. Use Commands

Run YK to enter the interactive interface:

```bash
yk
```

In the interactive interface:

- Type `hello` or `demo` to search
- Press `Enter` to execute selected command
- Press `Esc` to exit

### 4. Command Line Options

YK supports the following commands:

| Command | Description |
|---------|-------------|
| `yk` or `yk find` | Find and execute commands |
| `yk new` | Create new command |
| `yk init` | Initialize configuration |
| `yk --help` | Display help information |

## Configuration

### Main Configuration File

File path: `~/.config/yk/config.json`

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

#### Configuration Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `fzf_executable` | string | "fzf" | fzf executable path |
| `rg_executable` | string | "rg" | ripgrep executable path |
| `editor` | string | "hx" | Text editor path |
| `if_run` | boolean | true | Whether to execute selected command |
| `if_run_confirm` | boolean | true | Whether to confirm before execution |
| `if_yank` | boolean | true | Whether to copy to clipboard |

## Command Management

### Complete Command Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `labels` | array | [] | Command labels |
| `description` | string | "" | Command description |
| `executable` | string | "" | Executable file path |
| `entry_point` | string | "" | Plugin entry file path relative to plugin directory |
| `args` | array | [] | Command arguments |
| `if_shell` | boolean | false | Whether to execute in shell |

### Creating Simple Commands or Plugins

Simple commands can be created interactively using `yk new` by following the prompts, or by manually editing the corresponding files.

#### Example Configuration

```json
{
  "hello": {
    "labels": ["greeting", "demo"],
    "description": "Print greeting message",
    "executable": "echo",
    "args": ["Hello, World!"],
    "if_shell": false
  }
}
```

#### Creating Plugins

Plugin directory structure:

```plaintext
~/.config/yk/plugins/
└── myplugin.yk/
    ├── myplugin.yk.json  # Plugin configuration
    └── scripts/          # Plugin scripts (no specific requirements)
```

#### Plugin Configuration Format

```json
{
  "complex-command": {
    "labels": ["deployment", "production"],
    "description": "Deploy application to production environment",
    "executable": "/usr/local/bin/deploy.sh",
    "entry_point": "scripts/deploy.js",
    "args": ["--env", "production", "--verbose"],
    "if_shell": true
  }
}
```

#### Command Execution Rules

- **Simple Commands**: `executable arguments`
- **Plugin Commands**: `executable plugin_entry_file arguments`
- **Shell Execution**: When `if_shell` is `true`, commands execute in shell (cmd on Windows, bash on Unix-like systems)

## License

MIT License - See [LICENSE](LICENSE) file for details

## Acknowledgments

- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [fzf](https://github.com/junegunn/fzf) - Fuzzy finder
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast search
- [serde](https://github.com/serde-rs/serde) - Serialization framework
