pub mod bin;
pub mod command;

pub mod util {
    use std::os::unix::fs::MetadataExt;
    use std::{fs, path::PathBuf};

    pub fn is_executable(file_path: PathBuf) -> bool {
        match fs::metadata(file_path) {
            Ok(file_info) => {
                let mode = file_info.mode();
                let is_executable = mode & 0o001;

                is_executable != 0
            }
            Err(..) => false,
        }
    }
}
