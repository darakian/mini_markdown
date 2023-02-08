//
pub struct MiniIter <'a> {
    the_str: &'a str,
    char_index_iter: Box<dyn Iterator<Item = (usize, char)> + 'a>,
    peek_storage: Option<&'a str>,
    index: usize,
}

impl <'a> Iterator for MiniIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek_storage {
            Some(s) => {
                self.peek_storage = None;
                self.index += s.len();
                Some(s)
            },
            None => {
                match self.char_index_iter.next(){
                    Some(t) => {
                        self.index = t.0;
                        self.the_str.get(t.0..=(t.0+t.1.len_utf8()-1))},
                    None => None,
                }                
            },
        }
    }
}

impl <'a> MiniIter<'a> {
    pub fn new(source: &'a str) -> MiniIter<'a>{
        MiniIter {
            the_str: source, 
            char_index_iter: Box::new(source.char_indices()), 
            peek_storage: None,
            index: 0
        }
    }

    pub fn peek(&mut self) -> Option<&'a str> {
        if self.peek_storage.is_none() {
            self.peek_storage = self.next();
        }
        self.peek_storage
    }

    pub fn next_if_eq(&mut self, expected: &'a str) -> Option<&'a str> {
        if self.peek() == Some(expected) {
            return self.next()
        }
        None
    }

    pub fn consume_while_case_holds(&mut self, func: &dyn Fn(&str) -> bool) -> Option<&'a str> {
        let start_index = self.index;
        while self.peek().is_some() && func(self.peek().unwrap()) {
            self.next();
        }
        self.the_str.get(start_index..self.index)
    }

    pub fn consume_until_tail_is(&mut self, tail: &str) -> Option<&'a str> {
        let start_index = self.index;
        while self.peek().is_some() 
        && !self.the_str.get(start_index..self.index).unwrap_or(tail).ends_with(tail) { //unwrap_or(tail) to ensure exit in unforseen situation
            self.next();
        }
        self.the_str.get(start_index..self.index)
    }

    pub fn consume_until_end(&mut self) -> Option<&'a str> {
        let start_index = self.index;
        while self.peek().is_some() {
            self.next();
        }
        self.the_str.get(start_index..self.index)
    }

    pub fn get_index(&self) -> usize{
        self.index
    }

    pub fn get_substring_from(&self, start: usize) -> Option<&'a str> {
        self.the_str.get(start..self.index)
    }

    pub fn get_substring_ahead(&self, end: usize) -> Option<&'a str> {
        self.the_str.get(self.index..(self.index+end))
    }

    pub fn find_next(&self, pattern: &str) -> Option<usize>{
        self.the_str[self.index..].find(pattern)
    }

    pub fn peek_line_ahead(&self) -> Option<&'a str> {
        match self.find_next("\n") {
            Some(newline_index) => return self.the_str.get(self.index..=(self.index+newline_index)),
            None => return None,
        }
    }
}