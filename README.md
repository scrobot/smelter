# Smelter

Smelter is a tool that combines project files into a single file.

## Usage

```
Smelter 0.1
Alex Scrobot
Combines project files into a single file

USAGE:
    Smelter [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    melt    Melt files into one
```

### melt

Melt files into one.

```
USAGE:
    Smelter melt [OPTIONS] <directory>

ARGS:
    <directory>    The directory to scan

OPTIONS:
    -e, --exclude <PATTERN>    File pattern to exclude
    -o, --output <FILE>        Output file name
```

## Example

```
Smelter melt my_directory -e "*.txt" -o combined_project.txt
```

## Installation

To use Smelter, add it to your `Cargo.toml`:

```
[dependencies]
clap = "3"
indicatif = "0.15"
walkdir = "2"
```

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.