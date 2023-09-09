use std::{fmt::Debug, ops::Deref, sync::Arc};

use super::{Token, TokenType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringCursor {
    pub src: Arc<str>,
    idx: usize,
    start: usize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct SrcStr {
    pub src: Arc<str>,
    pub start_idx: usize,
    pub end_idx: usize,
}

impl StringCursor {
    pub fn from(src: &str) -> Self {
        Self {
            src: src.into(),
            idx: 0,
            start: 0,
        }
    }

    pub fn current(&self) -> Option<char> {
        self.src.chars().nth(self.idx)
    }

    pub fn next(&mut self) -> Option<char> {
        self.idx += 1;
        self.current()
    }

    pub fn last(&mut self) -> Option<char> {
        self.idx -= 1;
        self.current()
    }

    pub fn rollback(&mut self) {
        self.idx = self.start;
    }

    pub fn fold(&mut self) -> SrcStr {
        let start_idx = self.start;
        self.start = self.idx;
        SrcStr {
            src: self.src.clone(),
            start_idx,
            end_idx: self.idx,
        }
    }

    fn is_internal(&mut self, str: &str) -> Option<bool> {
        if self.src.get(self.idx..)?.starts_with(str) {
            self.idx += str.len();
            return Some(true);
        }
        Some(false)
    }

    pub fn is(&mut self, str: &str) -> bool {
        self.is_internal(str).unwrap_or(false)
    }

    pub fn expect_value<T, F>(&mut self, f: F) -> Option<T>
    where
        T: Sized + Copy,
        F: Fn(char) -> Option<T> + Sized + Copy,
    {
        let val = f(self.current()?);
        if val.is_some() {
            self.next();
        }
        val
    }

    fn expect_func_internal<T>(&mut self, func: T) -> Option<()>
    where
        T: Fn(char) -> bool + Sized + Copy,
    {
        if func(self.current()?) {
            self.next();
            return Some(());
        }
        None
    }

    pub fn expect_func<T>(&mut self, func: T) -> bool
    where
        T: Fn(char) -> bool + Sized + Copy,
    {
        self.expect_func_internal(func).is_some()
    }

    pub fn expect_any(&mut self, chars: &str) -> bool {
        self.expect_func(|c| chars.contains(c))
    }

    pub fn consume_until(&mut self, str: &str, allow_escapes: bool) -> Option<()> {
        while !self.is_internal(str)? {
            let curr = self.next()?;
            if allow_escapes && curr == '\\' {
                self.next()?;
            }
        }
        Some(())
    }

    pub fn consume_func<T>(&mut self, func: T)
    where
        T: Fn(char) -> bool + Sized + Copy,
    {
        while self.expect_func(func) {}
    }

    pub fn consume_any(&mut self, chars: &str) {
        while self.expect_any(chars) {}
    }
}

impl From<StringCursor> for Vec<Token> {
    fn from(mut cursor: StringCursor) -> Self {
        let mut out = vec![];

        while let Some(token) = Token::from(&mut cursor) {
            out.push(token.clone());

            if let Token { src: _, value: TokenType::Eof } = token {
                break;
            }
        }

        out
    }
}

impl Debug for SrcStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.deref().fmt(f)
    }
}

impl Deref for SrcStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.src[self.start_idx..self.end_idx]
    }
}

pub macro cursor_match(
    $cursor:expr;
    $($c:literal => $v:expr),+ $(, _ => $e:expr)? $(,)?
) {
    $(
        if $cursor.is($c) {
            return Some($v);
        }
    )+
    $(
        return $cursor.expect_value($e);
    )?
}
