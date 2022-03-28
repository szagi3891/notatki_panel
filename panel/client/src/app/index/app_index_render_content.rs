use vertigo::{Css, VDomElement};
use vertigo::{css, html};

use super::AppIndex;
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

pub fn render_content(state: &AppIndex) -> VDomElement {
    let current_content = state.app.data.tab.current_content.get_value();

    let content = current_content.to_string();

    if let Some(content) = content {
        let chunks = parse_text(content.as_str());

        let mut out: Vec<VDomElement> = Vec::new();

        for item in chunks {
            match item {
                ParseTextItem::Link { url } => {
                    let url = url.to_string();
                    let thumb = get_thumbnail(url.as_str());

                    let has_open = state.data.tab.open_links.tabs_has(&url);

                    let open_link = if has_open {
                        let on_click = {
                            let state = state.clone();
                            let url = url.clone();
                            
                            move || {
                                state.data.tab.open_links.tabs_remove(url.clone());
                            }
                        };

                        html! {
                            <span on_click={on_click} css={open_css()}>"(zamknij)"</span>
                        }
                    } else {
                        let on_click = {
                            let state = state.clone();
                            let url = url.clone();
                            
                            move || {
                                state.data.tab.open_links.tabs_add(url.clone());
                            }
                        };

                        html! {
                            <span on_click={on_click} css={open_css()}>"(otwórz)"</span>
                        }
                    };

                    let img = if let Some(thumb) = thumb {
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
                            { open_link }
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
        return html! {
            <div css={css_content()}>
                { ..out }
            </div>
        };
    }

    return html!{
        <div></div>
    };
}

