use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use rayon::prelude::*;
use strsim::jaro_winkler;
use walkdir::WalkDir;

#[derive(Debug)]
struct MatchResult {
    score: f64,
    path: PathBuf,
}

fn main() {
    print!("Enter file name to search for: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim().to_lowercase();

    if query.is_empty() {
        println!("No input provided.");
        return;
    }

    let root = if cfg!(target_os = "windows") {
        PathBuf::from("C:\\")
    } else {
        PathBuf::from("/")
    };

    println!("Searching from {:?} ... this may take a while.\n", root);

    let results: Vec<MatchResult> = WalkDir::new(root)
        .into_iter()
        .par_bridge() // parallelize
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if !entry.file_type().is_file() {
                return None;
            }

            let filename = entry.file_name().to_string_lossy().to_lowercase();

            let score = jaro_winkler(&query, &filename);

            if score > 0.75 {
                Some(MatchResult {
                    score,
                    path: entry.path().to_path_buf(),
                })
            } else {
                None
            }
        })
        .collect();

    if results.is_empty() {
        println!("No good matches found.");
        return;
    }

    // Sort best first
    let mut results = results;
    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    println!("\nBest matches:\n");

    for r in results.iter().take(10) {
        println!("{:.3}  {}", r.score, r.path.display());
    }
}
