pub mod config;
pub mod cron;
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

        /// Number of concepts to generate (default: from config)
        #[arg(short, long)]
        count: Option<usize>,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,

        /// Quiet mode - minimal output
        #[arg(short, long)]
        quiet: bool,

        /// Skip confirmation prompts
        #[arg(long)]
        no_confirm: bool,

        /// Skip git commit and push after completion
        #[arg(long)]
        no_git: bool,
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

    /// Tag management commands
    Tags {
        #[command(subcommand)]
        action: TagAction,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,
    },

    /// Show document connection graph
    Graph {
        /// Starting document title
        title: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Maximum depth to traverse
        #[arg(long, default_value = "2")]
        depth: usize,

        /// Show incoming links only
        #[arg(long)]
        incoming: bool,

        /// Show outgoing links only
        #[arg(long)]
        outgoing: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Extended statistics
    Stats {
        /// Statistics to show
        #[arg(default_value = "basic")]
        stat_type: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Detect and fix wiki technical debt
    Clean {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Dry run - preview without making changes
        #[arg(long)]
        dry_run: bool,

        /// Fix issues automatically (remove empty stubs)
        #[arg(long)]
        fix: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Import external markdown files into the wiki
    Import {
        /// Path to file or directory to import
        source: String,

        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Export wiki as a static site for hosting
    Export {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Output directory for exported files
        #[arg(short, long, default_value = "dist")]
        output: String,

        /// Hosting target: firebase, cloudflare, or both
        #[arg(short, long, value_delimiter = ',', default_value = "firebase")]
        hosting: Vec<String>,
    },

    /// Start HTTP server to browse wiki
    Serve {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Port to listen on
        #[arg(short, long, default_value = "15432")]
        port: u16,

        /// Host address to bind
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Open browser automatically
        #[arg(short, long)]
        open: bool,
    },

    /// Detect duplicate or similar documents
    Dedup {
        /// Path to wiki directory
        #[arg(default_value = ".")]
        path: String,

        /// Similarity threshold for fuzzy matching (0.0-1.0)
        #[arg(long, default_value = "0.8")]
        threshold: f64,

        /// Output as JSON
        #[arg(long)]
        json: bool,
    },

    /// Manage cron job for scheduled runs
    Cron {
        /// Set cron time (HH:MM format, e.g., 14:30)
        #[arg(long, value_name = "TIME")]
        set: Option<String>,

        /// Show current wistra cron job
        #[arg(long)]
        show: bool,

        /// Remove wistra cron job
        #[arg(long)]
        remove: bool,

        /// Install cron job automatically
        #[arg(long)]
        install: bool,

        /// Skip git commit/push in cron job
        #[arg(long)]
        no_git: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum TagAction {
    /// List all tags with document counts
    List,

    /// Rename a tag across all documents
    Rename {
        /// Old tag name
        old_tag: String,

        /// New tag name
        new_tag: String,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,
    },

    /// Merge source tag into target tag
    Merge {
        /// Source tag (will be merged into target)
        source: String,

        /// Target tag (will be kept)
        target: String,

        /// Dry run - preview without writing
        #[arg(long)]
        dry_run: bool,
    },

    /// Find unused tags (no documents)
    Orphans,
}
