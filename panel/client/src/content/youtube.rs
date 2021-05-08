
use url::Url;
use qstring::QString;

/*
    strona która generuje podglądy dla youtube
    http://www.get-youtube-thumbnail.com/

    przykładowy wejściowy link
    https://www.youtube.com/watch?v=AP0TvEIDuqY

    miniaturki
    http://i3.ytimg.com/vi/AP0TvEIDuqY/maxresdefault.jpg
    http://i3.ytimg.com/vi/AP0TvEIDuqY/hqdefault.jpg
*/

fn get_thumbnail_youtube(url: &str) -> Option<String> {
    let result = Url::parse(url);

    let url = match result {
        Ok(result) => result,
        Err(_) => {
            return None;
        }
    };

    if url.domain() != Some("www.youtube.com") {
        return None;
    }

    if url.path() != "/watch" {
        return None;
    }

    let query = match url.query() {
        Some(query) => query,
        None => {
            return None;
        }
    };

    let qs = QString::from(query);

    let id = qs.get("v");

    let id = match id {
        Some(id) => id,
        None => {
            return None;
        }
    };

    return Some(format!("http://i3.ytimg.com/vi/{}/hqdefault.jpg", id));
}

pub fn get_thumbnail(url: &str) -> Option<String> {
    get_thumbnail_youtube(url)
}

#[test]
fn basic() {
    let result = Url::parse("https://www.youtube.com/watch?v=AP0TvEIDuqY").unwrap();

    assert!(result.scheme() == "https");
    assert!(result.path() == "/watch");
    assert!(result.domain() == Some("www.youtube.com"));
    assert!(result.query() == Some("v=AP0TvEIDuqY"));

    let qs = QString::from("v=AP0TvEIDuqY");
    let val = qs.get("foo");
    assert_eq!(val, None);
    let val = qs.get("v");
    assert_eq!(val, Some("AP0TvEIDuqY"));


    let qs = QString::from("?v=AP0TvEIDuqY");
    let val = qs.get("foo");
    assert_eq!(val, None);
    let val = qs.get("v");
    assert_eq!(val, Some("AP0TvEIDuqY"));
}

#[test]
fn test_get_thumbnail() {
    assert_eq!(get_thumbnail("https://www.youtube.com/watch?v=AP0TvEIDuqY"), Some("http://i3.ytimg.com/vi/AP0TvEIDuqY/hqdefault.jpg".into()));
}
