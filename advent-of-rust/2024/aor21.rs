use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let mut file_path = std::env::temp_dir();
        Self::push_random_filename(&mut file_path)?;
        let file = File::create(&file_path)?;
        Ok(Self { file_path, file })
    }

    fn push_random_filename(p: &mut PathBuf) -> Result<(), std::io::Error> {
        let mut buf = [0_u8; 16];
        {
            use std::io::Read;
            File::open("/dev/urandom")?.read_exact(&mut buf)?;
        }
        let mut f = String::with_capacity(4 + buf.len() * 2);
        f.push_str("tmp.");
        for b in buf {
            use std::fmt::Write;
            write!(f, "{:02x}", b).unwrap();
        }
        p.push(&f);
        Ok(())
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        // Your code here...
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        use std::io::Write;
        file.write_all(data)
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        // Your code here...
        let mut file = File::open(&self.file_path)?;
        let metadata = file.metadata()?;
        let mut buf = String::with_capacity(metadata.len() as usize);
        use std::io::Read;
        file.read_to_string(&mut buf)?;
        Ok(buf)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ignore_errors = std::fs::remove_file(&self.file_path);
    }
}
