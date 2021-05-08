use std::fmt;

#[derive(PartialEq)]
pub struct Text {
    list: Vec<char>
}

impl Text {
    pub fn from_str(text: &str) -> Text {
        let list = text.chars().collect::<Vec<char>>();
        Text {
            list
        }
    }

    pub fn from_slice(list: &[char]) -> Text {
        Text {
            list: Vec::from(list)
        }
    }

    pub fn empty() -> Text {
        Text {
            list: Vec::new()
        }
    }

    pub fn as_slice(&self) -> &[char] {
        self.list.as_slice()
    }

    pub fn sum(text1: Text, text2: Text) -> Text {
        let mut list = text1.list.clone();
        list.extend(text2.list.as_slice());

        Text {
            list
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

impl fmt::Debug for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: String = self.list.iter().collect();
        f.write_str(data.as_str())

        // f.debug_struct("Text")
        //  .field("list", &self.x)
        //  .finish()
    }
}

fn parse_prefix(prefix: &str, text: &Text) -> Option<(Text, Text)> {
    let text = text.as_slice();
    if text.len() < prefix.len() {
        return None;
    }

    let begin = &text[0..prefix.len()];
    let rest = &text[prefix.len()..];

    if Text::from_str(prefix) == Text::from_slice(begin) {
        return Some((Text::from_slice(begin), Text::from_slice(rest)));
    }

    None
}

fn parse_http(text: &Text) -> Option<(Text, Text)> {
    parse_prefix("http://", text)
}

fn parse_https(text: &Text) -> Option<(Text, Text)> {
    parse_prefix("https://", text)
}

fn parse_link_prefix(text: &Text) -> Option<(Text, Text)> {
    if let Some(data) = parse_http(text) {
        return Some(data);
    }

    if let Some(data) = parse_https(text) {
        return Some(data);
    }
    
    None
}

fn parse_until_whitespace(text: &Text) -> Option<(Text, Text)> {
    let text = text.as_slice();

    for (index, char) in text.into_iter().enumerate() {
        if char.is_whitespace() {
            if index == 0 {
                return None;
            }

            let begin = &text[0..index];
            let rest = &text[index..];

            return Some((Text::from_slice(begin), Text::from_slice(rest)));
        }
    }

    Some((Text::from_slice(text), Text::empty()))
}

//char_indices()

fn parse_trim_whitespace(text: &Text) -> Text {
    let text = text.as_slice();

    for (index, char) in text.into_iter().enumerate() {
        if !char.is_whitespace() {
            let rest = &text[index..];
            return Text::from_slice(rest);
        }
    }

    return Text::empty();
}

fn parse_url(text: &Text) -> Option<(Text, Text)> {
    if let Some((scheme, rest)) = parse_link_prefix(text) {
        if let Some((link_body, rest)) = parse_until_whitespace(&rest) {
            //return Some((scheme ++ link_body, parse_trim_whitespace(rest)));

            return Some((
                Text::sum(scheme, link_body),
                rest
                //parse_trim_whitespace(&rest)
            ));
        }
    }

    None
}

fn get_first_char(text: &Text) -> Option<(char, Text)> {
    let text = text.as_slice();
    let first = text.first();

    if let Some(first) = first {
        return Some((
            first.clone(),
            Text::from_slice(&text[1..])
        ));
    }

    None
}

#[derive(Debug, PartialEq)]
pub enum ParseTextItem {
    Text {
        text: Text,
    },
    Link {
        url: Text,
    }
}

impl ParseTextItem {
    pub fn text(text: Text) -> ParseTextItem {
        ParseTextItem::Text {
            text
        }
    }

    pub fn text_str(text: &str) -> ParseTextItem {
        ParseTextItem::Text {
            text: Text::from_str(text)
        }
    }

    pub fn link(url: Text) -> ParseTextItem {
        ParseTextItem::Link {
            url
        }
    }

    pub fn link_str(url: &str) -> ParseTextItem {
        ParseTextItem::Link {
            url: Text::from_str(url)
        }
    }
}

struct ParserText {
    text: Text,
    chars: Vec<char>,
    out: Vec<ParseTextItem>,
}

impl ParserText {
    pub fn new(text: &str) -> ParserText {
        ParserText {
            text: Text::from_str(text),
            chars: Vec::new(),
            out: Vec::new(),
        }
    }

    fn push_word(&mut self) {
        if self.chars.len() > 0 {
            self.out.push(ParseTextItem::Text {
                text: Text::from_slice(self.chars.as_slice())
            });

            self.chars = Vec::new();
        }
    }

