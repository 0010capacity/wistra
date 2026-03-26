pub mod config;
pub mod onboard;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run the onboarding wizard to set up wistra
    Onboard,

    /// Run the wiki growth process
    Run {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Number of concepts to generate
        #[arg(short, long, default_value = "5")]
        count: usize,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,

        /// Quiet mode - minimal output
        #[arg(short, long)]
        quiet: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        no_confirm: bool,
    },

    /// Scan wiki and print report
    Scan {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,
    },

    /// Modify configuration
    Config {
        /// Re-run onboarding
        #[arg(short, long)]
        onboard: bool,
    },

    /// Print wiki status summary
    Status {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,
    },

    /// Modify interest domains
    Interests,
}
