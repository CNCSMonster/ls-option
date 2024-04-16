use std::{ffi::OsStr, path::Path};

#[derive(Clone, Debug)]
pub struct ListOption {
    // if true, list directories
    dir: bool,
    // if true, list files
    file: bool,
    // if true, show hidden files
    hidden: bool,
    // if true,show unhidden files
    unhidden: bool,
    // if true, list recursively
    recursive: bool,
    // default 1, list only current directory
    level: usize,
    // if not empty, list only files with these extensions
    sufs: Vec<String>,
}
// Default implementation for ListOption
impl Default for ListOption {
    fn default() -> Self {
        Self {
            dir: true,
            file: true,
            hidden: false,
            unhidden: true,
            recursive: false,
            level: 1,
            sufs: Vec::new(),
        }
    }
}

/// specify the list option
impl ListOption {
    pub fn new() -> Self {
        Self::default()
    }
    /// set if allow this option to show directories
    pub fn dir(&mut self, if_show: bool) -> &mut Self {
        self.dir = if_show;
        self
    }

    /// set if allow this option to show files
    pub fn file(&mut self, if_show: bool) -> &mut Self {
        self.file = if_show;
        self
    }

    /// set if allow this option to show hidden files
    pub fn hidden(&mut self, if_show: bool) -> &mut Self {
        self.hidden = if_show;
        self
    }

    /// set if allow this option to show unhidden files
    pub fn unhidden(&mut self, if_show: bool) -> &mut Self {
        self.unhidden = if_show;
        self
    }

    /// set the level of recursion while listing files in some path
    pub fn level(&mut self, level: usize) -> &mut Self {
        self.level = level;
        self
    }

    /// set if allow this option to list recursively
    pub fn recursive(&mut self, if_choose: bool) -> &mut Self {
        self.recursive = if_choose;
        self
    }

    /// add one ext to the list of allowed extensions
    ///
    /// only files with one of these extensions will be listed
    ///
    /// e.g. ext("rs") will allow files with .rs extension to be listed
    ///
    /// e.g. ext("rs").ext("toml") will allow files with .rs and .toml extensions to be listed
    pub fn ext(&mut self, ext: &str) -> &mut Self {
        self.sufs.push(format!(".{}", ext));
        self
    }

    /// add multiple exts to the list of allowed extensions
    ///
    /// only files with one of these extensions will be listed
    ///
    /// but this function will shadow the previous sufs
    ///
    /// e.g. exts(vec!["rs", "toml"]) will allow files with .rs and .toml extensions to be listed
    ///
    /// e.g. exts(vec!["rs"]).exts(vec!["toml"]) will only allow files with .toml extensions to be listed
    ///
    pub fn exts(&mut self, exts: Vec<&str>) -> &mut Self {
        self.sufs = exts.iter().map(|s| format!(".{}", s)).collect();
        self
    }

    /// add one suf to the list of allowed suffixes,
    /// only files with one of these suffixes will be listed
    ///
    /// e.g. suf("rs") will allow files with rs suffix to be listed
    ///
    /// notice that exts function will shadow the previous sufs
    ///
    /// e.g. suf(".rs").ext("toml") will only allow files with .rs and .toml extensions to be listed
    ///
    /// e.g. suf(".rs").suf(".toml") will only allow files with .toml extensions to be listed
    ///
    /// e.g. suf("rs").exts(vec!["toml"]) will only allow files with .toml extensions to be listed
    pub fn suf(&mut self, suf: &str) -> &mut Self {
        self.sufs.push(suf.to_string());
        self
    }

    /// add multiple sufs to the list of allowed suffixes
    ///
    /// only files with one of these suffixes will be listed
    ///
    /// but this function will shadow the previous sufs
    ///
    /// e.g. sufs(vec!["rs", "toml"]) will allow files with rs and toml suffixes to be listed
    ///
    /// e.g. sufs(vec![".rs"]).sufs(vec![".toml"]) will only allow files with .toml extensions to be listed
    ///
    /// e.g. sufs(vec![".rs"]).ext("toml") will allow files with .rs or .toml extension to be listed
    ///
    /// e.g. sufs(vec!["rs"]).exts(vec!["toml"]) will only allow files with .toml extensions to be listed
    ///
    pub fn sufs(&mut self, sufs: Vec<&str>) -> &mut Self {
        self.sufs = sufs.iter().map(|s| s.to_string()).collect();
        self
    }
}

impl ListOption {
    pub fn only_dir(&mut self) -> &mut Self {
        self.file = false;
        self.dir = true;
        self
    }
    pub fn only_file(&mut self) -> &mut Self {
        self.file = true;
        self.dir = false;
        self
    }
    pub fn only_hidden(&mut self) -> &mut Self {
        self.hidden = true;
        self.unhidden = false;
        self
    }
    pub fn only_unhidden(&mut self) -> &mut Self {
        self.hidden = false;
        self.unhidden = true;
        self
    }
}

/// impl list functionality
impl ListOption {
    /// Lists the files and directories at the given path according to the options set in the ListOption
    ///
    /// if the path is a file, it will be listed if it matches the options set in the ListOption
    ///
    /// if the path is a directory, all files and directories in it will be listed if they match the options set in the ListOption
    pub fn list<S>(&self, path: &S) -> Vec<String>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let mut ret: Vec<String> = Vec::new();
        if self.level == 0 {
            return ret;
        }
        let path = Path::new(path);
        if self.would_show(path) {
            ret.push(path.to_str().unwrap().to_string());
        }
        if path.is_file() {
            return ret;
        }
        if path.is_dir() {
            // if is a directory, list all files and directories in it
            for entry in path.read_dir().unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let mut sub_option = self.clone();
                if !self.recursive {
                    sub_option.level = if self.level == 0 { 0 } else { self.level - 1 };
                }
                if self.would_show(&path) {
                    ret.push(path.to_str().unwrap().to_string());
                }
                ret.extend(sub_option.inner_list(&path));
            }
        }
        ret
    }
    fn inner_list(&self, path: &Path) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        if self.level == 0 {
            return ret;
        }
        if path.is_dir() {
            // if is a directory, list all files and directories in it
            for entry in path.read_dir().unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                let mut sub_option = self.clone();
                if !self.recursive {
                    sub_option.level = if self.level == 0 { 0 } else { self.level - 1 };
                }
                if self.would_show(&path) {
                    ret.push(path.to_str().unwrap().to_string());
                }
                ret.extend(sub_option.inner_list(&path));
            }
        }
        ret
    }

    /// check if the path would be shown according to the options set in the ListOption
    pub fn would_show<S>(&self, path: &S) -> bool
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let check_hidden = |path: &Path| {
            let base_name = path.file_name().unwrap().to_str().unwrap();
            if self.hidden && base_name.starts_with('.') {
                true
            } else {
                self.unhidden && !base_name.starts_with('.')
            }
        };
        let check_file_dir =
            |path: &Path| (path.is_file() && self.file) || (path.is_dir() && self.dir);
        let check_level = || self.recursive || self.level > 0;
        let check_ext = |path: &Path| {
            self.sufs.is_empty()
                || self
                    .sufs
                    .iter()
                    .any(|suf| path.to_str().unwrap().ends_with(suf))
        };
        let path = Path::new(path);
        if !path.exists() {
            return false;
        }
        let path = &path.canonicalize().unwrap();
        path.exists()
            && check_hidden(path)
            && check_file_dir(path)
            && check_level()
            && check_ext(path)
    }
}
