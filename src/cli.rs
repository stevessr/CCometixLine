use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PatchLevel {
    /// Minimal patches (token counter, context warnings)
    Low,
    /// Low + ESC interrupt display
    Medium,
    /// Medium + Chrome subscription features
    High,
    /// High + Ultracode patches (dynamic workflow + xhigh model gate)
    Xhigh,
    /// All patches (same as Xhigh currently)
    Max,
    /// Only Ultracode-specific patches (dynamic workflow + xhigh model gate)
    Ultracode,
    /// Auto-detect recommended level (defaults to high)
    Auto,
}

impl Default for PatchLevel {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Parser, Debug)]
#[command(name = "ccline")]
#[command(version, about = "High-performance Claude Code StatusLine")]
pub struct Cli {
    /// Enter TUI configuration mode
    #[arg(short = 'c', long = "config")]
    pub config: bool,

    /// Set theme
    #[arg(short = 't', long = "theme")]
    pub theme: Option<String>,

    /// Patch Claude Code cli.js to disable context warnings
    ///
    /// If no path is provided, ccline auto-detects `claude` from PATH.
    #[arg(long = "patch", value_name = "CLI_JS_OR_SHIM", num_args = 0..=1)]
    pub patch: Option<Option<String>>,

    /// Patch level: low, medium, high, xhigh, max, ultracode, or auto
    #[arg(long = "patch-level", value_enum, default_value = "auto")]
    pub patch_level: PatchLevel,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
