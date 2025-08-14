use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::{Parser, Subcommand};
use clipboard::{ClipboardContext, ClipboardProvider};
use serde::{Deserialize, Serialize};

// Define configuration file constants
const CONFIG_DIR_NAME: &str = ".config/yk";
const CONFIG_FILE_NAME: &str = "config.json";
const PLUGINS_DIR_NAME: &str = "plugins";
const SIMPLE_COMMAND_FILE_NAME: &str = "simple_commands.json";

// Define delimiter constants
const FZF_DELIMITER: &str = "🍃────🍃";

/// Get configuration directory path
fn get_config_dir() -> PathBuf {
    let home = env::home_dir().expect("Failed to get home directory");
    home.join(CONFIG_DIR_NAME)
}

/// Get configuration file path
fn get_config_file() -> PathBuf {
    get_config_dir().join(CONFIG_FILE_NAME)
}

/// Get plugins directory path
fn get_plugins_dir() -> PathBuf {
    get_config_dir().join(PLUGINS_DIR_NAME)
}

/// Get simple commands configuration file path
fn get_simple_command_file() -> PathBuf {
    get_config_dir().join(SIMPLE_COMMAND_FILE_NAME)
}

/// YK configuration file data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct YkConfig {
    pub fzf_executable: PathBuf,
    pub rg_executable: PathBuf,
    pub editor: PathBuf,
    pub if_run: bool,
    pub if_run_confirm: bool,
    pub if_yank: bool,
}

impl Default for YkConfig {
    fn default() -> Self {
        Self {
            fzf_executable: PathBuf::from("fzf"),
            rg_executable: PathBuf::from("rg"),
            editor: PathBuf::from("hx"),
            if_run: true,
            if_run_confirm: true,
            if_yank: true,
        }
    }
}

/// Command snippet data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSnippet {
    #[serde(default)]
    pub labels: Vec<String>,
    pub description: Option<String>,
    pub executable: Option<PathBuf>,
    pub entry_point: Option<PathBuf>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub if_shell: bool,
}

/// Single plugin data structure
#[derive(Debug, Clone)]
pub struct Plugin {
    pub plugin_dir: PathBuf,
    pub config_file: PathBuf,
    pub commands: HashMap<String, CommandSnippet>,
}

/// Complete command snippet data structure
#[derive(Debug, Clone)]
pub struct FullCommandSnippet {
    pub name: String,
    pub complete_command: String,
    pub config_file: PathBuf,
    pub labels: Vec<String>,
    pub description: Option<String>,
    pub executable: Option<PathBuf>,
    pub entry_point: Option<PathBuf>,
    pub args: Vec<String>,
    pub if_shell: bool,
}

/// All command snippets data structure
#[derive(Debug, Clone)]
pub struct AllCommandSnippets {
    pub commands: Vec<FullCommandSnippet>,
}

impl Default for AllCommandSnippets {
    fn default() -> Self {
        Self::new()
    }
}

impl AllCommandSnippets {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

/// Initialize configuration files and directories
pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = get_config_dir();
    let plugins_dir = get_plugins_dir();
    let config_file = get_config_file();
    let simple_command_file = get_simple_command_file();

    // Create configuration directories
    fs::create_dir_all(&config_dir)?;
    fs::create_dir_all(&plugins_dir)?;

    // Handle configuration file
    if !config_file.exists() {
        let config = YkConfig::default();
        let serialized = serde_json::to_string_pretty(&config)?;
        fs::write(&config_file, serialized)?;
        println!("Created default configuration file: {:?}", config_file);
    } else {
        print!(
            "Configuration file {:?} already exists, overwrite? (y/N): ",
            config_file
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            let config = YkConfig::default();
            let serialized = serde_json::to_string_pretty(&config)?;
            fs::write(&config_file, serialized)?;
            println!("Configuration file updated");
        } else {
            println!("Skipped configuration file update");
        }
    }

    // Create simple commands configuration file
    if !simple_command_file.exists() {
        fs::write(&simple_command_file, "{}")?;
        println!(
            "Created simple commands configuration file: {:?}",
            simple_command_file
        );
    }

    Ok(())
}

