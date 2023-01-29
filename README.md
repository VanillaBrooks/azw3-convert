# azw3-convert

Quickly convert a list of files in `.epub` format to `.azw3`

## About

`azw3-convert` is a wrapper around Calibre's `ebook-convert`. You must have `ebook-convert` installed for 
`azw3-convert` to properly function.

## Installation

You can install with cargo. Either:

```bash
git clone https://github.com/VanillaBrooks/azw3-convert.git && \
cd azw3-convert && \
cargo install --path .
```

or


```bash
cargo install --git "https://github.com/VanillaBrooks/azw3-convert.git" -f
```


## Usage


```
azw3-convert --help
```

```
Convert .epub files to .azw3 files

Usage: azw3-convert [OPTIONS] [EPUB_FILES]...

Arguments:
  [EPUB_FILES]...  List of .epub files that need conversion. Files in the output directory with a .azw3 extension will not be converted

Options:
  -o, --output <OUTPUT>  directory to dump the .azw3 files to [default: ./]
  -h, --help             Print help
  -V, --version          Print version

```

### Example

If you `cd` into a directory with 

```
book_1.epub
book_2.epub
book_3.epub
book_1.azw3
```

and run

```
azw3-convert *.epub
```

the resulting files will be
```
book_1.epub
book_2.epub
book_3.epub
book_1.azw3
book_2.azw3
book_3.azw3
```

`book_1.epub` will not be converted, since `book_1.azw3` already exists. You can change the output directory
with `--output`.
