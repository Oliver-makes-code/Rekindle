pub struct StringCursor {
    pub string: String,
    pub idx: usize,
}

impl StringCursor {
    pub fn from(str: &str) -> Self {
        Self {
            string: str.into(),
            idx: 0
        }
    }

    pub fn current(&self) -> Option<char> {
        self.string.chars().nth(self.idx)
    }
    pub fn advance(&mut self) {
        self.idx += 1;
    }
}