/// Load YK configuration file
pub fn load_config() -> Result<YkConfig, Box<dyn std::error::Error>> {
    let config_file = get_config_file();

    if !config_file.exists() {
        println!("Configuration file {:?} does not exist", config_file);
        return Err("Configuration file does not exist".into());
    }

    let file_content = fs::read_to_string(&config_file)?;
    let config: YkConfig = serde_json::from_str(&file_content)?;
    Ok(config)
}

/// Create new simple command
pub fn new_command() -> Result<(), Box<dyn std::error::Error>> {
    let simple_command_file = get_simple_command_file();

    // Interactive input
    print!("Enter command name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim();
    if name.is_empty() {
        return Err("Command name cannot be empty".into());
    }

    print!("Enter command labels (space-separated): ");
    io::stdout().flush()?;
    let mut labels_input = String::new();
    io::stdin().read_line(&mut labels_input)?;
    let labels: Vec<String> = labels_input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    print!("Enter command description: ");
    io::stdout().flush()?;
    let mut description = String::new();
    io::stdin().read_line(&mut description)?;
    let description = description.trim();
    let description = if description.is_empty() {
        None
    } else {
        Some(description.to_string())
    };

    print!("Enter executable path (optional): ");
    io::stdout().flush()?;
    let mut executable = String::new();
    io::stdin().read_line(&mut executable)?;
    let executable = executable.trim();
    let executable = if executable.is_empty() {
        None
    } else {
        Some(PathBuf::from(executable))
    };

    print!("Simple commands do not have entry points!");
    let entry_point = None;

    print!("Enter command arguments (space-separated, optional): ");
    io::stdout().flush()?;
    let mut args_input = String::new();
    io::stdin().read_line(&mut args_input)?;
    let args: Vec<String> = args_input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    print!("Execute in shell? (y/N): ");
    io::stdout().flush()?;
    let mut if_shell_input = String::new();
    io::stdin().read_line(&mut if_shell_input)?;
    let if_shell = if_shell_input.trim().to_lowercase() == "y";

    // Create CommandSnippet object
    let command_snippet = CommandSnippet {
        labels,
        description,
        executable,
        entry_point,
        args,
        if_shell,
    };

    // Load existing configuration
    let mut config: HashMap<String, CommandSnippet> = if simple_command_file.exists() {
        let file_content = fs::read_to_string(&simple_command_file)?;
        if file_content.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&file_content)?
        }
    } else {
        HashMap::new()
    };

    // Check if command name already exists
    if config.contains_key(name) {
        print!("Command '{}' already exists, overwrite? (y/N): ", name);
        io::stdout().flush()?;
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm)?;
        if confirm.trim().to_lowercase() != "y" {
            println!("Cancelled creation of command '{}'", name);
            return Ok(());
        }
    }

    // Add new command
    config.insert(name.to_string(), command_snippet);

    // Save configuration
    let serialized = serde_json::to_string_pretty(&config)?;
    fs::write(&simple_command_file, serialized)?;

    println!(
        "Created simple command '{}' and saved to {:?}",
        name, simple_command_file
    );

    Ok(())
}

/// Generic function to load commands from configuration file
pub fn load_commands_from_config(
    config_file: &PathBuf,
    source_name: &str,
) -> Result<Option<Plugin>, Box<dyn std::error::Error>> {
    if !config_file.exists() {
        println!(
            "Warning: {} configuration file {:?} does not exist",
            source_name, config_file
        );
        return Ok(None);
    }

    if !config_file.is_file() {
        println!(
            "Warning: {} configuration file {:?} is not a valid file",
            source_name, config_file
        );
        return Ok(None);
    }

    let base_dir = config_file
        .parent()
        .ok_or("Failed to get configuration file directory")?;

    let file_content = fs::read_to_string(config_file)?;
    let data: serde_json::Value = serde_json::from_str(&file_content)?;

    let commands_map = match data {
        serde_json::Value::Object(map) => map,
        _ => {
            println!(
                "Warning: {} configuration file format error: expected object format",
                source_name
            );
            return Ok(None);
        }
    };

    let mut commands = HashMap::new();
    for (name, cmd_data) in commands_map {
        if let Ok(mut cmd_snippet) = serde_json::from_value::<CommandSnippet>(cmd_data) {
            // Convert entry_point to absolute path relative to configuration file directory
            if let Some(ref mut entry_point) = cmd_snippet.entry_point {
                if entry_point.is_relative() {
                    *entry_point = base_dir.join(&entry_point);
                }
            }

            commands.insert(name, cmd_snippet);
        } else {
            println!("Warning: {} command '{}' format error", source_name, name);
        }
    }

    if commands.is_empty() {
        println!(
            "Warning: {} configuration file has no valid commands",
            source_name
        );
        return Ok(None);
    }

    Ok(Some(Plugin {
        plugin_dir: base_dir.to_path_buf(),
        config_file: config_file.clone(),
        commands,
    }))
}