    pub fn parse(mut self) -> Vec<ParseTextItem> {
        loop {
            if self.text.len() == 0 {
                self.push_word();
                return self.out;
            }
    
            if let Some((url, rest)) = parse_url(&self.text) {
                self.push_word();
                self.out.push(ParseTextItem::Link{
                    url 
                });
    
                self.text = rest;
                continue;
            }
    
            if let Some((char, rest)) = get_first_char(&self.text) {
                self.chars.push(char);
    
                self.text = rest;
                continue;
            }
        }
    }
}

pub fn parse_text(text: &str) -> Vec<ParseTextItem> {
    let parser = ParserText::new(text);
    parser.parse()
}

#[test]
fn text_http() {
    let text_http = Text::from_str("http://dsadsa");
    let text_https = Text::from_str("https://dsadsa");
    let text_other = Text::from_str("fdsfdsfdsfdsfs");

    assert_eq!(parse_http(&text_http), Some((Text::from_str("http://"), Text::from_str("dsadsa"))));
    assert_eq!(parse_http(&text_https), None);
    assert_eq!(parse_http(&text_other), None);

    assert_eq!(parse_https(&text_http), None);
    assert_eq!(parse_https(&text_https), Some((Text::from_str("https://"), Text::from_str("dsadsa"))));
    assert_eq!(parse_https(&text_other), None);
}

#[test]
fn test_parse_until_whitespace() {
    let text1 = Text::from_str("cos bla");
    let text2 = Text::from_str(" cos bla");
    let text3 = Text::from_str("  cos bla");

    assert_eq!(parse_until_whitespace(&text1), Some((Text::from_str("cos"), Text::from_str(" bla"))));
    assert_eq!(parse_until_whitespace(&text2), None);
    assert_eq!(parse_until_whitespace(&text3), None);
}

#[test]
fn test_parse_trim_whitespace() {
    let text1 = Text::from_str("cos bla");
    let text2 = Text::from_str(" cos bla");
    let text3 = Text::from_str("  cos bla");
    let text4 = Text::from_str("");
    let text5 = Text::from_str(" ");
    let text6 = Text::from_str("  ");

    assert_eq!(parse_trim_whitespace(&text1), Text::from_str("cos bla"));
    assert_eq!(parse_trim_whitespace(&text2), Text::from_str("cos bla"));
    assert_eq!(parse_trim_whitespace(&text3), Text::from_str("cos bla"));
    assert_eq!(parse_trim_whitespace(&text4), Text::from_str(""));
    assert_eq!(parse_trim_whitespace(&text5), Text::from_str(""));
    assert_eq!(parse_trim_whitespace(&text6), Text::from_str(""));
}

#[test]
fn test_parse_url() {
    let text1 = Text::from_str("https://css.gg/play-button");
    let text2 = Text::from_str("http://css.gg/play-button");
    let text3 = Text::from_str("https://css.gg/play-button dd");
    let text4 = Text::from_str("http://css.gg/play-button tt");

    assert_eq!(parse_url(&text1), Some((Text::from_str("https://css.gg/play-button"), Text::empty())));
    assert_eq!(parse_url(&text2), Some((Text::from_str("http://css.gg/play-button"), Text::empty())));
    assert_eq!(parse_url(&text3), Some((Text::from_str("https://css.gg/play-button"), Text::from_str(" dd"))));
    assert_eq!(parse_url(&text4), Some((Text::from_str("http://css.gg/play-button"), Text::from_str(" tt"))));
}

#[test]
fn basic_parse() {
    let text = "kolekcja ikon do wykorzystania https://css.gg/play-button";
    let out = parse_text(&text);

    assert_eq!(out.len(), 2);
    assert_eq!(out[0], ParseTextItem::text_str("kolekcja ikon do wykorzystania "));
    assert_eq!(out[1], ParseTextItem::link_str("https://css.gg/play-button"));
}

#[test]
fn basic_parse2() {
    let text = "kolekcja ikon do wykorzystania https://css.gg/play-button dsadasdasdsada";
    let out = parse_text(&text);

    assert_eq!(out.len(), 3);
    assert_eq!(out[0], ParseTextItem::text_str("kolekcja ikon do wykorzystania "));
    assert_eq!(out[1], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[2], ParseTextItem::text_str(" dsadasdasdsada"));
}

#[test]
fn basic_parse3() {
    let text = "https://css.gg/play-button dsadasdasdsada";
    let out = parse_text(&text);

    assert_eq!(out.len(), 2);
    assert_eq!(out[0], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[1], ParseTextItem::text_str(" dsadasdasdsada"));
}

#[test]
fn basic_parse4() {
    let text = "https://css.gg/play-button";
    let out = parse_text(&text);

    assert_eq!(out.len(), 1);
    assert_eq!(out[0], ParseTextItem::link_str("https://css.gg/play-button"));
}

#[test]
fn basic_parse5() {
    let text = "dsadsa https://css.gg/play-button dsadsa dsadsadsa dsadsadas https://css.gg/play-button dsadasdasdsadas dsadasdas";
    let out = parse_text(&text);

    assert_eq!(out.len(), 5);
    assert_eq!(out[0], ParseTextItem::text_str("dsadsa "));
    assert_eq!(out[1], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[2], ParseTextItem::text_str(" dsadsa dsadsadsa dsadsadas "));
    assert_eq!(out[3], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[4], ParseTextItem::text_str(" dsadasdasdsadas dsadasdas"));
}