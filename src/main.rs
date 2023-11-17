use clap::{Arg, Command};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let matches = Command::new("Smelter")
        .version("0.1")
        .author("Alex Scrobot")
        .about("Combines project files into a single file")
        .subcommand(Command::new("melt")
            .about("Melt files into one")
            .arg(Arg::new("directory")
                .help("The directory to scan")
                .required(true)
                .index(1))
            .arg(Arg::new("exclude")
                .short('e')
                .long("exclude")
                .value_name("PATTERN")
                .help("File pattern to exclude")
                .num_args(1..))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file name")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("melt") {
        let directory = matches.get_one::<String>("directory").unwrap();
        let excludes = matches.get_many::<String>("exclude").map_or(vec![], |vals| vals.map(|s| s.to_string()).collect());
        let output = matches.get_one::<String>("output").map_or_else(|| "combined_project.txt".to_string(), |s| s.to_string());

        melt_files(directory, excludes, &output);
    }
}
fn melt_files(directory: &str, excludes: Vec<String>, output: &str) {
    // Проверка существования директории
    if !Path::new(directory).exists() {
        eprintln!("Error: Directory not found: {}", directory);
        return;
    }

    // Проверка прав доступа к директории
    let metadata = match fs::metadata(directory) {
        Ok(metadata) => metadata,
        Err(_) => {
            eprintln!("Error: Unable to access directory metadata: {}", directory);
            return;
        }
    };

    if !metadata.is_dir() {
        eprintln!("Error: '{}' is not a directory", directory);
        return;
    }

    let files: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !excludes.iter().any(|pattern| e.path().to_string_lossy().contains(pattern)))
        .collect();

    // Обработка пустой директории
    if files.is_empty() {
        eprintln!("Error: No files found in the directory: {}", directory);
        return;
    }

    let progress_bar = ProgressBar::new(files.len() as u64);
    let style = ProgressStyle::default_bar()
        .template("{wide_bar} {pos}/{len}")
        .unwrap_or_else(|e| {
            eprintln!("Error setting progress bar style: {}", e);
            ProgressStyle::default_bar()
        });
    progress_bar.set_style(style);

    let mut output_content = String::new();
    let mut current_dir = PathBuf::new();
    let mut dir_files_count = 0;

    for entry in files {
        let path = entry.path();

        if path.is_dir() {
            if !current_dir.as_os_str().is_empty() {
                println!("Processed directory: {:?} - {} files", current_dir, dir_files_count);
            }
            current_dir = path.to_path_buf();
            dir_files_count = 0;
            continue;
        }

        if path.is_file() {
            dir_files_count += 1;
            match fs::read_to_string(path) {
                Ok(content) => {
                    output_content.push_str(&format!("\n\n---\n{}:\n\n{}", path.display(), content));
                },
                Err(e) => {
                    eprintln!("Error reading file {}: {}", path.display(), e);
                    continue;
                }
            }
        }

        progress_bar.inc(1);
    }

    if !current_dir.as_os_str().is_empty() {
        println!("Processed directory: {:?} - {} files", current_dir, dir_files_count);
    }

    progress_bar.finish_with_message("Done!");

    // Проверка и обработка пути выходного файла
    let mut output_file = PathBuf::from(output);
    if output_file.is_relative() {
        output_file = fs::canonicalize(directory).unwrap().join(&output_file);
    }

    // Обработка ошибки записи
    if let Err(e) = fs::write(&output_file, output_content) {
        eprintln!("Error writing to file {}: {}", output_file.display(), e);
    }
}
