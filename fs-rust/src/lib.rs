use std::io::Result;
use std::path::{Path, PathBuf};

pub trait FileSystem {
    fn init(&mut self, _req: &Request) -> Result<(), c_int>{
        Ok(())
    }
    fn current_dir(&self) -> Result<PathBuf>;
}
