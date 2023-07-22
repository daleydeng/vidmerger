use std::path::PathBuf;

pub fn filter_files(all_files: Vec<PathBuf>, file_format: &str) -> Vec<PathBuf> {
    let mut filtered_files = Vec::new();
    let suffix = regex::escape(file_format);

    for possible_file_to_merge in all_files {
        if possible_file_to_merge.extension().unwrap().to_string_lossy() == suffix {
            filtered_files.push(possible_file_to_merge);
        }
    }
    filtered_files
}
