use std::rc::Rc;

use vertigo::{Css, VDomElement, css, html, bind, Resource, VDomComponent};

use crate::app::App;
use crate::components::list_items_from_dir;
use crate::data::{Data, ContentType};
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

fn render_content_text(state: &App, content: Rc<String>) -> Vec<VDomElement> {
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

                let on_click = bind(state)
                    .and(&url)
                    .call(|state, url| {
                        state.data.tab.open_links.tabs_toogle(url.clone());
                    });

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

fn render_dir(data: &Data, dir: &Vec<String>) -> VDomElement {
    // let mut result = Vec::new();

    // for item in list.get_list() {
    //     result.push(html! {
    //         <div>
    //             { item.name }
    //         </div>
    //     })
    // }

    let result = list_items_from_dir(data, dir, false);

    html! {
        <div css={css_content_dir()}>
            { ..result }
        </div>
    }
}

pub fn render_content(state: &App) -> VDomComponent {
    VDomComponent::from_ref(state, |state| {
        let current_content = state.data.tab.current_content.get();

        match current_content {
            Resource::Loading => {
                html! {
                    <div></div>
                }
            },
            Resource::Error(message) => {
                let message = format!("Error: {message}");
                html! {
                    <div>
                        { message }
                    </div>
                }
            },
            Resource::Ready(content) => {
                match content {
                    ContentType::Text { content } => {
                        let out: Vec<VDomElement> = render_content_text(state, content);

                        html! {
                            <div css={css_content_file()}>
                                { ..out }
                            </div>
                        }
                    },
                    ContentType::Image { url } => {
                        let url = url.as_str();
                        html! {
                            <div css={css_content_file()}>
                                <img css={css_content_file_image()} src={url} />
                            </div>
                        }
                    },
                    ContentType::Dir { list } => {
                        render_dir(&state.data, list.dir_path().as_ref())
                    },
                }
            },
        }
    })
}

