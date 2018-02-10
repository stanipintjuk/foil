use std::path::{Path, PathBuf};
use std::io::{Write, Read, Error as IOError};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ffi::OsStr;
use std::fs;
use grammar::html;
use validate::validate_and_get_paths;
use interpret::into_html;
use std::iter::FromIterator;

struct FileState {
    processed: bool,
    path: PathBuf,
}
impl FileState {
    fn copy(&self) -> Self {
        FileState {
            processed: self.processed,
            path: self.path.clone(),
        }
    }
}
impl PartialEq for FileState {
    fn eq(&self, other: &FileState) -> bool {
        self.path == other.path
    }
}
impl Eq for FileState {}
impl Hash for FileState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

enum FileBuildResult {
    Copy(PathBuf),
    ParseAndCopy(PathBuf),
    Error(PathBuf, BuildError),
}

enum BuildError {
    Failed,
    IO(IOError),
    Parser(html::ParseError),
    InvalidPaths(Vec<(String, usize)>)
}

struct FileIterator {
    files: HashSet<FileState>,
}
impl Iterator for FileIterator {
    type Item = FileState;

    fn next(&mut self) -> Option<Self::Item> {
        let file = self.get_unprocessed_file();

        if let Some(file_state) = file {
            Some(file_state.copy())
        } else {
            None
        }
    }
}
impl  FileIterator {
    fn new() -> FileIterator {
        FileIterator{files: HashSet::new()}
    }

    fn get_unprocessed_file(&self) -> Option<&FileState> {
        self.files.iter().find(|file_state|{!file_state.processed})
    }

    fn put(&mut self,files: Vec<FileState>) {
        for file in files {
            self.files.insert(file);
        }
    }
}

fn build_dir(src_root: PathBuf, out_root: PathBuf) -> Result<(), BuildError> {

    let index_file = src_root.join(Path::new("index.foil"));
    let mut file_iterator = FileIterator::new();
    let result = parse_and_copy(&index_file, &src_root, &out_root);
    if let Ok(files) = result {
        file_iterator.put(files);
    }

    let foil_extension = OsStr::new("foil");
    while let Some(file) = file_iterator.next() {
        if file.path.extension() == Some(foil_extension) {
            let result = parse_and_copy(&file.path, &src_root, &out_root);
            match result {
                Ok(files) => { file_iterator.put(files); }
                Err(err) => { return Err(err); }
            }
        } else {
            let result = copy_to_output(&file.path, &src_root, &out_root);
            match result {
                Ok(file_state) => {file_iterator.put(vec![file_state])}
                Err(err) => { return Err(err); }
            }
        }
    }
    Ok(())
}

/// All paths need to be full paths.
fn parse_and_copy(file_path: &Path, src_root: &Path, out_root: &Path)
    -> Result<Vec<PathBuf>, BuildError> {
        let result = fs::File::open(file_path);
        if let Err(err) = result {
            return Err(BuildError::IO(err));
        }

        let mut file = result.unwrap();
        let mut contents = String::new();

        let result = file.read_to_string(&mut contents);
        if let Err(err) = result {
            return Err(BuildError::IO(err));
        }

        let result = html::node(&contents);
        if let Err(err) = result {
            return Err(BuildError::Parser(err));
        }
        let html_tree = result.unwrap();

        let result = validate_and_get_paths(&html_tree);
        if let Err(invalid_paths) = result {
            let invalid_paths = invalid_paths.iter()
                .map(|&(s, p)|{(s.to_string(), *p)});
            return Err(BuildError::InvalidPaths(Vec::from_iter(invalid_paths)));
        }
        let referenced_paths = result.unwrap();

        let relative_path = file_path.strip_prefix(src_root).unwrap();
        let output_path = out_root.join(relative_path);
        
        let result = fs::File::create(output_path);
        if let Err(err) = result {
            return Err(BuildError::IO(err));
        }
        let mut out_file = result.unwrap();
        let html = into_html(&html_tree);
        out_file.write_all(html.as_bytes());

        let cur_path_folder = file_path.parent().unwrap();
        let referenced_paths = referenced_paths.iter()
            .map(|&(p, _)|{
                let file_path = Path::new(p);
                if file_path.is_relative() {
                    cur_path_folder.join(file_path)
                } else {
                    file_path.to_owned()
                }
            });

        Ok(Vec::from_iter(referenced_paths))
    }

/// All paths need to be absolute.
fn copy_to_output(file: &Path, src_root: &Path, out_root: &Path) 
    -> Result<(), BuildError> {
        let relative_path = file.strip_prefix(src_root).unwrap();
        let output_path = out_root.join(relative_path);
        let result = fs::copy(file, output_path);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(BuildError::IO(err)),
        }
    }
