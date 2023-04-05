//
pub struct MiniIter <'a> {
    the_str: &'a str,
    index: usize,
}

impl <'a> Iterator for MiniIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 1..=3 {
            if self.the_str.is_char_boundary(self.index+i) {
                let ret = self.the_str.get(self.index..self.index+i);
                self.index += i;
                return ret
            }
        }
        None
    }
}

impl <'a> MiniIter<'a> {
    pub fn new(source: &'a str) -> MiniIter<'a>{
        MiniIter {
            the_str: source, 
            index: 0
        }
    }

    fn update_index_to(&mut self, i: usize) {
        self.index = i;
    }

    pub fn peek(&self) -> Option<&'a str> {
        for i in 1..=3 {
            if self.the_str.is_char_boundary(self.index+i) {
                return self.the_str.get(self.index..self.index+i)
            }
        }
        None
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

    pub fn peek_until_end(&self) -> Option<&'a str> {
        self.the_str.get(self.index..=(self.the_str.len()-1))
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
            None => return self.the_str.get(self.index..=(self.the_str.len()-1)),
        }
    }

    pub fn consume_line_ahead(&mut self) -> Option<&'a str> {
        match self.find_next("\n") {
            Some(newline_index) => {
                let ret = self.the_str.get(self.index..=(self.index+newline_index));
                self.update_index_to(self.index+newline_index+1);
                return ret
            },
            None if self.peek().is_some() => return {
                let ret = self.the_str.get(self.index..=(self.the_str.len()-1));
                self.update_index_to(self.the_str.len());
                return ret
            },
            _ => None,
        }
    }
}