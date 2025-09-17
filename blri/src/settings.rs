use colored::*;
use inquire::Confirm;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlriConfig {
    pub target: Option<String>,
    pub release: bool,
    pub package: Option<String>,
    pub binary_path: Option<String>, // Auto-generated from target/package/release
    pub port: Option<String>,
    pub baudrate: u32,
    pub reset: bool,
    pub console: bool,
}

impl Default for BlriConfig {
    fn default() -> Self {
        Self {
            target: None,
            release: false,
            package: None,
            binary_path: None,
            port: None,
            baudrate: 2000000,
            reset: false,
            console: false,
        }
    }
}

impl BlriConfig {
    /// Get the path to the settings file
    pub fn get_settings_path() -> Option<PathBuf> {
        std::env::current_dir()
            .ok()
            .map(|current| current.join("target").join("settings.toml"))
    }

    /// Load configuration from file
    pub fn load() -> Self {
        if let Some(path) = Self::get_settings_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str::<BlriConfig>(&content) {
                        println!(
                            "{} {}",
                            "📋 Loaded configuration:".bright_blue().bold(),
                            path.display()
                        );
                        Self::display_config(&config);
                        return config;
                    }
                }
            }
        }
        println!(
            "{}",
            "📋 No configuration found, using defaults".bright_yellow()
        );
        Self::default()
    }

    /// Save configuration to file
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::get_settings_path() {
            // Update binary path before saving
            self.update_binary_path();

            // Create target directory if it doesn't exist
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let content = toml::to_string_pretty(self)?;
            fs::write(&path, content)?;

            println!(
                "{} {}",
                "💾 Configuration saved to:".bright_green().bold(),
                path.display()
            );
            Self::display_config(self);
            Ok(())
        } else {
            Err("Cannot determine home directory".into())
        }
    }

    /// Display current configuration
    pub fn display_config(config: &BlriConfig) {
        println!(
            "  {}: {}",
            "Target".bright_cyan(),
            config.target.as_deref().unwrap_or("None").bright_white()
        );
        println!(
            "  {}: {}",
            "Release".bright_cyan(),
            if config.release {
                "Yes".bright_green()
            } else {
                "No".bright_red()
            }
        );
        println!(
            "  {}: {}",
            "Package".bright_cyan(),
            config.package.as_deref().unwrap_or("None").bright_white()
        );
        if let Some(binary_path) = &config.binary_path {
            println!(
                "  {}: {}",
                "Binary".bright_cyan(),
                binary_path.bright_white()
            );
        }
        println!(
            "  {}: {}",
            "Port".bright_cyan(),
            config.port.as_deref().unwrap_or("None").bright_white()
        );
        println!(
            "  {}: {}",
            "Baudrate".bright_cyan(),
            config.baudrate.to_string().bright_white()
        );
        println!(
            "  {}: {}",
            "Reset".bright_cyan(),
            if config.reset {
                "Yes".bright_green()
            } else {
                "No".bright_red()
            }
        );
        println!(
            "  {}: {}",
            "Console".bright_cyan(),
            if config.console {
                "Yes".bright_green()
            } else {
                "No".bright_red()
            }
        );
        println!();
    }

    /// Check if configuration is complete for running
    pub fn is_complete(&self) -> bool {
        self.target.is_some() && self.package.is_some() && self.port.is_some()
    }

    /// Generate binary path from target, package, and release mode
    pub fn generate_binary_path(&self) -> Option<PathBuf> {
        if let (Some(target), Some(package)) = (&self.target, &self.package) {
            let mode = if self.release { "release" } else { "debug" };

            // Detect if we're in the blri subdirectory or project root
            let current_dir = std::env::current_dir().ok()?;
            let is_in_blri_dir = current_dir
                .file_name()
                .map(|name| name == "blri")
                .unwrap_or(false);

            let path = if is_in_blri_dir {
                // From blri directory: ../target/...
                format!("../target/{}/{}/{}", target, mode, package)
            } else {
                // From project root: target/...
                format!("target/{}/{}/{}", target, mode, package)
            };

            Some(PathBuf::from(path))
        } else {
            None
        }
    }

    /// Update binary path based on current target, package, and release settings
    pub fn update_binary_path(&mut self) {
        self.binary_path = self
            .generate_binary_path()
            .map(|p| p.to_string_lossy().to_string());
    }

    /// Get the binary path as PathBuf, generating it if needed
    pub fn get_binary_path(&self) -> Option<PathBuf> {
        // First try the stored binary_path, but check if it exists
        if let Some(binary_path) = &self.binary_path {
            let path = PathBuf::from(binary_path);
            if path.exists() {
                return Some(path);
            }

            // If stored path doesn't exist, try converting between ../target and target
            let alternative_path = if binary_path.starts_with("../target/") {
                // Convert ../target to target (from blri dir to root dir)
                PathBuf::from(binary_path.strip_prefix("../").unwrap_or(binary_path))
            } else if binary_path.starts_with("target/") {
                // Convert target to ../target (from root dir to blri dir)
                PathBuf::from(format!("../{}", binary_path))
            } else {
                path
            };

            if alternative_path.exists() {
                return Some(alternative_path);
            }
        }

        // If stored path doesn't exist or is None, generate from components
        self.generate_binary_path()
    }

    /// Merge with command line arguments and ask whether to use current configuration
    /// Unified configuration management with two-step confirmation
    /// Returns (use_current_config, should_save_after_run)
    pub fn handle_configuration_conflict(
        &mut self,
        port: &str,
        baudrate: u32,
        target: Option<String>,
        release: bool,
        package: Option<String>,
        reset: bool,
        console: bool,
    ) -> Result<(bool, bool), Box<dyn std::error::Error>> {
        // Check for conflicts with current configuration
        let mut conflicts = Vec::new();

        if self.port.as_deref() != Some(port) {
            conflicts.push(format!(
                "Port: {} -> {}",
                self.port.as_deref().unwrap_or("None"),
                port
            ));
        }

        if self.baudrate != baudrate {
            conflicts.push(format!("Baudrate: {} -> {}", self.baudrate, baudrate));
        }

        if let Some(new_target) = &target {
            if self.target.as_ref() != Some(new_target) {
                conflicts.push(format!(
                    "Target: {} -> {}",
                    self.target.as_deref().unwrap_or("None"),
                    new_target
                ));
            }
        }

        if self.release != release {
            conflicts.push(format!("Release: {} -> {}", self.release, release));
        }

        if let Some(new_package) = &package {
            if self.package.as_ref() != Some(new_package) {
                conflicts.push(format!(
                    "Package: {} -> {}",
                    self.package.as_deref().unwrap_or("None"),
                    new_package
                ));
            }
        }

        if self.reset != reset {
            conflicts.push(format!("Reset: {} -> {}", self.reset, reset));
        }

        if self.console != console {
            conflicts.push(format!("Console: {} -> {}", self.console, console));
        }

        // If there are conflicts, ask first confirmation
        if !conflicts.is_empty() {
            println!();
            println!(
                "{}",
                "⚠️  Configuration conflicts detected:"
                    .bright_yellow()
                    .bold()
            );
            for conflict in &conflicts {
                println!("  {}", conflict.bright_white());
            }
            println!();

            let use_current =
                Confirm::new("Use current configuration instead of saved configuration?")
                    .with_default(true)
                    .prompt()?;

            if use_current {
                // User wants to use current config, prepare for potential save after run
                return Ok((true, true));
            } else {
                // User wants to use saved config, no save needed
                return Ok((false, false));
            }
        }

        // No conflicts, use current config and maybe save if it's new
        Ok((true, false))
    }

    /// Save current configuration after successful run (second confirmation)
    pub fn save_after_run(
        &mut self,
        port: &str,
        baudrate: u32,
        target: Option<String>,
        release: bool,
        package: Option<String>,
        reset: bool,
        console: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Update configuration with current values
        self.port = Some(port.to_string());
        self.baudrate = baudrate;
        self.target = target;
        self.release = release;
        self.package = package;
        self.reset = reset;
        self.console = console;
        self.update_binary_path();

        println!();
        let should_save = Confirm::new("Save current configuration for future use?")
            .with_default(true)
            .prompt()?;

        if should_save {
            self.save()?;
            println!("{}", "✅ Configuration saved successfully!".bright_green());
        }

        Ok(())
    }
}
