use std::rc::Rc;

use vertigo::{Css, css, bind, Resource, dom, Computed, DomNode, Value};

use crate::app::App;
use crate::components::list_items_from_dir;
use crate::data::{ContentType, ListItem};
use crate::{
    content::{
        parse_text,
        ParseTextItem,
        get_thumbnail,
    }
};

fn css_content_file() -> Css {
    css!("
        width: 100%;
        font-family: monospace;
        white-space: pre-wrap;
    ")
}

fn css_content_dir() -> Css {
    css!("
        width: 100%;
    ")
}

fn css_content_file_image() -> Css {
    css!("
        width: 100%;
        display: block;
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

fn render_content_chunk(state: &App, item: &ParseTextItem) -> DomNode {
    match item {
        ParseTextItem::Link { url, has_open } => {
            let url = url.to_string();

            let link_label = match has_open {
                true => "(zamknij)",
                false => "(otwórz)"
            };

            let on_click = bind!(state, url, || {
                state.data.tab.open_links.tabs_toogle(url.clone());
            });

            let img = if let Some(thumb) = get_thumbnail(url.as_str()) {
                dom! {
                    <img css={youtube_css()} src={thumb} />
                }
            } else {
                dom! {
                    <span></span>
                }
            };

            dom!{
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
            }
        },
        ParseTextItem::Text { text } => {
            let text = text.to_string();

            dom!{
                <span>{ text }</span>
            }
        }
    }
}

fn render_content_text(state: &App, content: Rc<String>) -> DomNode {
    let chunks = Computed::from({
        let state = state.clone();
        move |context| {
            parse_text(content.as_str(), |url| {
                state.data.tab.open_links.tabs_has(context, url)
            })
        }
    });

    chunks.render_list(
        |link| link.clone(),
        {
            let state = state.clone();
            move |link| {
                render_content_chunk(&state, link)
            }
        }
    )
}

fn render_dir(state: &App, dir: Computed<ListItem>) -> DomNode {
    let result = list_items_from_dir(&state.data, &dir, false);

    dom! {
        <div css={css_content_dir()}>
            { result }
        </div>
    }
}

pub fn render_content(state: &App) -> DomNode {
    let current_content = Computed::from({
        let select_content = state.data.tab.select_content.clone();
        move |context| {
            if let Some(item) = select_content.get(context) {
                item.get_content_type(context)
            } else {
                Resource::Loading
            }
        }
    });

    current_content.render_value({
        let state = state.clone();
        move |current_content| {

            match current_content {
                Resource::Loading => {
                    dom! {
                        <div></div>
                    }
                },
                Resource::Error(message) => {
                    let message = format!("Error: {message}");
                    dom! {
                        <div>
                            { message }
                        </div>
                    }
                },
                Resource::Ready(content) => {
                    match content {
                        ContentType::Text { content } => {
                            let out = render_content_text(&state, content);

                            dom! {
                                <div css={css_content_file()}>
                                    { out }
                                </div>
                            }
                        },
                        ContentType::Image { url } => {
                            let url = url.as_ref().clone();
                            dom! {
                                <div css={css_content_file()}>
                                    <img css={css_content_file_image()} src={url} />
                                </div>
                            }
                        },
                        ContentType::Dir { item } => {
                            let item = Value::new(item).to_computed();
                            render_dir(&state, item)
                        },
                    }
                },
            }

        }
    })
}

