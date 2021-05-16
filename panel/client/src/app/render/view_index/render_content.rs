use vertigo::{Css, VDomElement, computed::Computed};
use vertigo_html::{css, html};

use crate::{
    app::state::StateViewIndex,
    content::{
        parse_text,
        ParseTextItem,
        get_thumbnail,
    }
};

fn css_content() -> Css {
    css!("
        white-space: pre-line;
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

pub fn render_content(state: &Computed<StateViewIndex>) -> VDomElement {

    let state = state.get_value();

    let current_content = state.current_content.get_value();

    let content = current_content.to_string();

    if let Some(content) = content {
        let chunks = parse_text(content.as_str());

        let mut out: Vec<VDomElement> = Vec::new();

        for item in chunks {
            match item {
                ParseTextItem::Link { url } => {
                    let url = url.to_string();
                    let thumb = get_thumbnail(url.as_str());

                    if let Some(thumb) = thumb {
                        out.push(html!{r#"
                            <a href={url.clone()} target="_blank" css={link_css()}>
                                <span>{url}</span>
                                <img src={thumb} />
                            </a>
                        "#});
                    } else {
                        out.push(html!{r#"
                            <a href={url.clone()} target="_blank" css={link_css()}>
                                {url}
                            </a>
                        "#});
                    }
                },
                ParseTextItem::Text { text } => {
                    let text = text.to_string();

                    out.push(html!{"
                        <span>{ text }</span>
                    "});
                }
            }
        }
        return html!("
            <div css={css_content()}>
                { ..out }
            </div>
        ");
    }

    return html!("
        <div></div>
    ");
}
