use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing::{error, info};

mod core;
mod knowledge;
mod utils;

use crate::core::ResearchAgent;
use crate::knowledge::{HubReader, SearchQuery};

#[derive(Parser)]
#[command(name = "research-agent")]
#[command(about = "Autonomous research agent for academic corpus analysis", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Search the Hub corpus
    Search {
        /// Search query
        query: String,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        max_results: usize,

        /// Filter by discipline
        #[arg(short, long)]
        discipline: Option<String>,
    },

    /// Start a complete research session
    Query {
        /// Research question
        question: String,

        /// Output format (markdown, google-docs, html)
        #[arg(short, long, default_value = "markdown")]
        format: String,

        /// Output path/name
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Analyze a specific document
    Analyze {
        /// File name in Hub
        file: String,

        /// Extract arguments
        #[arg(long)]
        extract_arguments: bool,

        /// Validate citations
        #[arg(long)]
        validate_citations: bool,
    },

    /// List all documents in Hub
    Corpus {
        #[command(subcommand)]
        action: CorpusActions,
    },

    /// Initialize the research agent (build indexes, etc.)
    Init {
        /// Force rebuild of all indexes
        #[arg(long)]
        rebuild: bool,
    },
}

#[derive(Subcommand)]
enum CorpusActions {
    /// Show corpus statistics
    Stats,

    /// List all files
    List {
        /// Filter by pattern
        #[arg(short, long)]
        pattern: Option<String>,
    },

