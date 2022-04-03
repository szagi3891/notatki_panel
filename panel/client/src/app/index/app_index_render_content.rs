use std::rc::Rc;

use vertigo::{Css, VDomElement};
use vertigo::{css, html};

use super::AppIndex;
use crate::data::{CurrentContent, DirList};
use crate::{
    content::{
        parse_text,
        ParseTextItem,
        get_thumbnail,
    }
};

fn css_content() -> Css {
    css!("
        width: 100%;
        font-family: monospace;
        white-space: pre-wrap;
    ")
}

fn link_css() -> Css {
    css!("
        color: blue;
        text-decoration: none;

        :hover {
            text-decoration: underline;
        }
    ")
}

fn youtube_css() -> Css {
    css!("
        display: block;
    ")
}

fn open_css() -> Css {
    css!("
        cursor: pointer;
    ")
}

fn render_content_text(state: &AppIndex, content: &Rc<String>) -> Vec<VDomElement> {
    let chunks = parse_text(content.as_str());

    let mut out: Vec<VDomElement> = Vec::new();

    for item in chunks {
        match item {
            ParseTextItem::Link { url } => {
                let url = url.to_string();

                let has_open = state.data.tab.open_links.tabs_has(&url);
                let link_label = match has_open {
                    true => "(zamknij)",
                    false => "(otwórz)"
                };

                let on_click = {
                    let state = state.clone();
                    let url = url.clone();
                    
                    move || {
                        state.data.tab.open_links.tabs_toogle(url.clone());
                    }
                };

                let img = if let Some(thumb) = get_thumbnail(url.as_str()) {
                    html! {
                        <img css={youtube_css()} src={thumb} />
                    }
                } else {
                    html! {
                        <span></span>
                    }
                };

                out.push(html!{
                    <span>
                        <a href={url.clone()} target="_blank" css={link_css()}>
                            <span>{url}</span>
                            { img }
                        </a>
                        " "
                        <span on_click={on_click} css={open_css()}>
                            { link_label }
                        </span>
                    </span>
                });
            },
            ParseTextItem::Text { text } => {
                let text = text.to_string();

                out.push(html!{
                    <span>{ text }</span>
                });
            }
        }
    }

    out
}

fn render_dir(list: &DirList) -> VDomElement {
    let mut result = Vec::new();

    for item in list.get_list() {
        result.push(html! {
            <div>
                { item.name }
            </div>
        })
    }

    html! {
        <div css={css_content()}>
            { ..result }
        </div>
    }
}

pub fn render_content(state: &AppIndex) -> VDomElement {
    let current_content = state.app.data.tab.current_content.get_value();

    match current_content.as_ref() {
        CurrentContent::File { file_name: _, file_hash: _, content } => {
            let out: Vec<VDomElement> = render_content_text(state, content);

            html! {
                <div css={css_content()}>
                    { ..out }
                </div>
            }
        }
        CurrentContent::Dir { dir: _, dir_hash: _, list } => {
            render_dir(list)
        },
        CurrentContent::None => {
            html!{
                <div></div>
            }
        }
    }
}

