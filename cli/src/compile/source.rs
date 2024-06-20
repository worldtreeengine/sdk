use std::collections::VecDeque;
use std::fs::DirEntry;
use std::path::PathBuf;
use anyhow::{Context, Result};

pub fn gather_sources(context: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut entries: VecDeque<std::io::Result<DirEntry>> = VecDeque::new();
    entries.extend(std::fs::read_dir(context).with_context(|| format!("FATAL Failed to read context {:?}", context))?);
    eprintln!("Gathering sources...");
    while let Some(entry) = entries.pop_front() {
        match entry {
            Err(_e) => continue,
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    if let Ok(dir_entries) = std::fs::read_dir(&path) {
                        entries.extend(dir_entries);
                    } else {
                        eprintln!("    WARNING Failed to read subdirectory in context {:?}", &path);
                    }
                }

                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension.eq_ignore_ascii_case("yaml") || extension.eq_ignore_ascii_case("yml") {
                            eprintln!("    {}", (&path).strip_prefix(&context)?.to_str().unwrap_or(""));
                            paths.push(path);
                        }
                    }
                }
            }
        }
    }
    eprintln!();
    Ok(paths)
}
