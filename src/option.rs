use std::path::Path;

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
    /// set if allow this option to show directories
    pub fn dir(&mut self, if_show: bool) -> Self {
        self.dir = if_show;
        self.clone()
    }

    /// set if allow this option to show files
    pub fn file(&mut self, if_show: bool) -> Self {
        self.file = if_show;
        self.clone()
    }

    /// set if allow this option to show hidden files
    pub fn hidden(&mut self, if_show: bool) -> Self {
        self.hidden = if_show;
        self.clone()
    }

    /// set if allow this option to show unhidden files
    pub fn unhidden(&mut self, if_show: bool) -> Self {
        self.unhidden = if_show;
        self.clone()
    }

    /// set the level of recursion while listing files in some path
    pub fn level(&mut self, level: usize) -> Self {
        self.level = level;
        self.clone()
    }

    /// set if allow this option to list recursively
    pub fn recursive(&mut self, if_choose: bool) -> Self {
        self.recursive = if_choose;
        self.clone()
    }

    /// add one ext to the list of allowed extensions
    ///
    /// only files with one of these extensions will be listed
    ///
    /// e.g. ext("rs") will allow files with .rs extension to be listed
    ///
    /// e.g. ext("rs").ext("toml") will allow files with .rs and .toml extensions to be listed
    pub fn ext(&mut self, ext: &str) -> Self {
        self.sufs.push(format!(".{}", ext));
        self.clone()
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
    pub fn exts(&mut self, exts: Vec<&str>) -> Self {
        self.sufs = exts.iter().map(|s| format!(".{}", s)).collect();
        self.clone()
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
    pub fn suf(&mut self, suf: &str) -> Self {
        self.sufs.push(suf.to_string());
        self.clone()
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
    pub fn sufs(&mut self, sufs: Vec<&str>) -> Self {
        self.sufs = sufs.iter().map(|s| s.to_string()).collect();
        self.clone()
    }
}

/// impl list functionality
impl ListOption {
    /// Lists the files and directories at the given path according to the options set in the ListOption
    ///
    /// if the path is a file, it will be listed if it matches the options set in the ListOption
    ///
    /// if the path is a directory, all files and directories in it will be listed if they match the options set in the ListOption
    pub fn list(&self, path: &str) -> Vec<String> {
        // check if is a file
        let path = Path::new(path);
        // check if exists
        if !path.exists() {
            return Vec::new();
        }
        let mut ret: Vec<String> = Vec::new();
        if path.is_file() {
            if self.file && self.check_if_show_file(path.to_str().unwrap()) {
                ret.push(path.to_str().unwrap().to_string());
            }
        } else if path.is_dir() {
            // list all files
            let files = path.read_dir().unwrap();
            for file in files {
                let file = file.unwrap();
                let file_path = file.path();
                if file_path.is_dir() {
                    let next_level = if self.recursive {
                        self.level
                    } else {
                        self.level - 1
                    };
                    let sub_opt = ListOption {
                        level: next_level,
                        ..self.clone()
                    };
                    let sub_dir = file_path.to_str().unwrap();
                    if self.dir {
                        ret.push(sub_dir.to_string());
                    }
                    if sub_opt.check_if_list_dir(sub_dir) {
                        let sub_list = sub_opt.inner_list(file_path.to_str().unwrap());
                        ret.extend(sub_list);
                    }
                } else if file_path.is_file() {
                    let file_name = file_path.to_str().unwrap();
                    if self.check_if_show_file(file_name) {
                        ret.push(file_name.to_string());
                    }
                }
            }
        }
        ret
    }
    // Checks if a file should be shown based on the options set in the ListOption
    fn check_if_show_file(&self, file_path: &str) -> bool {
        let base_name = Path::new(file_path).file_name().unwrap().to_str().unwrap();
        // dbg!(file_path);
        // dbg!(self.sufs.is_empty() || self.sufs.iter().any(|suf| base_name.ends_with(suf)));
        // dbg!(&self.sufs);
        self.file
            && (self.sufs.is_empty() || self.sufs.iter().any(|suf| base_name.ends_with(suf)))
            && (self.hidden && base_name.starts_with('.')
                || self.unhidden && !base_name.starts_with('.'))
    }
    // Checks if a directory should be listed based on the options set in the ListOption
    fn check_if_list_dir(&self, dir_path: &str) -> bool {
        let dir_path = Path::new(dir_path).canonicalize().unwrap();
        let base_name = dir_path
            .file_name()
            .unwrap_or_else(|| panic!("{:?}", dir_path))
            .to_str()
            .unwrap();
        (self.level > 0 || self.recursive)
            && (self.hidden && base_name.starts_with('.')
                || self.unhidden && !base_name.starts_with('.'))
    }

    // Lists the files and directories at the given path according to the options set in the ListOption, used for recursive listing
    fn inner_list(&self, path: &str) -> Vec<String> {
        // check if is a file
        let path = Path::new(path);
        // check if exists
        if !path.exists() {
            return Vec::new();
        }
        let mut ret: Vec<String> = Vec::new();
        if path.is_file() {
            if self.file && self.check_if_show_file(path.to_str().unwrap()) {
                ret.push(path.to_str().unwrap().to_string());
            }
        } else if path.is_dir() && self.check_if_list_dir(path.to_str().unwrap()) {
            // list all files
            let files = path.read_dir().unwrap();
            for file in files {
                let file = file.unwrap();
                let file_path = file.path();
                if file_path.is_dir() {
                    let next_level = if self.recursive {
                        self.level
                    } else {
                        self.level - 1
                    };
                    let sub_opt = ListOption {
                        level: next_level,
                        ..self.clone()
                    };
                    let sub_dir = file_path.to_str().unwrap();
                    if self.dir {
                        ret.push(sub_dir.to_string());
                    }
                    if sub_opt.check_if_list_dir(sub_dir) {
                        let sub_list = sub_opt.inner_list(file_path.to_str().unwrap());
                        ret.extend(sub_list);
                    }
                } else if file_path.is_file() {
                    let file_name = file_path.to_str().unwrap();
                    if self.check_if_show_file(file_name) {
                        ret.push(file_name.to_string());
                    }
                }
            }
        }
        ret
    }
}
