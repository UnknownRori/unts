use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    rc::Rc,
};

use directories::ProjectDirs;

use crate::utility::project_dir;

/// This struct provide abstract way to write file to the [`ProjectDirs`]
pub struct FileService {
    project_dir: Rc<ProjectDirs>,
}

impl Default for FileService {
    fn default() -> Self {
        Self {
            project_dir: Rc::new(project_dir().unwrap()),
        }
    }
}

impl FileService {
    pub fn new(project_dir: &Rc<ProjectDirs>) -> std::io::Result<FileService> {
        let temp = Self {
            project_dir: Rc::clone(project_dir),
        };

        std::fs::create_dir_all(temp.project_dir.project_path())?;
        std::fs::create_dir_all(temp.project_dir.data_dir())?;
        std::fs::create_dir_all(temp.project_dir.cache_dir())?;

        Ok(temp)
    }

    pub fn write_data(&self, filename: &str, content: &str) -> std::io::Result<()> {
        let path = self.generate_data_path(filename);

        Ok(std::fs::write(path, content)?)
    }

    /// Read a file from data
    pub fn read_data(&self, filename: &str) -> std::io::Result<String> {
        let path = self.generate_data_path(filename);

        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    pub fn delete_data(&self, filename: &str) -> std::io::Result<()> {
        let path = self.generate_data_path(filename);
        std::fs::remove_file(path)?;

        Ok(())
    }

    /// This method will generate a path that used [`ProjectDirs`]
    ///
    /// @param  &str
    /// @return String
    pub fn generate_data_path(&self, filepath: &str) -> Box<Path> {
        self.project_dir.data_dir().join(filepath).into_boxed_path()
    }
}
