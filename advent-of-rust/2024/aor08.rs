use std::fs::File;
use std::io::Write;

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    const NEWLINE: [u8; 1] = [b'\n'];
    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        let logs = self.search(keyword);
        let mut f = File::create(path)?;
        for line in logs {
            f.write(line.as_bytes())?;
            // Undocumented: `self.logs` do not end in newlines, so we need our own.
            f.write(&Self::NEWLINE)?;
        }
        Ok(())
    }
}
