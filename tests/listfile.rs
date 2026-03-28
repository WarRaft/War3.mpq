#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    fn collect_files(dir: &Path, relative: &Path, result: &mut Vec<String>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .expect("Failed to read directory")
            .filter_map(|e| e.ok())
            .collect();

        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Skip hidden files/directories
            if name_str.starts_with('.') {
                continue;
            }

            let child_relative = relative.join(&name);

            if path.is_dir() {
                collect_files(&path, &child_relative, result);
            } else {
                let line = child_relative
                    .to_string_lossy()
                    .replace('/', "\\");
                result.push(line);
            }
        }
    }

    #[test]
    fn generate_listfile() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let extract_dir = manifest_dir.join("extract");

        assert!(extract_dir.exists(), "extract directory not found");

        let mut files = Vec::new();
        collect_files(&extract_dir, Path::new(""), &mut files);

        let content = files.join("\n");
        let output_path = manifest_dir.join("listfile.txt");
        fs::write(&output_path, &content).expect("Failed to write listfile.txt");

        println!("Written {} paths to {:?}", files.len(), output_path);
        assert!(!files.is_empty(), "No files found");
    }
}

