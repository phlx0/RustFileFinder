# 🔍 Fuzzy File Finder (Rust)

A fast, parallel CLI tool that searches your filesystem for files whose names **approximately** match a query using Jaro–Winkler similarity.

## Features

- Fuzzy filename matching (handles typos)
- Parallel search with `rayon`
- Works on Windows, Linux, and macOS
- Ranks and shows the top 10 matches

## Dependencies

```toml
[dependencies]
rayon = "1"
strsim = "0.11"
walkdir = "2"
```

## Run

```bash
cargo run --release
```

Enter part of a filename when prompted.

## How It Works

- Searches from `C:\` (Windows) or `/` (Unix)
- Compares every filename to your query
- Keeps matches with score > `0.75`
- Sorts by best similarity

## Notes

- First run can be slow (it scans the whole disk)
- Permission errors are ignored
- `--release` is recommended for speed

## Ideas to Extend

- Exclude system folders
- Filter by extension
- Start from a custom directory
- Add CLI arguments instead of prompt
