use clap::Parser;
use std::path::PathBuf;

/// Convert .epub files to .azw3 files. You must have calibre's `ebook-convert` in your $PATH
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// List of .epub files that need conversion. Files in the output directory
    /// with a .azw3 extension will not be converted.
    pub(crate) epub_files: Vec<PathBuf>,

    /// directory to dump the .azw3 files to
    #[arg(long, short)]
    #[arg(default_value = "./")]
    pub(crate) output: PathBuf,
}
