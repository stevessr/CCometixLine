use ccometixline::cli::Cli;
use ccometixline::config::{Config, InputData};
use ccometixline::core::{collect_all_segments, StatusLineGenerator};
use ccometixline::ui::{MainMenu, MenuResult};
use std::io::{self, IsTerminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse_args();

    if cli.config {
        ccometixline::ui::run_configurator()?;
        return Ok(());
    }

    // Handle Claude Code patcher
    if let Some(patch_arg) = cli.patch {
        use ccometixline::utils::ClaudeCodePatcher;

        // Convert CLI PatchLevel to patcher PatchLevel
        let patch_level = match cli.patch_level {
            ccometixline::cli::PatchLevel::Low => {
                ccometixline::utils::claude_code_patcher::PatchLevel::Low
            }
            ccometixline::cli::PatchLevel::Medium => {
                ccometixline::utils::claude_code_patcher::PatchLevel::Medium
            }
            ccometixline::cli::PatchLevel::High => {
                ccometixline::utils::claude_code_patcher::PatchLevel::High
            }
            ccometixline::cli::PatchLevel::Xhigh => {
                ccometixline::utils::claude_code_patcher::PatchLevel::Xhigh
            }
            ccometixline::cli::PatchLevel::Max => {
                ccometixline::utils::claude_code_patcher::PatchLevel::Max
            }
            ccometixline::cli::PatchLevel::Ultracode => {
                ccometixline::utils::claude_code_patcher::PatchLevel::Ultracode
            }
            ccometixline::cli::PatchLevel::Auto => {
                ccometixline::utils::claude_code_patcher::PatchLevel::Auto
            }
        };

        println!("🔧 Claude Code Patcher (Level: {:?})", patch_level);
        let claude_path = ClaudeCodePatcher::resolve_patch_target(patch_arg.as_deref())?;
        println!("Target file: {}", claude_path);

        // Create backup in same directory
        let backup_path = format!("{}.backup", claude_path);
        std::fs::copy(&claude_path, &backup_path)?;
        println!("📦 Created backup: {}", backup_path);

        // Load and patch
        let mut patcher = ClaudeCodePatcher::new(&claude_path)?;

        println!("\n🔄 Applying patches...");
        let results = patcher.apply_patches_with_level(patch_level);
        patcher.save()?;

        ClaudeCodePatcher::print_summary(&results);
        println!("💡 To restore original file, replace with the backup:");
        println!("   cp {} {}", backup_path, claude_path);

        return Ok(());
    }

    // Load configuration
    let mut config = Config::load().unwrap_or_else(|_| Config::default());

    // Apply theme override if provided
    if let Some(theme) = cli.theme {
        config = ccometixline::ui::themes::ThemePresets::get_theme(&theme);
    }

    // Check if stdin has data
    if io::stdin().is_terminal() {
        if let Some(result) = MainMenu::run()? {
            match result {
                MenuResult::LaunchConfigurator => {
                    ccometixline::ui::run_configurator()?;
                }
                MenuResult::InitConfig | MenuResult::CheckConfig => {}
                MenuResult::Exit => {}
            }
        }
        return Ok(());
    }

    // Read Claude Code data from stdin
    let stdin = io::stdin();
    let input: InputData = serde_json::from_reader(stdin.lock())?;

    // Collect segment data
    let segments_data = collect_all_segments(&config, &input);

    // Render statusline
    let generator = StatusLineGenerator::new(config);
    let statusline = generator.generate(segments_data);

    println!("{}", statusline);

    Ok(())
}
