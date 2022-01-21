#![macro_use]
use std::path::{Path, PathBuf};

// Encountered a fatal error
// Print error message and exit the current process
macro_rules! exit {
    ($($arg:tt)*) => {
        {   
            eprint!("{}", "[ERROR]: ");
            eprintln!($($arg)*);
            std::process::exit(1)
        }
    };
}

/// Validate and return a directory path.
pub fn get_valid_dirpath<P: AsRef<Path>>(path: P) -> anyhow::Result<PathBuf>
    where PathBuf: From<P>,
{
    match PathBuf::from(path) {
        p if !p.exists() => exit!("path \"{:?}\" was not found or inaccessible", &p),
        p if !p.is_dir() => exit!("path \"{:?}\" is not a valid directory", &p),
        p => Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    #[test]
    fn test_pathbuf() {
        let path = PathBuf::from("not_exist.txt");
        assert_eq!(path.exists(), false);
        let path2: PathBuf = [r"config/", "default.toml"].iter().collect();
        assert_eq!(path2.exists(), true);

        assert_eq!(Path::new("SMLNODE/fs/").is_dir(), true);
        assert_eq!(Path::new("a_file.txt").is_dir(), false);
    }
}