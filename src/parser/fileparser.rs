use std::path::{Path, PathBuf};
use std::io::{Write, Read, Error as IOError};
use std::collections::{HashMap};
use std::ffi::OsStr;
use std::fs;
use std::fs::create_dir_all;
use grammar::html;
use validate::validate_and_get_paths;
use interpret::into_html;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum BuildError {
    IO(IOError),
    Parser(html::ParseError),
    InvalidPaths(Vec<(String, usize)>)
}
impl PartialEq for BuildError {
    fn eq(&self, other: &BuildError) -> bool {
        match (self, other) {
            (&BuildError::IO(_), &BuildError::IO(_)) => false,

            (&BuildError::Parser(ref err1), 
             &BuildError::Parser(ref err2)) => err1 ==  err2,

            (&BuildError::InvalidPaths(ref paths1), 
             &BuildError::InvalidPaths(ref paths2)) => paths1 == paths2,
            _ => false,
        }
    }
}

struct FileBag {
    processed_files: HashMap<PathBuf, bool>,
}
impl FileBag {
    fn next(&mut self) -> Option<PathBuf> {
        let result = self.processed_files.iter()
            .find(|&(_, processed)|{!processed})
            .map(|(filepath, _)|{ filepath.to_owned() });

        if let Some(filepath) = result {
            self.processed_files.insert(filepath.clone(), true);
            Some(filepath)
        } else {
            None
        }
    }

    fn put(&mut self, paths: Vec<PathBuf>) {
        for path in paths {
            if None == self.processed_files.get(&path) {
                self.processed_files.insert(path, false);
            }
        }
    }

    fn new() -> FileBag {
        FileBag{processed_files: HashMap::new()}
    }
}

/// Takes the path to the source directory, builds it, 
/// and places the output in the out_root directory.
///
/// For the build to be successfull the source path needs to contain an
/// index.foil file, because this is the first file being built.
/// All specified paths must be absolute.
pub fn build_dir(src_root: PathBuf, out_root: PathBuf) -> Result<(), BuildError> {
    let index_file = src_root.join(Path::new("index.foil"));
    let mut file_bag = FileBag::new();
    let result = parse_and_copy(&index_file, &src_root, &out_root);
    match result {
        Ok(files) => { file_bag.put(files); },
        Err(err) => { return Err(err); },
    }

    let foil_extension = OsStr::new("foil");
    while let Some(file) = file_bag.next() {
        if file.extension() == Some(foil_extension) {
            let result = parse_and_copy(&file, &src_root, &out_root);
            match result {
                Ok(paths) => { file_bag.put(paths) },
                Err(err) => { return Err(err); },
            }
        } else {
            let result = copy_to_output(&file, &src_root, &out_root);
            match result {
                Err(err) => { return Err(err) },
                Ok(_) => { },
            }
        }
    }
    Ok(())
}

/// All paths need to be full paths.
fn parse_and_copy(file_path: &Path, src_root: &Path, out_root: &Path)
    -> Result<Vec<PathBuf>, BuildError> {
        let file_folder = file_path.parent().unwrap();

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

        let result = validate_and_get_paths(&html_tree, file_folder);
        if let Err(invalid_paths) = result {
            let invalid_paths = invalid_paths.iter()
                .map(|&(s, p)|{(s.to_string(), *p)});
            return Err(BuildError::InvalidPaths(Vec::from_iter(invalid_paths)));
        }
        let referenced_paths = result.unwrap();

        let relative_path = file_path.strip_prefix(src_root).unwrap();
        let output_path = out_root.join(relative_path).with_extension("html");

        //make sure that the folder exists
        {
            let output_file_dir = output_path.parent().unwrap();
            create_dir_all(output_file_dir);
        }
        
        let result = fs::File::create(output_path);
        if let Err(err) = result {
            return Err(BuildError::IO(err));
        }
        let mut out_file = result.unwrap();
        let html = into_html(&html_tree);
        let result = out_file.write_all(html.as_bytes());
        if let Err(err) = result {
            return Err(BuildError::IO(err));
        }

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

        let output_path = out_root.join(relative_path);

        {
            // make sure folder exists
            let output_file_dir = output_path.parent().unwrap();
            create_dir_all(output_file_dir);
        }
        let result = fs::copy(file, output_path);

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(BuildError::IO(err)),
        }
    }

#[cfg(test)]
mod tests {
    use tempdir::TempDir;
    use std::io::{Write, Read};
    use std::fs::{File, create_dir_all};
    use super::*;