/// Load simple commands plugin
pub fn load_simple_plugin() -> Result<Option<Plugin>, Box<dyn std::error::Error>> {
    let simple_command_file = get_simple_command_file();
    load_commands_from_config(&simple_command_file, "simple commands")
}

/// Load all plugins
pub fn load_plugins() -> Result<Vec<Plugin>, Box<dyn std::error::Error>> {
    let plugins_dir = get_plugins_dir();
    let mut plugins = Vec::new();

    if !plugins_dir.exists() {
        return Ok(plugins);
    }

    if let Ok(entries) = fs::read_dir(&plugins_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let plugin_dir = entry.path();
                let config_file =
                    plugin_dir.join(format!("{}.json", entry.file_name().to_string_lossy()));

                if let Ok(Some(plugin)) = load_commands_from_config(
                    &config_file,
                    &format!("plugin {}", entry.file_name().to_string_lossy()),
                ) {
                    plugins.push(plugin);
                }
            }
        }
    }

    Ok(plugins)
}

/// Load all commands
pub fn load_commands() -> Result<AllCommandSnippets, Box<dyn std::error::Error>> {
    let simple_plugin = load_simple_plugin()?;
    let plugins = load_plugins()?;

    let mut all_plugins = Vec::new();
    if let Some(plugin) = simple_plugin {
        all_plugins.push(plugin);
    }
    all_plugins.extend(plugins);

    let mut all_commands = AllCommandSnippets::new();

    for plugin in all_plugins {
        for (name, cmd_snippet) in plugin.commands {
            let mut command_parts = Vec::new();

            if let Some(ref executable) = cmd_snippet.executable {
                command_parts.push(executable.to_string_lossy().to_string());
            }
            if let Some(ref entry_point) = cmd_snippet.entry_point {
                command_parts.push(entry_point.to_string_lossy().to_string());
            }
            if !cmd_snippet.args.is_empty() {
                command_parts.extend(cmd_snippet.args.clone());
            }

            let complete_command = if command_parts.is_empty() {
                String::new()
            } else {
                command_parts.join(" ")
            };

            all_commands.commands.push(FullCommandSnippet {
                name,
                complete_command,
                config_file: plugin.config_file.clone(),
                labels: cmd_snippet.labels,
                description: cmd_snippet.description,
                executable: cmd_snippet.executable,
                entry_point: cmd_snippet.entry_point,
                args: cmd_snippet.args,
                if_shell: cmd_snippet.if_shell,
            });
        }
    }

    Ok(all_commands)
}

