pub struct Matcher {
    arena: Vec<u8>,
    slices: Vec<(usize, usize)>,
}

impl Matcher {
    pub fn new(suffixes: &[&str]) -> Self {
        let mut arena = Vec::with_capacity(suffixes.iter().map(|s| s.len()).sum());
        let mut slices = Vec::with_capacity(suffixes.len());

        for suffix in suffixes {
            slices.push((arena.len(), suffix.len()));
            arena.extend_from_slice(suffix.as_bytes());
        }

        Self { arena, slices }
    }

    pub fn matches(&self, input: &str) -> bool {
        for &(index, len) in &self.slices {
            if input.as_bytes().ends_with(&self.arena[index..index + len]) {
                return true;
            }
        }

        false
    }
}
