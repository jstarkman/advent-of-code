pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> LogQuery<'a> {
        Self { logs }
    }

    pub fn search<'b>(&'a self, keyword: &'b str) -> Vec<&'a str> {
        self.logs
            .iter()
            .filter_map(|s| s.contains(keyword).then_some(s.as_str()))
            .collect()
    }
}
