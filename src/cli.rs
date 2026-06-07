use clap::Parser;

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
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
