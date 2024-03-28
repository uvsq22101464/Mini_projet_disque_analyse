mod file_tree;
mod print_tree;
mod size;
mod duplicate;

use clap::{Parser, Subcommand};
use file_tree::FileTree;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the disk usage tree for the given path
    Usage {
        /// (default '.')
        path: Option<PathBuf>,

        /// ajout option -l ou --lexicographic-sort
        #[clap(short = 'l', long = "lexicographic-sort", long_help = "Sort by alphabetical order")]
        lexicographic_sort: bool,

        /// ajout option -f ou --filter
        #[clap(short = 'f', long = "filter", long_help = "returns directories and files ending with the given argument")]
        filter: Option<PathBuf>,
    },

    /// Show all duplicated files for the given path
    Duplicates {
        /// (default '.')
        path: Option<PathBuf>
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Usage { path , lexicographic_sort, filter} => {
            let path = path.as_deref().unwrap_or(Path::new("."));
            let tree = FileTree::new(path)?;
            match lexicographic_sort {
                false => {
                    match filter {
                        Some(filter) => tree.show_filter(filter),
                        None => tree.show()
                    }
                }
                true => {
                    match filter {
                        Some(filter) => tree.show_filter_lexicographic(filter),
                        None => tree.show_lexicographic()
                    }
                }
            }
        }
        Commands::Duplicates { path } => {
            let path = path.as_deref().unwrap_or(Path::new("."));
            FileTree::new(path)?.signature();
        }
    }
    Ok(())
}
