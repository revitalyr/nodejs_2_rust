/// Ethereum Boilerplate Utilities
//
/// Common utilities and helper functions used across all crates.

use colored::Colorize; // NOTE: This import is necessary for .bright_cyan() and other color methods
use ethereum_boilerplate_shared::{cli::TITLE, cli::SUBTITLE, cli::VERSION, cli::PROGRESS_BAR_TEMPLATE, cli::PROGRESS_CHARS, cli::BANNER_LINE_LENGTH};

pub mod crypto;
pub mod error;
pub mod formatting;
pub mod validation;
pub mod config;
pub mod logging;
pub mod network;
pub mod async_utils;

// Re-export commonly used items
pub use crate::error::{Result, UtilsError};
pub use crate::config::Config;
pub use crate::logging::init_logging;
pub use crate::validation::{validate_address, validate_private_key, validate_amount};
pub use crate::formatting::{format_eth, format_wei, parse_eth, parse_wei, format_address_display};



// --- CLI Display Utilities ---

/// Prints beautiful banner in terminal
pub fn print_banner() {
    let title = format!("{} {}", TITLE, VERSION).bright_cyan().bold();
    let subtitle = SUBTITLE.bright_black();

    println!(
        "╔{line}╗\n║{title:^72}║\n║{subtitle:^69}║\n╚{line}╝",
        line = "═".repeat(BANNER_LINE_LENGTH),
        title = title,
        subtitle = subtitle
    );
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✅".green(), msg.bold());
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "❌".red(), msg.bright_red());
}

/// Creates configured progress bar for CLI tasks
pub fn create_progress_bar(total: u64) -> indicatif::ProgressBar {
    use indicatif::{ProgressBar, ProgressStyle};

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_BAR_TEMPLATE)
            .expect("Invalid progress bar template")
            .progress_chars(PROGRESS_CHARS)
    );
    pb
}