use std::{str::Chars, iter::FromIterator};

struct AccData {
    space_mode: bool,
    chars: Vec<char>,
}

impl AccData {
    fn empty() -> AccData {
        AccData {
            space_mode: true,
            chars: Vec::new(),
        }
    }

    fn push<'a>(&mut self, char: char) -> Option<Result<String, String>> {
        let char_is_whitespace = char.is_whitespace();
        let space_mode = self.space_mode;

        if space_mode == char_is_whitespace {
            self.chars.push(char);
            None
        } else {
            let current = self.get_current();
            self.space_mode = char_is_whitespace;
            self.chars.push(char);
            current
        }
    }

    fn get_current<'a>(&mut self) -> Option<Result<String, String>> {
        if self.chars.len() == 0 {
            return None;
        }

        let text = String::from_iter(std::mem::take(&mut self.chars).into_iter());
        if self.space_mode {
            Some(Err(text))
        } else {
            Some(Ok(text))
        }
    }
}

pub struct WordIter<'a> {
    is_end: bool,
    source: Chars<'a>,
    data: AccData,
}

impl<'a> WordIter<'a> {
    pub fn new(text: &'a str) -> WordIter<'a> {
        WordIter {
            is_end: false,
            source: text.chars(),
            data: AccData::empty(),
        }
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = Result<String, String>;

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
                return self.data.get_current();
            }
        }
    }
}

#[test]
fn test_basic() {
    let content = "aaa bbb ccc";

    let words = WordIter::new(content)
        .collect::<Vec<Result<String, String>>>();
    
    assert_eq!(words, vec![
        Ok(String::from("aaa")),
        Err(String::from(" ")),
        Ok(String::from("bbb")),
        Err(String::from(" ")),
        Ok(String::from("ccc")),
    ]);
}

#[test]
fn test_basic2() {
    let content = "  aaa bbb ccc";

    let words = WordIter::new(content)
        .collect::<Vec<Result<String, String>>>();
    
    assert_eq!(words, vec![
        Err(String::from("  ")),
        Ok(String::from("aaa")),
        Err(String::from(" ")),
        Ok(String::from("bbb")),
        Err(String::from(" ")),
        Ok(String::from("ccc")),
    ]);
}

#[test]
fn test_basic3() {
    let content = "  aaa bbb ccc   ";

    let words = WordIter::new(content)
        .collect::<Vec<Result<String, String>>>();
    
    assert_eq!(words, vec![
        Err(String::from("  ")),
        Ok(String::from("aaa")),
        Err(String::from(" ")),
        Ok(String::from("bbb")),
        Err(String::from(" ")),
        Ok(String::from("ccc")),
        Err(String::from("   ")),
    ]);
}

/*
    Grupuje
    Result<T, String> - Stringi są grupowane razem jeśli występują obok siebie
*/

pub struct GroupTextIter<T, I: Iterator<Item=Result<T, String>>> {
    source: I,
    is_end: bool,
    text: Vec<String>,
    next_item: Option<Result<T, String>>,
}

impl<T, I: Iterator<Item=Result<T, String>>> GroupTextIter<T, I> {
    pub fn new(source: I) -> GroupTextIter<T, I> {
        GroupTextIter {
            source,
            is_end: false,
            text: Vec::new(),
            next_item: None,
        }
    }
}

impl<T, I: Iterator<Item=Result<T, String>>> Iterator for GroupTextIter<T, I> {
    type Item = Result<T, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }

        if let Some(item) = std::mem::take(&mut self.next_item) {
            return Some(item);
        }

        loop {
            if let Some(item) = self.source.next() {
                match item {
                    Ok(item) => {
                        if self.text.len() > 0 {
                            self.next_item = Some(Ok(item));
                            return Some(Err(
                                std::mem::take(&mut self.text).join("")
                            ));
                        }

                        return Some(Ok(item));
                    },
                    Err(text) => {
                        self.text.push(text);
                    },
                }
            } else {
                self.is_end = true;

                if self.text.len() > 0 {
                    return Some(Err(
                        std::mem::take(&mut self.text).join("")
                    ));
                }

                return None;
            }
        }
    }
}
