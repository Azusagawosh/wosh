
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    match env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            scan_dir(path, String::new());
        }
        Err(error) => {
            eprintln!("[ERROR] Cannot get current directory: {}", error);
        }
    }
}

fn scan_dir(path: PathBuf, prefix: String) {
    match fs::read_dir(&path) {
        Ok(entries) => {
            let ignored_dirs = [
                "target",
                ".git",
                "node_modules",
                ".venv",
                "venv",
                "__pycache__",
                "dist",
                "build",
                ".next",
                ".nuxt",
            ];

            let entries: Vec<_> = entries
                .into_iter()
                .filter(|entry| {
                    match entry {
                        Ok(entry) => {
                            match entry.file_type() {
                                Ok(file_type) => {
                                    let is_dir = file_type.is_dir();
                                    let name = entry.file_name();

                                    !(is_dir
                                        && ignored_dirs.contains(
                                            &name.to_string_lossy().as_ref()
                                        ))
                                }
                                Err(error) => {
                                    eprintln!(
                                        "[WARNING] Cannot determine file type for '{}': {}",
                                        entry.path().display(),
                                        error
                                    );
                                    true
                                }
                            }
                        }
                        Err(_) => true,
                    }
                })
                .collect();

            let total = entries.len();

            for (index, entry) in entries.into_iter().enumerate() {
                match entry {
                    Ok(entry) => {
                        let is_last = index == total - 1;

                        let branch = if is_last {
                            "└──"
                        } else {
                            "├──"
                        };

                        let current_prefix = format!(
                            "{}{}",
                            prefix,
                            if is_last { "    " } else { "│   " }
                        );

                        let name = entry.file_name();
                        let name = name.to_string_lossy();

                        match entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_file() {
                                    println!(
                                        "{}{} {} - FILE",
                                        prefix, branch, name
                                    );
                                } else if file_type.is_dir() {
                                    println!(
                                        "{}{} {} - DIR",
                                        prefix, branch, name
                                    );

                                    scan_dir(entry.path(), current_prefix);
                                } else {
                                    println!(
                                        "{}{} {} - OTHER",
                                        prefix, branch, name
                                    );
                                }
                            }
                            Err(error) => {
                                eprintln!(
                                    "[WARNING] Cannot determine file type for '{}': {}",
                                    entry.path().display(),
                                    error
                                );
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!(
                            "[WARNING] Cannot read directory entry in '{}': {}",
                            path.display(),
                            error
                        );
                    }
                }
            }
        }
        Err(error) => {
            eprintln!(
                "[WARNING] Cannot read directory '{}': {}",
                path.display(),
                error
            );
        }
    }
}