    #[test]
    fn simple_index_build_without_paths_works() {
        // Create a tmp directory with the following structure:
        // tmp 
        // ├── src
        // │   └── index.foil
        // └── out
        let tmpdir = TempDir::new("test").unwrap();
        let src_root = tmpdir.path().join("src");
        let out_root = tmpdir.path().join("out");
        create_dir_all(src_root.to_str().unwrap());
        create_dir_all(out_root.to_str().unwrap());

        let index_file = src_root.join("index.foil");
        
        {
            let mut f = File::create(index_file).unwrap();
            f.write_all(b"html { h1 \"test\" }");
            f.sync_all();
        }
        
        // Run the build and expect the following file structure
        // tmp 
        // ├── src
        // │   └── index.foil
        // └── out
        //     └── index.html

        let result = build_dir(src_root, out_root.clone());
        assert!(result.is_ok());

        let mut contents = String::new();
        {
            let mut f = File::open(out_root.join("index.html")).unwrap();
            f.read_to_string(&mut contents);
        }
        assert_eq!(contents, "<html><h1>test</h1></html>");
    }

    #[test]
    fn works_with_non_foil_reference() {
        // Create a tmp directory with the src root havin an index file 
        // and a referenced resource file.
        //
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   └── resource.txt
        // └── out
        
        let tmpdir = TempDir::new("test").unwrap();
        let src_root = tmpdir.path().join("src");
        let out_root = tmpdir.path().join("out");
        create_dir_all(src_root.to_str().unwrap());
        create_dir_all(out_root.to_str().unwrap());

        let index_file = src_root.join("index.foil");
        {
            let mut f = File::create(index_file).unwrap();
            f.write_all(b"html { a href=<./resource.txt> \"resource file\" }");
            f.sync_all();
        }

        let resource_file = src_root.join("resource.txt");
        {
            let mut f = File::create(resource_file).unwrap();
            f.write_all(b"Some resources");
            f.sync_all();
        }

        // Run the build and expect the following file structure
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   └── resource.txt
        // └── out
        //     ├── index.html
        //     └── resource.txt
        
        let result = build_dir(src_root, out_root.clone());
        assert_eq!(Ok(()), result);

        let mut index_contents = String::new();
        {
            let mut f = File::open(out_root.join("index.html")).unwrap();
            f.read_to_string(&mut index_contents);
        }

        let mut resource_contents = String::new();
        {
            let mut f = File::open(out_root.join("resource.txt")).unwrap();
            f.read_to_string(&mut resource_contents);
        }

        assert_eq!(index_contents, 
                   "<html><a href=\"./resource.txt\">resource file</a></html>");
        assert_eq!(resource_contents, "Some resources");
    }

    #[test]
    fn works_with_foil_reference() {
        // Create a tmp directory with the src root havin an index file 
        // and a referenced resource file.
        //
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   ├── subsite.foil
        // │   └── resource.txt
        // └── out
        
        let tmpdir = TempDir::new("test").unwrap();
        let src_root = tmpdir.path().join("src");
        let out_root = tmpdir.path().join("out");
        create_dir_all(src_root.to_str().unwrap());
        create_dir_all(out_root.to_str().unwrap());

        let index_file = src_root.join("index.foil");
        {
            let mut f = File::create(index_file).unwrap();
            f.write_all(b"html { a href=<./subsite.foil> \"subsite\" }");
            f.sync_all();
        }

        let subsite_file = src_root.join("subsite.foil");
        {
            let mut f = File::create(subsite_file).unwrap();
            f.write_all(b"html { a href=<./resource.txt> \"resource file\" }");
            f.sync_all();
        }

        let resource_file = src_root.join("resource.txt");
        {
            let mut f = File::create(resource_file).unwrap();
            f.write_all(b"Some resources");
            f.sync_all();
        }

        // Run the build and expect the following file structure
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   ├── subsite.foil
        // │   └── resource.txt
        // └── out
        //     ├── index.html
        //     ├── subsite.html
        //     └── resource.txt
        
        let result = build_dir(src_root, out_root.clone());
        assert_eq!(Ok(()), result);

        let mut index_contents = String::new();
        {
            let mut f = File::open(out_root.join("index.html")).unwrap();
            f.read_to_string(&mut index_contents);
        }

        let mut subsite_contents = String::new();
        {
            let mut f = File::open(out_root.join("subsite.html")).unwrap();
            f.read_to_string(&mut subsite_contents);
        }

        let mut resource_contents = String::new();
        {
            let mut f = File::open(out_root.join("resource.txt")).unwrap();
            f.read_to_string(&mut resource_contents);
        }

        assert_eq!(index_contents, 
                   "<html><a href=\"./subsite.html\">subsite</a></html>");
        assert_eq!(subsite_contents, 
                   "<html><a href=\"./resource.txt\">resource file</a></html>");
        assert_eq!(resource_contents, "Some resources");
    }