/// Find all commands and perform operations
pub fn find_command(edit: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let commands = load_commands()?;

    if commands.commands.is_empty() {
        println!("No commands found");
        return Ok(());
    }

    // Build fzf input
    let mut fzf_input = Vec::new();
    for (index, command) in commands.commands.iter().enumerate() {
        let executable_str = command
            .executable
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "None".to_string());

        let labels_str = if command.labels.is_empty() {
            "No labels".to_string()
        } else {
            command.labels.join(" ")
        };

        let line = format!(
            "{}: {}{}{}{}{}{}{}",
            index,
            executable_str,
            FZF_DELIMITER,
            command.name,
            FZF_DELIMITER,
            labels_str,
            FZF_DELIMITER,
            command.config_file.to_string_lossy()
        );
        fzf_input.push(line);
    }

    // Create fzf command, using path from configuration
    let mut fzf_cmd = Command::new(&config.fzf_executable)
        .arg(format!("--delimiter={}", FZF_DELIMITER))
        .arg("--with-nth=1,2,3")
        .arg("--border")
        .arg("--cycle")
        .arg("--prompt=Select a command 🔍: ")
        .arg(format!(
            "--preview={} --color=always -A 20 {{2}} {{4}}",
            config.rg_executable.to_string_lossy()
        ))
        .arg("--preview-window=right:45%")
        .arg("--bind=esc:abort,ctrl-c:abort")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write data to fzf
    if let Some(stdin) = &mut fzf_cmd.stdin {
        for line in &fzf_input {
            use std::io::Write;
            writeln!(stdin, "{}", line)?;
        }
    }

    // Get fzf output
    let output = fzf_cmd.wait_with_output()?;

    if !output.status.success() {
        println!("User cancelled selection or fzf execution failed");
        return Ok(());
    }

    let selected_line = String::from_utf8_lossy(&output.stdout);
    let selected_line = selected_line.trim();

    if selected_line.is_empty() {
        println!("No command selected");
        return Ok(());
    }

    // Parse selected command index
    let parts: Vec<&str> = selected_line.split(FZF_DELIMITER).collect();

    if parts.is_empty() {
        println!("Invalid selection format: {}", selected_line);
        return Ok(());
    }

    // Extract index - first part should be "index: executable"
    let first_part = parts[0];
    let index_str = first_part.split(':').next().unwrap_or("").trim();

    let index: usize = match index_str.parse::<usize>() {
        Ok(num) if num < commands.commands.len() => num,
        Ok(num) => {
            println!(
                "Index {} out of range, valid range is 0-{}, total commands: {}",
                num,
                commands.commands.len() - 1,
                commands.commands.len()
            );
            return Ok(());
        }
        Err(e) => {
            println!(
                "Failed to parse index: {}, original value: '{}'",
                e, index_str
            );
            return Ok(());
        }
    };

    let selected_command = &commands.commands[index];

    // If edit parameter is enabled, open editor
    if edit {
        println!("Editing command: {}", selected_command.name);
        println!("Current command: {}", selected_command.complete_command);

        let config_path = &selected_command.config_file;
        if config_path.exists() {
            println!("Opening configuration file: {:?}", config_path);
            let status = std::process::Command::new(&config.editor)
                .arg(config_path)
                .status()?;

            if status.success() {
                println!("Configuration file opened, please edit and save");
            } else {
                println!("Failed to open configuration file");
            }
        } else {
            println!("Configuration file does not exist: {:?}", config_path);
        }
        return Ok(());
    }

    // Copy to clipboard
    if config.if_yank {
        let mut ctx: ClipboardContext = ClipboardProvider::new()
            .map_err(|e| format!("Clipboard initialization failed: {}", e))?;

        ctx.set_contents(selected_command.complete_command.clone())
            .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;

        println!("Copied to clipboard: {}", selected_command.complete_command);
    }

    // Execute command
    if config.if_run {
        if config.if_run_confirm {
            print!(
                "Confirm to run command: {} (y/N): ",
                selected_command.complete_command
            );
            io::stdout().flush()?;

            let mut confirm = String::new();
            io::stdin().read_line(&mut confirm)?;

            if confirm.trim().to_lowercase() != "y" {
                println!("Execution cancelled");
                return Ok(());
            }
        }

        // Execute command
        if selected_command.if_shell {
            let status = if cfg!(target_os = "windows") {
                std::process::Command::new("cmd")
                    .arg("/C")
                    .arg(&selected_command.complete_command)
                    .status()?
            } else {
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&selected_command.complete_command)
                    .status()?
            };

            if !status.success() {
                println!("Command execution failed");
            }
        } else {
            let mut parts = selected_command.complete_command.split_whitespace();
            if let Some(program) = parts.next() {
                let args: Vec<&str> = parts.collect();
                let status = std::process::Command::new(program).args(&args).status()?;

                if !status.success() {
                    println!("Command execution failed");
                }
            }
        }
    }

    Ok(())
}

/// CLI parameter structure
#[derive(Parser)]
#[command(name = "yk")]
#[command(about = "Simple commands and complex commands plugin management tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Subcommand definitions
#[derive(Subcommand)]
enum Commands {
    /// Initialize configuration file
    Init,
    /// Find all commands and perform operations
    Find {
        /// Edit command configuration file
        #[arg(short, long)]
        edit: bool,
    },
    /// Create new simple command
    New,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => init_config()?, // Initialize configuration file
        Some(Commands::New) => new_command()?,  // Create new command
        Some(Commands::Find { edit }) => find_command(edit)?, // Find and execute commands
        None => find_command(false)?,           // Default execute find command
    }

    Ok(())
}
