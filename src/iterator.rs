pub trait PeekableIterator {
    type Item;

    fn peek(&self) -> Option<Self::Item>;

    fn offset(&self, offset: usize) -> Option<Self::Item>;
}

pub struct StringIterator<'a> {
    text: &'a String,
    cur: usize,
    cur_char: usize,
    cur_line: usize,
}

impl<'a> StringIterator<'a> {
    pub fn new(s: &'a String) -> Self {
        StringIterator { text: s, cur: 0, cur_char: 1, cur_line: 1 }
    }

    pub fn text(&self) -> &String {
        return self.text;
    }

    pub fn char(&self) -> usize {
        return self.cur_char;
    }

    pub fn line(&self) -> usize {
        return self.cur_line;
    }
}

impl PeekableIterator for StringIterator<'_> {
    type Item = char;

    fn peek(&self) -> Option<Self::Item> {
        if self.cur >= self.text.len() {
            return None
        }

        let b = self.text.as_bytes()[self.cur];

        return Some(b as char);
    }

    fn offset(&self, offset: usize) -> Option<Self::Item> {
        if self.cur + offset >= self.text.len() {
            return None
        }

        let b = self.text.as_bytes()[self.cur + offset];

        return Some(b as char);
    }
}

impl Iterator for StringIterator<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.text.len() {
            return None
        }

        let b = self.text.as_bytes()[self.cur];

        self.cur += 1;

        let as_char =  b as char;

        if as_char == '\n' {
            self.cur_line += 1;
            self.cur_char = 1;
        } else {
            self.cur_char += 1;
        }

        return Some(as_char);
    }
}