    #[test]
    fn works_with_subfolders() {
        // Create a tmp directory with the src root havin an index file 
        // and a referenced resource file.
        //
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   ├── sites
        // │   │   └── subsite.foil
        // │   └── resources
        // │       └── resource.txt
        // └── out
        
        let tmpdir = TempDir::new("test").unwrap();
        let src_root = tmpdir.path().join("src");
        let out_root = tmpdir.path().join("out");
        let sites_dir = src_root.join("sites");
        let resources_dir = src_root.join("resources");
        let out_sites_dir = out_root.join("sites");
        let out_resources_dir = out_root.join("resources");
        create_dir_all(sites_dir.to_str().unwrap());
        create_dir_all(resources_dir.to_str().unwrap());

        let index_file = src_root.join("index.foil");
        {
            let mut f = File::create(index_file).unwrap();
            f.write_all(b"html { a href=<./sites/subsite.foil> \"subsite\" }");
            f.sync_all();
        }

        let subsite_file = sites_dir.join("subsite.foil");
        {
            let mut f = File::create(subsite_file).unwrap();
            f.write_all(b"html { a href=<../resources/resource.txt> \"resource file\" }");
            f.sync_all();
        }

        let resource_file = resources_dir.join("resource.txt");
        {
            let mut f = File::create(resource_file).unwrap();
            f.write_all(b"Some resources");
            f.sync_all();
        }

        // Run the build and expect the following file structure
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   ├── sites
        // │   │   └── subsite.foil
        // │   └── resources
        // │       └── resource.txt
        // └── out
        //     ├── index.html
        //     ├── sites
        //     │   └── subsite.html
        //     └── resources
        //         └── resource.txt
        
        let result = build_dir(src_root, out_root.clone());
        assert_eq!(Ok(()), result);

        let mut index_contents = String::new();
        {
            let mut f = File::open(out_root.join("index.html")).unwrap();
            f.read_to_string(&mut index_contents);
        }

        let mut subsite_contents = String::new();
        {
            let mut f = File::open(out_sites_dir.join("subsite.html")).unwrap();
            f.read_to_string(&mut subsite_contents);
        }

        let mut resource_contents = String::new();
        {
            let mut f = File::open(out_resources_dir.join("resource.txt")).unwrap();
            f.read_to_string(&mut resource_contents);
        }

        assert_eq!(index_contents, 
                   "<html><a href=\"./sites/subsite.html\">subsite</a></html>");
        assert_eq!(subsite_contents, 
                   "<html><a href=\"../resources/resource.txt\">resource file</a></html>");
        assert_eq!(resource_contents, "Some resources");
    }

    #[test]
    fn works_with_circular_references() {
        // Create a tmp directory with the src root havin an index file 
        // and a referenced resource file.
        //
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   └── about.foil
        // └── out
        
        let tmpdir = TempDir::new("test").unwrap();
        let src_root = tmpdir.path().join("src");
        let out_root = tmpdir.path().join("out");
        create_dir_all(src_root.to_str().unwrap());

        let index_file = src_root.join("index.foil");
        {
            let mut f = File::create(index_file).unwrap();
            f.write_all(b"html { a href=<./about.foil> \"about\" }");
            f.sync_all();
        }

        let about_file = src_root.join("about.foil");
        {
            let mut f = File::create(about_file).unwrap();
            f.write_all(b"a href=<./index.foil> \"go back to index\"");
            f.sync_all();
        }

        // Run the build and expect the following file structure
        // tmp 
        // ├── src
        // │   ├── index.foil
        // │   └── about.foil
        // └── out
        //     ├── index.html
        //     └── about.foil
        
        let result = build_dir(src_root, out_root.clone());
        assert_eq!(Ok(()), result);

        let mut index_contents = String::new();
        {
            let mut f = File::open(out_root.join("index.html")).unwrap();
            f.read_to_string(&mut index_contents);
        }

        let mut about_contents = String::new();
        {
            let mut f = File::open(out_root.join("about.html")).unwrap();
            f.read_to_string(&mut about_contents);
        }

        assert_eq!(index_contents, 
                   "<html><a href=\"./about.html\">about</a></html>");
        assert_eq!(about_contents, "<a href=\"./index.html\">go back to index</a>");
    }

}
