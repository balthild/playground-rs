use glob_match::glob_match;

pub struct Matcher {
    pattern: String,
}

impl Matcher {
    pub fn new(suffixes: &[&str]) -> Self {
        let pattern = format!("*{{{}}}", suffixes.join(","));
        Self { pattern }
    }

    pub fn matches(&self, input: &str) -> bool {
        glob_match(&self.pattern, input)
    }
}
