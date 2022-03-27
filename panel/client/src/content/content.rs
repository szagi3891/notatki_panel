use crate::content::words::{WordIter, GroupTextIter};

#[derive(Debug, PartialEq)]
pub enum ParseTextItem {
    Text {
        text: String,
    },
    Link {
        url: String,
    }
}

impl ParseTextItem {
    #[cfg(test)]
    pub fn text_str(text: &str) -> ParseTextItem {
        ParseTextItem::Text {
            text: String::from(text)
        }
    }

    #[cfg(test)]
    pub fn link_str(url: &str) -> ParseTextItem {
        ParseTextItem::Link {
            url: String::from(url)
        }
    }
}

pub fn parse_text<'a>(text: &'a str) -> impl Iterator<Item=ParseTextItem> + 'a {
    let iter = WordIter::new(text)
        .map(|item: Result<String, String>| -> Result<String, String> {

            match item {
                Ok(item) => {
                    if item.starts_with("http://") || item.starts_with("https://") {
                        Ok(item)
                    } else {
                        Err(item)
                    }
                },
                Err(err) => {
                    Err(err)
                }
            }
        });

    let iter = GroupTextIter::new(iter)
        .map(|item| -> ParseTextItem {
            match item {
                Ok(url) => ParseTextItem::Link { url },
                Err(text) => ParseTextItem::Text { text }
            }
        });

    iter
}

#[test]
fn basic_parse() {
    let text = "kolekcja ikon do wykorzystania https://css.gg/play-button";
    let out = parse_text(&text).collect::<Vec<_>>();

    assert_eq!(out[0], ParseTextItem::text_str("kolekcja ikon do wykorzystania "));
    assert_eq!(out[1], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out.len(), 2);
}

#[test]
fn basic_parse2() {
    let text = "kolekcja ikon do wykorzystania https://css.gg/play-button dsadasdasdsada";
    let out = parse_text(&text).collect::<Vec<_>>();

    assert_eq!(out[0], ParseTextItem::text_str("kolekcja ikon do wykorzystania "));
    assert_eq!(out[1], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[2], ParseTextItem::text_str(" dsadasdasdsada"));
    assert_eq!(out.len(), 3);
}

#[test]
fn basic_parse3() {
    let text = "https://css.gg/play-button dsadasdasdsada";
    let out = parse_text(&text).collect::<Vec<_>>();

    assert_eq!(out[0], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[1], ParseTextItem::text_str(" dsadasdasdsada"));
    assert_eq!(out.len(), 2);
}

#[test]
fn basic_parse4() {
    let text = "https://css.gg/play-button";
    let out = parse_text(&text).collect::<Vec<_>>();

    assert_eq!(out[0], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out.len(), 1);
}

#[test]
fn basic_parse5() {
    let text = "dsadsa https://css.gg/play-button dsadsa dsadsadsa dsadsadas https://css.gg/play-button dsadasdasdsadas dsadasdas";
    let out = parse_text(&text).collect::<Vec<_>>();

    assert_eq!(out[0], ParseTextItem::text_str("dsadsa "));
    assert_eq!(out[1], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[2], ParseTextItem::text_str(" dsadsa dsadsadsa dsadsadas "));
    assert_eq!(out[3], ParseTextItem::link_str("https://css.gg/play-button"));
    assert_eq!(out[4], ParseTextItem::text_str(" dsadasdasdsadas dsadasdas"));
    assert_eq!(out.len(), 5);
}