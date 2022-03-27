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

pub struct WordPairIter<'a> {
    is_end: bool,
    source: Chars<'a>,
    data: AccData,
}

impl<'a> WordPairIter<'a> {
    fn new(text: &'a str) -> WordPairIter<'a> {
        WordPairIter {
            is_end: false,
            source: text.chars(),
            data: AccData::empty(),
        }
    }
}

impl<'a> Iterator for WordPairIter<'a> {
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

    let words = WordPairIter::new(content)
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

    let words = WordPairIter::new(content)
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

    let words = WordPairIter::new(content)
        .map(|(_, word)| String::from(word))
        .collect::<Vec<String>>();
    
    assert_eq!(words, vec![
        String::from("aaa"),
        String::from("bbb"),
        String::from("ccc"),
        String::from("")
    ]);
}

/*
    Zamienia iterator (String, String) na itarator Result<String, String>
    Ok - oznacza wyraz który będzie dalej przetwarzany
    Err - oznacza to co zostało odrzucone z przetwarzania
*/

pub struct WordIter<'a> {
    is_end: bool,
    source: WordPairIter<'a>,
    next_item: Option<Result<String, String>>,
}

impl<'a> WordIter<'a> {
    pub fn new(text: &'a str) -> WordIter<'a> {
        WordIter {
            is_end: false,
            source: WordPairIter::new(text),
            next_item: None,
        }
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = Result<String, String>;                 //Ok - word, Err - white string

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end {
            return None;
        }

        if let Some(item) = std::mem::take(&mut self.next_item) {
            return Some(item);
        }

        if let Some((white, text)) = self.source.next() {
            self.next_item = Some(Ok(text));
            return Some(Err(white));
        }

        self.is_end = true;
        None
    }
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
