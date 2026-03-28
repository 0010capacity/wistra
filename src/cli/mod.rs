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

    /// Rename a document and update all links
    Rename {
        /// Old title of the document
        old_title: String,

        /// New title for the document
        new_title: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,
    },

    /// Merge two documents into one
    Merge {
        /// Title of document to merge (will be deleted)
        source: String,

        /// Title of target document (will be kept)
        target: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,
    },

    /// Delete a document and clean up links
    Delete {
        /// Title of the document to delete
        title: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,

        /// Skip confirmation prompt
        #[arg(long)]
        no_confirm: bool,
    },

    /// Show documents that link to a specific document (backlinks)
    Backlinks {
        /// Target document title
        title: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Find orphaned documents (no incoming links)
    Orphans {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Exclude stub documents
        #[arg(long)]
        exclude_stubs: bool,

        /// Sort by field (created, tags)
        #[arg(long, default_value = "created")]
        sort: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Search documents by text
    Search {
        /// Search query
        query: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Case sensitive search
        #[arg(long)]
        case_sensitive: bool,

        /// Use regular expression
        #[arg(long)]
        regex: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