    /// Build semantic index
    Index {
        /// Force rebuild
        #[arg(long)]
        rebuild: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize tracing
    init_logging(cli.verbose);

    info!("Research Agent v{}", env!("CARGO_PKG_VERSION"));

    match cli.command {
        Commands::Search {
            query,
            max_results,
            discipline,
        } => {
            handle_search(&query, max_results, discipline).await?;
        }

        Commands::Query {
            question,
            format,
            output,
        } => {
            handle_query(&question, &format, output.as_deref()).await?;
        }

        Commands::Analyze {
            file,
            extract_arguments,
            validate_citations,
        } => {
            handle_analyze(&file, extract_arguments, validate_citations).await?;
        }

        Commands::Corpus { action } => match action {
            CorpusActions::Stats => {
                handle_corpus_stats().await?;
            }
            CorpusActions::List { pattern } => {
                handle_corpus_list(pattern.as_deref()).await?;
            }
            CorpusActions::Index { rebuild } => {
                handle_corpus_index(rebuild).await?;
            }
        },

        Commands::Init { rebuild } => {
            handle_init(rebuild).await?;
        }
    }

    Ok(())
}

async fn handle_search(
    query: &str,
    max_results: usize,
    discipline: Option<String>,
) -> anyhow::Result<()> {
    println!("{}", "🔍 Searching corpus...".cyan().bold());

    let hub_reader = HubReader::new();
    
    let search_query = SearchQuery {
        text: query.to_string(),
        max_results,
        discipline,
        ..Default::default()
    };

    let results = hub_reader.search(&search_query).await?;

    if results.is_empty() {
        println!("{}", "No results found.".yellow());
        return Ok(());
    }

    println!("\n{} {}\n", "Found".green().bold(), format!("{} results:", results.len()).green());

    for (i, result) in results.iter().enumerate() {
        println!("{}. {}", (i + 1).to_string().cyan(), result.file_name.bold());
        println!("   Score: {:.3}", result.score);
        if let Some(summary) = &result.summary {
            println!("   {}", summary.dimmed());
        }
        println!();
    }

    Ok(())
}

async fn handle_query(question: &str, format: &str, output: Option<&str>) -> anyhow::Result<()> {
    println!("{}", "🤖 Starting research session...".cyan().bold());
    println!("Question: {}\n", question.italic());

    let agent = ResearchAgent::new().await?;

    // This will be the full pipeline
    println!("{}", "⚠️  Full pipeline not yet implemented.".yellow());
    println!("For now, use: research-agent search \"<query>\"");

    Ok(())
}

async fn handle_analyze(
    file: &str,
    extract_arguments: bool,
    validate_citations: bool,
) -> anyhow::Result<()> {
    println!("{}", format!("📄 Analyzing: {}", file).cyan().bold());

    let hub_reader = HubReader::new();
    let content = hub_reader.read_file(file, None, None).await?;

    println!("\n{}", "File content preview:".green());
    println!("{}", "-".repeat(80));
    
    let preview_lines: Vec<&str> = content.lines().take(30).collect();
    println!("{}", preview_lines.join("\n"));
    
    if content.lines().count() > 30 {
        println!("\n{}", format!("... ({} more lines)", content.lines().count() - 30).dimmed());
    }

    if extract_arguments {
        println!("\n{}", "⚠️  Argument extraction not yet implemented.".yellow());
    }

    if validate_citations {
        println!("{}", "⚠️  Citation validation not yet implemented.".yellow());
    }

    Ok(())
}

async fn handle_corpus_stats() -> anyhow::Result<()> {
    println!("{}", "📊 Corpus Statistics\n".cyan().bold());

    let hub_reader = HubReader::new();
    let files = hub_reader.list_files().await?;

    println!("Total documents: {}", files.len().to_string().green().bold());

    // Count by file type
    let mut pdf_count = 0;
    let mut other_count = 0;

    for file in &files {
        if file.file_name.ends_with(".pdf") {
            pdf_count += 1;
        } else {
            other_count += 1;
        }
    }

    println!("  PDF documents: {}", pdf_count);
    println!("  Other formats: {}", other_count);

    // Show sample files
    println!("\n{}", "Recent files:".cyan());
    for file in files.iter().take(5) {
        println!("  • {}", file.file_name);
        if let Some(summary) = &file.summary {
            println!("    {}", summary.dimmed());
        }
    }

    Ok(())
}

async fn handle_corpus_list(pattern: Option<&str>) -> anyhow::Result<()> {
    let hub_reader = HubReader::new();
    let files = hub_reader.list_files().await?;

    let filtered_files: Vec<_> = match pattern {
        Some(p) => files
            .into_iter()
            .filter(|f| f.file_name.contains(p))
            .collect(),
        None => files,
    };

    println!("{}", format!("📚 {} documents in corpus\n", filtered_files.len()).cyan().bold());

    for file in filtered_files {
        println!("• {}", file.file_name.bold());
        if let Some(summary) = &file.summary {
            println!("  {}", summary.dimmed());
        }
        println!();
    }

    Ok(())
}

async fn handle_corpus_index(rebuild: bool) -> anyhow::Result<()> {
    println!("{}", "🔨 Building corpus index...".cyan().bold());

    if rebuild {
        println!("{}", "Force rebuild enabled".yellow());
    }

    println!("{}", "⚠️  Indexing not yet fully implemented.".yellow());
    println!("Basic file listing is available via: research-agent corpus list");

    Ok(())
}

async fn handle_init(rebuild: bool) -> anyhow::Result<()> {
    println!("{}", "🚀 Initializing Research Agent...\n".cyan().bold());

    // Check Hub connectivity
    println!("{}", "Checking Hub connectivity...".dimmed());
    let hub_reader = HubReader::new();
    match hub_reader.list_files().await {
        Ok(files) => {
            println!("  {} Hub connected ({} files found)", "✓".green(), files.len());
        }
        Err(e) => {
            error!("Failed to connect to Hub: {}", e);
            return Err(e);
        }
    }

    // Initialize cache directory
    println!("{}", "Initializing cache...".dimmed());
    let cache_dir = std::env::current_dir()?.join("research-agent").join(".cache");
    std::fs::create_dir_all(&cache_dir)?;
    println!("  {} Cache directory: {}", "✓".green(), cache_dir.display());

    if rebuild {
        println!("\n{}", "Rebuilding indexes...".yellow());
        // TODO: Implement index rebuild
    }

    println!("\n{}", "✅ Initialization complete!".green().bold());
    println!("\nTry: research-agent search \"game theory\"");

    Ok(())
}

fn init_logging(verbose: bool) {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = if verbose {
        EnvFilter::new("research_agent=debug,warn")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("research_agent=info,warn"))
    };

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}
