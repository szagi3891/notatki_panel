use std::rc::Rc;

use vertigo::{Css, css, html, bind, Resource, VDomComponent, Context, dom, DomElement, Computed, render_list, DomComment};

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

fn render_content_chunk(state: &App, item: &ParseTextItem) -> DomElement {
    match item {
        ParseTextItem::Link { url, has_open } => {
            let url = url.to_string();

            let link_label = match has_open {
                true => "(zamknij)",
                false => "(otwórz)"
            };

            let on_click = bind(state)
                .and(&url)
                .call(|context, state, url| {
                    state.data.tab.open_links.tabs_toogle(context, url.clone());
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

fn render_content_text(state: &App, content: Rc<String>) -> DomComment {
    let chunks = Computed::from({
        let state = state.clone();
        move |context| {
            parse_text(content.as_str(), |url| {
                state.data.tab.open_links.tabs_has(context, url)
            })
        }
    });

    render_list(
        chunks,
        |link| link.clone(),
        {
            let state = state.clone();
            move |link| {
                render_content_chunk(&state, link)
            }
        }
    )
}

fn render_dir(context: &Context, data: &Data, dir: &Vec<String>) -> DomElement {
    let result = list_items_from_dir(context, data, dir, false);

    let out = dom! {
        <div css={css_content_dir()} />
    };

    for child in result.into_iter() {
        out.add_child(child);
    }

    out
}

pub fn render_content(state: &App) -> VDomComponent {
    VDomComponent::from_ref(state, |context, state| {
        let current_content = state.data.tab.current_content.get(context);

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
                        let out = render_content_text(state, content);
                        let out = VDomComponent::dom(dom! {         //TODO do usunięcia ten nadmiarowy div
                            <div>
                                { out }
                            </div>
                        });

                        html! {
                            <div css={css_content_file()}>
                                { out }
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
                        let out = VDomComponent::dom(render_dir(context, &state.data, list.dir_path().as_ref()));

                                            //TODO - usunąć tego nadmiarowego diva
                        html! {
                            <div>
                                { out }
                            </div>
                        }
                    },
                }
            },
        }
    })
}

