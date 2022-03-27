use std::{str::Chars, iter::FromIterator};

struct AccData {
    space_mode: bool,
    space: Vec<char>,
    word: Vec<char>,
}

impl AccData {
    fn empty() -> AccData {
        AccData {
            space_mode: true,
            space: Vec::new(),
            word: Vec::new(),
        }
    }

    fn push<'a>(&mut self, char: char) -> Option<(String, String)> {
        let char_is_whitespace = char.is_whitespace();
        let space_mode = self.space_mode;

        match (space_mode, char_is_whitespace) {
            (true, true) => {
                self.space.push(char);
                None
            },
            (true, false) => {
                self.space_mode = false;
                self.word.push(char);
                None
            },
            (false, true) => {
                self.space_mode = true;
                let current = self.get_current();
                self.space.push(char);
                Some(current)
            },
            (false, false) => {
                self.word.push(char);
                None
            },
        }
    }

    fn get_current<'a>(&mut self) -> (String, String) {
        let space = String::from_iter(std::mem::take(&mut self.space).into_iter());
        let word = String::from_iter(std::mem::take(&mut self.word).into_iter());

        (space, word)
    }
}

pub struct WordIter<'a> {
    is_end: bool,
    source: Chars<'a>,
    data: AccData,
}

impl<'a> WordIter<'a> {
    fn new(text: &'a str) -> WordIter<'a> {
        WordIter {
            is_end: false,
            source: text.chars(),
            data: AccData::empty(),
        }
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }

        loop {
            let char = self.source.next();

            if let Some(char) = char {
                if let Some(ready) = self.data.push(char) {
                    return Some(ready);
                }
            } else {
                self.is_end = true;
                return Some(self.data.get_current());
            }
        }
    }
}


#[test]
fn test_basic() {
    let content = "aaa bbb ccc";

    let words = WordIter::new(content)
        .map(|(_, word)| String::from(word))
        .collect::<Vec<String>>();
    
    assert_eq!(words, vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ]);
}

#[test]
fn test_basic2() {
    let content = "  aaa bbb ccc";

    let words = WordIter::new(content)
        .map(|(_, word)| String::from(word))
        .collect::<Vec<String>>();
    
    assert_eq!(words, vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
    ]);
}

#[test]
fn test_basic3() {
    let content = "  aaa bbb ccc   ";

    let words = WordIter::new(content)
        .map(|(_, word)| String::from(word))
        .collect::<Vec<String>>();
    
    assert_eq!(words, vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
        String::from("")
    ]);
}