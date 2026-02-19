use colored::{ColoredString, Colorize};
use once_cell::unsync::Lazy;

pub const WARNING_YELLOW_BOLD: Lazy<ColoredString> = Lazy::new(|| "WARNING:".yellow().bold());
pub const ERROR_RED_BOLD: Lazy<ColoredString> = Lazy::new(|| "ERROR:".red().bold());
