mod cli;

use anyhow::Result;
use clap::Parser;
use xshell::{cmd, Shell};

use std::path::Path;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    for file in args.epub_files.iter() {
        convert_epub(file, &args.output)?;
    }

    Ok(())
}

/// convert a single .epub file to a .azw3 file
fn convert_epub(epub_file: &Path, output_directory: &Path) -> Result<()> {
    let sh = Shell::new()?;

    // verify that the file has a .epub extension
    if let Some(ext) = epub_file.extension() {
        let ext = ext.to_string_lossy();

        if ext.to_lowercase() != "epub" {
            anyhow::bail!(
                "path {} does not have a .epub extension",
                epub_file.display()
            );
        }
    } else {
        anyhow::bail!(
            "path {} does not have a .epub extension",
            epub_file.display()
        );
    }

    // parse the filename from the string
    let filename = epub_file
        .file_stem()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "epub file specified {} does not have a file name",
                epub_file.display()
            )
        })?
        // now convert the OsStr to a UTF-8 string
        .to_str()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "epub file {} does not have a UTF8 file name",
                epub_file.display()
            )
        })?;

    // the name of the file that will be created in .azw3 format
    let output_azw3 = output_directory.join(format!("{filename}.azw3"));

    // skip the conversion for files that already exist
    if output_azw3.exists() {
        return Ok(());
    }

    cmd!(sh, "ebook-convert {epub_file} {output_azw3}").run()?;

    Ok(())
}
