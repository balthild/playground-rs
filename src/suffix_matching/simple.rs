pub struct Matcher {
    suffixes: Vec<String>,
}

impl Matcher {
    pub fn new(suffixes: &[&str]) -> Self {
        let suffixes = suffixes.iter().map(|x| x.to_string()).collect();
        Self { suffixes }
    }

    pub fn matches(&self, input: &str) -> bool {
        for suffix in self.suffixes.iter() {
            if input.ends_with(suffix) {
                return true;
            }
        }

        false
    }
}
