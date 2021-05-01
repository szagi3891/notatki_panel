use vertigo::{Css, VDomElement, computed::Computed};
use vertigo_html::{css, html};

use crate::app::state::State;

fn css_content() -> Css {
    css!("
        white-space: pre-line;
    ")
}

pub fn render_content(state: &Computed<State>) -> VDomElement {

    let state = state.get_value();

    let current_content = state.current_content.get_value();

    let content = current_content.to_string();

    if let Some(content) = content {
        return html!("
            <div css={css_content()}>{ content }</div>
        ");
    }

    return html!("
        <div></div>
    ");
}

