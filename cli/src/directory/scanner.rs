use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub size: u64,
    pub is_immediate_child: bool,
}

impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.size.cmp(&other.size)
    }
}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct DirectoryScanner {
    base_path: PathBuf,
    max_depth: Option<usize>,
    max_results: usize,
    show_all: bool,
}

impl DirectoryScanner {
    pub fn new(path: &Path) -> Self {
        Self {
            base_path: path.to_path_buf(),
            max_depth: Some(1),
            max_results: 20,
            show_all: false,
        }
    }

    pub fn recursive(mut self) -> Self {
        self.max_depth = None;
        self
    }

    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    pub fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = show_all;
        self
    }

    pub fn scan(&self) -> io::Result<Vec<DirectoryEntry>> {
        let mut heap: BinaryHeap<Reverse<DirectoryEntry>> = BinaryHeap::new();
        let mut scan_count = 0u64;

        // Single-pass: calculate size while traversing
        self.scan_single_pass(&self.base_path, 0, &mut heap, &mut scan_count)?;

        // Clear progress line
        eprint!("\r                                                    \r");

        // Extract and sort
        let mut results: Vec<DirectoryEntry> = heap.into_iter().map(|r| r.0).collect();
        results.sort_by(|a, b| b.size.cmp(&a.size));

        Ok(results)
    }

    fn scan_single_pass(
        &self,
        path: &Path,
        current_depth: usize,
        heap: &mut BinaryHeap<Reverse<DirectoryEntry>>,
        scan_count: &mut u64,
    ) -> io::Result<u64> {
        // Show progress every 500 directories
        *scan_count += 1;
        if *scan_count % 500 == 0 {
            eprint!("\rScanning... {} directories", scan_count);
            use std::io::Write;
            std::io::stderr().flush().ok();
        }

        let mut total_size = 0u64;

        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => return Ok(0),
        };

        for entry in entries.flatten() {
            let entry_path = entry.path();
            
            // Filter out unwanted directories (unless --all is specified)
            if !self.show_all && should_skip(&entry_path) {
                continue;
            }

            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total_size += metadata.len();
                } else if metadata.is_dir() {
                    // Recurse within depth limit
                    let should_recurse = self.max_depth.map_or(true, |max| current_depth < max);
                    
                    let subdir_size = if should_recurse {
                        self.scan_single_pass(&entry_path, current_depth + 1, heap, scan_count)?
                    } else {
                        // Even if we don't recurse, calculate size for this directory
                        calculate_size_no_recurse(&entry_path)
                    };

                    total_size += subdir_size;

                    // Add to heap if it's a candidate
                    let is_immediate = current_depth == 0;
                    let dir_entry = DirectoryEntry {
                        path: entry_path,
                        size: subdir_size,
                        is_immediate_child: is_immediate,
                    };

                    if heap.len() < self.max_results {
                        heap.push(Reverse(dir_entry));
                    } else if let Some(Reverse(smallest)) = heap.peek() {
                        if subdir_size > smallest.size {
                            heap.pop();
                            heap.push(Reverse(dir_entry));
                        }
                    }
                }
            }
        }

        Ok(total_size)
    }
}

/// Check if directory should be skipped (TreeSize-style filtering)
fn should_skip(path: &Path) -> bool {
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        // Skip hidden directories (starting with .)
        if name.starts_with('.') {
            return true;
        }

        // Skip common build/cache directories
        matches!(
            name,
            "node_modules"
                | "target"
                | "build"
                | "dist"
                | "__pycache__"
                | "venv"
                | ".venv"
                | "vendor"
                | "pkg"
                | "obj"
                | "bin"
                | ".pytest_cache"
                | ".mypy_cache"
                | ".tox"
                | "coverage"
                | ".coverage"
                | "htmlcov"
        )
    } else {
        false
    }
}

/// Calculate size without recursing into subdirectories
fn calculate_size_no_recurse(path: &Path) -> u64 {
    let mut total = 0u64;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    total += metadata.len();
                }
                // Don't recurse into subdirectories
            }
        }
    }

    total
}
