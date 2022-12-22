use crate::helpers::io_helper::read_dir;
use crate::helpers::vec_helper::filter_files;
use std::fmt::Write as FmtWrite;
use std::fs::{canonicalize, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn gen_ffmpeg_input_content(target_dir: &Path, file_format: &str) -> String {
    let all_files_on_target_dir: Vec<PathBuf> = read_dir(target_dir).unwrap();
    let files_to_merge = filter_files(all_files_on_target_dir, file_format);
    let mut ffmpeg_input_content = String::new();

    for file_to_merge in files_to_merge {
        writeln!(
            ffmpeg_input_content,
            "file '{}'",
            canonicalize(file_to_merge).unwrap().display()
        )
        .unwrap();
    }
    ffmpeg_input_content
}

pub fn gen_ffmpeg_input_file(string: String, mut dir: PathBuf) -> PathBuf {
    dir.push("ffmpeg_input_file.txt");
    File::create(&dir)
        .unwrap()
        .write_all(string.as_bytes())
        .unwrap();
    dir
}
